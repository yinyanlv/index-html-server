use std::io::Read;
use std::path::Path;
use std::fs::File;
use serde_json::{from_str, Value};

#[derive(Debug)]
pub struct JsonConfig(Value);

impl JsonConfig {
    
    pub fn new(path: &str) -> Result<Self, String> {
        let file_path = Path::new(path);
        let mut file = File::open(&file_path).expect(&*format!("config file {} is not exists!", path));
        let mut content = String::new();

        file.read_to_string(&mut content)
            .map_err(|err| {
                err.to_string()
            })
            .and_then(|_| {
                from_str(&*content)
                    .map_err(|err| {
                        err.to_string()
                    })
            })
            .map(|obj| {
               JsonConfig(obj)
            })
    }

    pub fn get_value(self) -> Value {
        self.0
    }
}
