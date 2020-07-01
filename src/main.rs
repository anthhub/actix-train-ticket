#[macro_use]
extern crate actix_web;

use actix_cors::Cors;
use actix_files as fs;
use actix_session::Session;
use std::{env, io};

use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};

mod city_list;
use city_list::get_city_list;

mod search_city_list;
use search_city_list::get_search_city_list;

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
#[get("/welcome")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8000")
                    .allowed_origin("http://10.180.8.10:8000")
                    .allowed_methods(vec!["GET", "POST"])
                    // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    // .allowed_header(header::CONTENT_TYPE)
                    // .max_age(3600)
                    .finish(),
            )
            .wrap(middleware::Logger::default())
            .service(get_city_list)
            .service(get_search_city_list)
            // default
            .default_service(
                // 404 for GET request
                web::resource("").route(web::get().to(p404)),
            )
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
