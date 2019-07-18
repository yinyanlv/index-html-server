extern crate actix_web;
extern crate actix_files;

use actix_web::{HttpServer, App};
use actix_files::Files;

fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
       App::new()
        .service(
            Files::new("/", "./static/").index_file("index.html")
        ) 
    })
    .bind("127.0.0.1:8000")?
    .run()
}