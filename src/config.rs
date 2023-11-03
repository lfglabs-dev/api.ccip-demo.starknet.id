use serde::{self, Deserialize};
use starknet::core::types::FieldElement;
use std::env;
use std::fs;

pub_struct!(Clone, Deserialize; Server { port: u16 });

pub_struct!(Clone, Deserialize;  Starknet {
    rpc_url: String,
    private_key : FieldElement,
});

pub_struct!(Clone, Deserialize;  Notion {
    api_url: String,
    database_id: String,
    secret: String,
});

pub_struct!(Clone, Deserialize;  Config {
    server: Server,
    starknet: Starknet,
    notion: Notion,
});

pub fn load() -> Config {
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() <= 1 {
        "config.toml"
    } else {
        args.get(1).unwrap()
    };
    let file_contents = fs::read_to_string(config_path);
    if file_contents.is_err() {
        panic!("error: unable to read file with path \"{}\"", config_path);
    }

    match toml::from_str(file_contents.unwrap().as_str()) {
        Ok(loaded) => loaded,
        Err(err) => {
            panic!("error: unable to deserialize config. {}", err);
        }
    }
}
