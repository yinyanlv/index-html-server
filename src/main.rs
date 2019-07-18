extern crate actix_web;
extern crate actix_files;
extern crate serde_json;

mod config;

use actix_web::{HttpServer, App};
use actix_files::Files;
use serde_json::Value;
use config::JsonConfig;

fn main() -> std::io::Result<()> {

    let config: Value = JsonConfig::new(r"./config.json").unwrap().get_value();

    let folder_path = config.get("folderPath")
                        .map(|val| {
                            if let Some(text) = val.as_str() {
                                text.to_string()
                            } else {
                                "./static/".to_string()
                            }
                        })
                        .unwrap_or("./static/".to_string());

    let entry_file = config.get("entryFile")
                        .map(|val| {
                            if let Some(text) = val.as_str() {
                                text.to_string()
                            } else {
                                "index.html".to_string()
                            }
                        })
                        .unwrap_or("index.html".to_string());

    let port = config.get("port")
                        .map(|val| {
                            if let Some(number) = val.as_u64() {
                                number
                            } else {
                                3000
                            }
                        })
                        .unwrap_or(3000);

    let bind_str = &*format!("127.0.0.1:{}", port);

    println!("Server for index.html is listening on port {}！", port);

    HttpServer::new(move || {
       App::new()
        .service(
            Files::new("/", &*folder_path).index_file(&*entry_file)
        )
    })
    .bind(bind_str)?
    .run()
}