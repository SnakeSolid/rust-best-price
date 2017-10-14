use iron::Iron;
use mount::Mount;
use router::Router;
use staticfile::Static;

use database::Database;

use super::error::BackendError;
use super::handler::EmptyHandler;
use super::handler::PriceHandler;


pub fn start_backend(
    database: Database,
    bind_address: String,
    bind_port: u16,
) -> Result<(), BackendError> {
    let mut router = Router::new();
    router.get("/price", PriceHandler::new(database), "price");
    router.get("/", EmptyHandler::new(), "empty");

    let mut mount = Mount::new();
    mount.mount("/static", Static::new("public/static"));
    mount.mount("/api/v1", router);
    mount.mount("/", Static::new("public/index.html"));

    info!("Starting WEB server {}:{}", bind_address, bind_port);

    Iron::new(mount).http((bind_address.as_str(), bind_port))?;

    Ok(())
}
