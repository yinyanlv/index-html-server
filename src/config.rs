
use std::io::Read;
use std::path::Path;
use std::fs::File;
use serde_json::{from_str, Value};

#[derive(Debug)]
pub struct JsonConfig(Value);

impl JsonConfig {
    
    pub fn new(path: &str) -> Self {
        let path = Path::new(path);
        let mut file = File::open(&path).expect("json file is not exists!");
        let mut json_str = String::new();

        file.read_to_string(&mut json_str).expect("");

        let obj: Value = from_str(&*json_str).unwrap();

        JsonConfig(obj)
    }

    pub fn get_value(self) -> Value {

        self.0
    }
}
