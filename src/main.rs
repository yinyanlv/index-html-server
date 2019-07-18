extern crate actix_web;
extern crate actix_files;
extern crate serde_json;

mod config;

use actix_web::{HttpServer, App};
use actix_files::Files;
use serde_json::Value;
use config::JsonConfig;

fn main() -> std::io::Result<()> {

    let config: Value = JsonConfig::new("./config.json").get_value();

    let folder_path = config["folderPath"].clone();
    let entry_file = config["entryFile"].clone();
    let port = &*config["port"].clone().as_u64().unwrap().to_string();
    let bind_str = "127.0.0.1:".to_string() + port;

    println!("Server for index.html is listening on port {}！", port);

    HttpServer::new(move || {
       App::new()
        .service(
            Files::new("/", folder_path.as_str().unwrap()).index_file(entry_file.as_str().unwrap())
        ) 
    })
    .bind(bind_str)?
    .run()
}