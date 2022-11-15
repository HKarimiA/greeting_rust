
use actix_cors::Cors;
use actix_web::{http, App, HttpServer};

use futures::future;
use greeting::{get_greeting_message, get_liveness, get_readiness, get_startup, get_version};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server1 = HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost")
            .allowed_origin("http://127.0.0.1")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .service(get_greeting_message)
            .service(get_version)
    })
    .bind("127.0.0.1:8080")?
    .run();

    let server2 = HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost")
            .allowed_origin("http://127.0.0.1")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .service(get_liveness)
            .service(get_readiness)
            .service(get_startup)
    })
    .bind("127.0.0.1:8081")?
    .run();
    future::try_join(server1, server2).await?;
    Ok(())
}
