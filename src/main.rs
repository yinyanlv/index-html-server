extern crate actix_web;
extern crate actix_files;
#[macro_use]
extern crate serde_json;
extern crate clap;
#[macro_use]
extern crate lazy_static;

mod config;

use std::path::Path;
use std::io::Error;
use std::path::PathBuf;
use actix_web::{HttpServer, App, web, HttpRequest};
use actix_files::NamedFile;
use serde_json::Value;
use clap::{App as ClapApp, Arg, ArgMatches};
use config::JsonConfig;

lazy_static! {

    static ref CLAP_MATCHES: ArgMatches<'static> = {
        init_clap()
    };

    static ref CONFIG: Value = {
        let config_path = CLAP_MATCHES.value_of("config");

        // 配置优先级：自定义配置文件 > 当前文件夹config.json > 默认配置
        match config_path {
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
        }
    };

    static ref FOLDER_PATH: String = {
        CONFIG.get("folderPath")
            .map(|val| {
                if let Some(text) = val.as_str() {
                    text.to_string()
                } else {
                    "./static/".to_string()
                }
            })
            .unwrap_or("./static/".to_string())
    };

    static ref ENTRY_FILE: String = {
        CONFIG.get("entryFile")
            .map(|val| {
                if let Some(text) = val.as_str() {
                    text.to_string()
                } else {
                    "index.html".to_string()
                }
            })
            .unwrap_or("index.html".to_string())
    };

    static ref PORT: u64 = {
        CONFIG.get("port")
            .map(|val| {
                if let Some(number) = val.as_u64() {
                    number
                } else {
                    3000
                }
            })
            .unwrap_or(3000)
    };
}

fn main() -> std::io::Result<()> {

    let bind_str = &*format!("127.0.0.1:{}", PORT.to_string());

    println!("Server for index.html is listening on port {}！", PORT.to_string());

    HttpServer::new(|| {
       App::new()
        .route("/{file_path:.*}", web::get().to(handle))
    })
    .bind(bind_str)?
    .run()
}

fn handle(req: HttpRequest) -> Result<NamedFile, Error> {

    let mut file_path: &str = req.match_info().get("file_path").unwrap_or("");

    if file_path == "" {
        file_path = &*ENTRY_FILE;
    }

    let mut real_path = FOLDER_PATH.to_string() + file_path;
    let temp_path = Path::new(&*real_path);
    if !temp_path.exists() || temp_path.is_dir() {
        real_path = FOLDER_PATH.to_string() + &*ENTRY_FILE;
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