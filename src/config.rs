use serde::Deserialize;
/**
            "mongo": {
                "url": "mongodb://localhost",
                "db_name": "feed"
            },
            "rabbitmq": {
                "addr": "amqp://localhost"
            },
            "clients": {
                "mns": {
                    "address": "localhost:55556"
                }
            },
            "metrics": {
                "db_query_hist": [0.001, 0.003, 0.005, 0.01, 0.015, 0.02, 0.03, 0.05, 0.08, 0.1, 0.3, 0.5, 1, 2, 3, 5]
            }

*/
use std::fs::read_to_string;
use std::path::Path;
use std::string::String;

#[derive(Debug, Deserialize)]
pub struct MongoCfg {
    pub url: String,
    pub db_name: String,
}

#[derive(Debug, Deserialize)]
struct RabbitCfg {
    #[serde(rename = "addr")]
    address: String,
}

#[derive(Debug, Deserialize)]
struct GrpcClientCfg {
    address: String,
}

#[derive(Debug, Deserialize)]
struct Clients {
    mns: GrpcClientCfg,
}
#[derive(Debug, Deserialize)]
pub struct Config {
    pub mongo: MongoCfg,
    rabbitmq: RabbitCfg,
    clients: Clients,
}

pub fn resolve_cfg() -> Result<Config, ()> {
    let json_file_path = Path::new("configs/cfg.json");
    let json_file_str = read_to_string(json_file_path).expect("file not found");
    // use instead of from_reader
    let cfg: Config = serde_json::from_str(&json_file_str).expect("error while reading json");
    Ok(cfg)
}
