mod conditional;
use conditional::Conditional;

use actix_web::{get, error, App, HttpRequest, HttpServer, Responder, HttpResponse, http::header::ContentType, middleware};

#[async_std::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let mut server = HttpServer::new(|| {
        let nocache = middleware::DefaultHeaders::new()
        .add(("Cache-Control", "no-cache, no-store, must-revalidate"))
        .add(("Pragma", "no-cache"))
        .add(("Expires", 0));

        App::new()
            .wrap(Conditional::new(nocache, cfg!(debug_assertions)))
            .wrap(middleware::Logger::default())
    })

    server.bind(("localhost", 80))?
    server.run().await;
}