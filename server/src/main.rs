#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_cors::Cors;
use actix_files as fs;
use actix_session::{CookieSession};
use actix_web::http::header;
use actix_web::{
    guard, middleware, web, App, HttpResponse, HttpRequest, HttpServer, Result,
};

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../frontend/dist/favicon.ico")?)
}

async fn vue_index() -> Result<fs::NamedFile> {
    println!("Redirected to Vue SPA");
    Ok(fs::NamedFile::open("../frontend/dist/index.html")?)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::Logger::default())
            .service(favicon)
            .service(fs::Files::new("/css", "../frontend/dist/css"))
            .service(fs::Files::new("/img", "../frontend/dist/img"))
            .service(fs::Files::new("/js", "../frontend/dist/js"))
            .default_service(
                web::resource("")
                    .route(web::get().to(vue_index))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed())
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
