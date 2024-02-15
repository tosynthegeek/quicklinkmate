mod handler;
mod store;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use handler::{create_short_url, hanlde_shorturl_redirect};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Content {
    message: String,
    number: i32,
}

pub async fn index() -> web::Json<Content> {
    let content = Content {
        message: "Hey Go URL Shortener !".to_owned(),
        number: 46,
    };
    web::Json(content)
}

pub async fn greet() -> impl Responder {
    format!("Hello Content not showing!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    let server = HttpServer::new(move || {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/create-short-url").route(web::post().to(create_short_url)))
            .service(web::resource("/:shortUrl").route(web::post().to(handle_short_url_redirect)))
    });
    store::initialize_store();
    println!("Server created, binding...");
    let server = server.bind(("127.0.0.1", 3030))?;
    println!("Server bound, running...");
    server.run().await
}
