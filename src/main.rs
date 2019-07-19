extern crate actix_web;
extern crate actix_files;
#[macro_use]
extern crate serde_json;
extern crate clap;

mod config;

use std::path::Path;
use std::io::Error;
use std::path::PathBuf;
use actix_web::{HttpServer, App, web, HttpRequest};
use actix_files::NamedFile;
use serde_json::Value;
use clap::{App as ClapApp, Arg, ArgMatches};
use config::JsonConfig;

fn main() -> std::io::Result<()> {

    let clap_matches = init_clap();

    let config_path = clap_matches.value_of("config");

    // 配置优先级：自定义配置文件 > 当前文件夹config.json > 默认配置
    let config: Value = match config_path {
        Some(path) => JsonConfig::new(path).unwrap().get_value(),
        None => {
            let default_path = "./config.json";
            
            if Path::new(default_path).exists() {
                JsonConfig::new(default_path).unwrap().get_value()
            } else {
                json!({
                    "folderPath": "./static/",
                    "entryFile": "index.html", 
                    "port": 8000
                })
            }
        }
    };

    let _folder_path = config.get("folderPath")
                        .map(|val| {
                            if let Some(text) = val.as_str() {
                                text.to_string()
                            } else {
                                "./static/".to_string()
                            }
                        })
                        .unwrap_or("./static/".to_string());

    let _entry_file = config.get("entryFile")
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
        .route("/{file_path:.*}", web::get().to(handle))
    })
    .bind(bind_str)?
    .run()
}

fn handle(req: HttpRequest) -> Result<NamedFile, Error> {

    let folder_path = "./static/";
    let entry_file = "index.html";

    let mut file_path: &str = req.match_info().get("file_path").unwrap_or("");

    if file_path == "" {
        file_path = entry_file.clone();
    }

    let mut real_path = folder_path.to_string() + file_path;
    let temp_path = Path::new(&*real_path);
    if !temp_path.exists() || temp_path.is_dir() {
        real_path = folder_path.to_string() + entry_file.clone();
    }

    let path_buf: PathBuf = real_path.parse().unwrap();

    Ok(NamedFile::open(path_buf)?)
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