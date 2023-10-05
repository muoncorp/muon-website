#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_cors::Cors;
use actix_files as fs;
use actix_session::{CookieSession};
use actix_web::{
    guard, middleware, web, App, HttpResponse, HttpServer, Result,
};
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../frontend-new-new/dist/favicon.ico")?)
}

async fn vue_index() -> Result<fs::NamedFile> {
    println!("Redirected to Vue SPA");
    Ok(fs::NamedFile::open("../frontend-new/dist/index.html")?)
}

#[get("/api/notice")]
async fn notice_list() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../frontend-new/public/data/notice/index.json")?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ContactUsFormData {
    email: String,
    subject: String,
    text: String,
}

#[post("/api/contact/send-message")]
async fn send_message(data: web::Json<ContactUsFormData>) -> Result<HttpResponse> {
    println!("data: {:?}", &data);
    let smtp_address = "smtp.gmail.com";
    let username = "DELETED";
    let password = "DELETED";

    let email = EmailBuilder::new()
        .to("contact@muon.co")
        .from(data.email.as_str())
        .reply_to(data.email.as_str())
        .subject(data.subject.as_str())
        .text(data.text.as_str())
        .build()
        .unwrap()
        .into();

    let credentials = (username, password).into_credentials();

    let mut client = SmtpClient::new_simple(smtp_address)
        .unwrap()
        .credentials(credentials)
        .transport();

    let result = client.send(email);
    println!("result: {:?}", result);

    Ok(HttpResponse::Ok().json(data.0))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new().supports_credentials().finish()
            )
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::Logger::default())
            .service(favicon)
            .service(fs::Files::new("/css", "../frontend-new/dist/css"))
            .service(fs::Files::new("/fonts", "../frontend-new/dist/fonts"))
            .service(fs::Files::new("/img", "../frontend-new/dist/img"))
            .service(fs::Files::new("/js", "../frontend-new/dist/js"))
            .service(fs::Files::new("/data", "../frontend-new/public/data"))
            .service(notice_list)
            .service(send_message)
            .default_service(
                web::resource("")
                    .route(web::get().to(vue_index))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed())
                    )
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
