use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};

mod utils;
mod routes;
mod configs;
mod handlers;
mod schemas;


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = configs::env_config::Config::from_env();
    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(greet)
            .configure(routes::routers::init)
    })
    .bind(format!("{}:{}", config.server_host, config.server_port))?
    .run();

    let server_handle = server.handle();
    tokio::spawn(async move {
        utils::graceful_shutdown::handle_graceful_shutdown(server_handle).await;
    });

    server.await
}