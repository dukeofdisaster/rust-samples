// actix has changed since the book was written; 
// as such, trying to compile this give sthe following errors; i didn't try
// too hard to figure it out; would probably be a good exercise though; we 
// are missing a trait...
//  error[E0277]: the trait bound `fn() -> std::result::Result<actix_files::named::NamedFile, actix_web::Error> {<foxes as actix_web::dev::HttpServiceFactory>::register::foxes}: actix_web::dev::Factory<_, _>` is not satisfied
//  --> src/main.rs:21:4
#[macro_use]
extern crate actix_web;

use actix_web::{web, App, middleware, HttpServer, Responder, Result};
use std::{env};
use actix_files as fs;

fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

fn path_parser(info: web::Path<(String, i32)>) -> impl Responder {
    format!("You tried to reach '{}/{}'", info.0, info.1);
}

fn rust_cookbook() -> impl Responder {
    format!("Welcome to the rust cookbook");
}

#[get("/foxes")]
fn foxes() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/anon.jpg")?)
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(
        || App::new()
            .wrap(middleware::Logger::default())
            .service(foxes)
            .service(web::resource("/").to(index))
            .service(web::resource("/welcome").to(rust_cookbook))
    .service(web::resource("/{path}/{id}").to(path_parser)))
        .bind("127.0.0.1:8081")?
        .run()
}