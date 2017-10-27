use iron::Iron;
use mount::Mount;
use router::Router;
use staticfile::Static;

use database::Database;

use super::error::BackendError;
use super::handler::EmptyHandler;
use super::handler::PriceHandler;
use super::handler::ProductHandler;


pub fn start_backend(
    database: Database,
    bind_address: &str,
    bind_port: u16,
) -> Result<(), BackendError> {
    let mut router = Router::new();
    router.get("/price", PriceHandler::new(database.clone()), "price");
    router.get("/product", ProductHandler::new(database), "product");
    router.get("/", EmptyHandler::new(), "empty");

    let mut mount = Mount::new();
    mount.mount("/static", Static::new("public/static"));
    mount.mount("/api/v1", router);
    mount.mount("/", Static::new("public/index.html"));

    info!("Starting WEB server {}:{}", bind_address, bind_port);

    Iron::new(mount).http((bind_address, bind_port))?;

    Ok(())
}
