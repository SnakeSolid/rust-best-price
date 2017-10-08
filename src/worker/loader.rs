use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::io::Error as IoError;
use std::time::Duration;

use futures::Future;
use futures::Stream;
use hyper::Body;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper::header::Cookie;
use hyper::Method;
use hyper::Request;
use hyper::Result as HyperResult;
use hyper::Uri;
use hyper_tls::HttpsConnector;
use kuchiki::parse_html;
use native_tls::Error as TlsError;
use tendril::TendrilSink;
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;

use super::Product;
use super::ProductError;


#[derive(Debug)]
pub struct PriceLoader {
    core: Core,
    handle: Handle,
    http_client: Client<HttpConnector, Body>,
    https_client: Client<HttpsConnector<HttpConnector>, Body>,
}


#[derive(Debug)]
pub enum PriceLoaderError {
    IoError { description: String },
    TlsError { description: String },
}


enum UriSchema {
    Http,
    Https,
}


impl Display for PriceLoaderError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &PriceLoaderError::IoError { ref description } => {
                write!(f, "IO error: {}", description)
            }
            &PriceLoaderError::TlsError { ref description } => {
                write!(f, "TLS error: {}", description)
            }
        }
    }
}


impl Error for PriceLoaderError {
    fn description(&self) -> &str {
        match self {
            &PriceLoaderError::IoError { ref description } => &description,
            &PriceLoaderError::TlsError { ref description } => &description,
        }
    }
}


impl From<IoError> for PriceLoaderError {
    fn from(error: IoError) -> PriceLoaderError {
        PriceLoaderError::IoError { description: error.description().into() }
    }
}


impl From<TlsError> for PriceLoaderError {
    fn from(error: TlsError) -> PriceLoaderError {
        PriceLoaderError::TlsError { description: error.description().into() }
    }
}


impl PriceLoader {
    pub fn new() -> Result<PriceLoader, PriceLoaderError> {
        let core = Core::new()?;
        let handle = core.handle();
        let timeout = Some(Duration::from_secs(60));
        let http_client = Client::configure()
            .keep_alive(true)
            .keep_alive_timeout(timeout)
            .build(&handle);
        let https_connector = HttpsConnector::new(1, &handle)?;
        let https_client = Client::configure()
            .connector(https_connector)
            .keep_alive(true)
            .keep_alive_timeout(timeout)
            .build(&handle);

        Ok(PriceLoader {
            core,
            handle,
            http_client,
            https_client,
        })
    }

    pub fn load(
        &mut self,
        uri: &String,
        cookies: &Option<HashMap<String, String>>,
        name_selector: &String,
        price_selector: &String,
        price_factor: f64,
        price_index: usize,
    ) -> Result<Product, ProductError> {
        let uri: Uri = uri.parse()?;
        let schema = match uri.scheme() {
            Some("http") => UriSchema::Http,
            Some("https") => UriSchema::Https,
            _ => return Err(ProductError::invalid_schema()),
        };
        let mut request = Request::new(Method::Get, uri);

        if let &Some(ref cookies) = cookies {
            let mut cookie = Cookie::new();

            for (key, value) in cookies {
                cookie.append(key.clone(), value.clone());
            }

            request.headers_mut().set(cookie);
        }

        let request = match schema {
            UriSchema::Http => self.http_client.request(request),
            UriSchema::Https => self.https_client.request(request),
        };

        let future_content = request.and_then(|res| {
            res.body().fold(
                Vec::new(),
                |mut acc, chunk| -> HyperResult<_> {
                    acc.extend(chunk);

                    Ok(acc)
                },
            )
        });
        let content = self.core.run(future_content)?;
        let document = parse_html().from_utf8().one(content.as_slice());
        let mut product_name: Option<String> = None;
        let mut product_price: Option<String> = None;

        for css_match in document.select(name_selector).map_err(
            ProductError::name_not_exists,
        )?
        {
            let children = css_match.as_node().children();

            for child in children {
                if let Some(text) = child.as_text() {
                    let text: String = text.borrow().trim().into();

                    if !text.is_empty() {
                        product_name = Some(text);

                        break;
                    }
                }
            }
        }

        for css_match in document
            .select(price_selector)
            .map_err(ProductError::price_not_exists)?
            .enumerate()
            .filter(|&(index, _)| index == price_index)
            .map(|(_, value)| value)
        {
            let children = css_match.as_node().children();

            for child in children {
                if let Some(text) = child.as_text() {
                    let text = text.borrow().chars().filter(|c| c.is_digit(10)).collect();

                    product_price = Some(text);
                }
            }
        }

        let product_name = product_name.ok_or_else(ProductError::name_not_found)?;
        let product_price: u64 = product_price
            .ok_or_else(ProductError::price_not_found)?
            .parse()?;

        Ok(Product::new(
            product_name,
            price_factor * product_price as f64,
        ))
    }
}
