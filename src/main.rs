extern crate actix_web;
extern crate actix_files;
#[macro_use]
extern crate serde_json;
extern crate clap;

mod config;

use actix_web::{HttpServer, App};
use actix_files::Files;
use serde_json::Value;
use clap::{App as ClapApp, Arg, ArgMatches};
use config::JsonConfig;

fn main() -> std::io::Result<()> {

    let clap_matches = init_clap();
                
    let config_path = clap_matches.value_of("config");

    let config: Value = match config_path {
        Some(path) => JsonConfig::new(path).unwrap().get_value(),
        None => json!({
            "folderPath": "./static/",
            "entryFile": "index.html", 
            "port": 8000
        })
    };

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

fn init_clap<'a>() ->  ArgMatches<'a> {
    ClapApp::new("index-html-server")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Set a custom json config file")
                .takes_value(true)
        )
        .get_matches()
}