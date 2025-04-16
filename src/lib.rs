use std::collections::HashMap;

use reqwest::header::HeaderName;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct PocoConfig {
    #[serde(rename = "$schema")]
    pub schema: String,
    #[serde(rename = "$compat")]
    pub compat: u8,
    #[serde(rename = "$seq")]
    seq: u8,
    clr: Vec<Clr>,
    tgt: Vec<Tgt>,
    sw: Vec<Sw>,
    sw_tgt: Vec<SwTgt>,
    lay: Vec<Lay>,
    lay_sw: Vec<LaySw>,
    extsw: Option<Vec<ExtSw>>,
    wint: Option<Vec<Win>>,
    wire: Option<Vec<Wire>>,
    lin: Lin,
}

#[derive(Debug, Deserialize)]
pub struct Clr {
    id: u8,
    txt: String,
    typ: u8,
    val: String,
}

#[derive(Debug, Deserialize)]
pub struct Tgt {
    id: u8,
    txt: String,
    chan: u8,
    clan: u8,
    out: u8,
    syn: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Sw {
    id: u8,
    txt: String,
    wdgt: u8,
}

#[derive(Debug, Deserialize)]
pub struct SwTgt {
    id: u8,
    clr: u8,
    dim: u8,
    tgt: u8,
    sw: u8,
    typ: u8,
}

#[derive(Debug, Deserialize)]
pub struct Lay {
    id: u8,
    txt: String,
}

#[derive(Debug, Deserialize)]
pub struct LaySw {
    id: u8,
    lay: u8,
    sw: u8,
}

#[derive(Debug, Deserialize)]
pub struct ExtSw {
    id: u8,
    sw: u8,
}

#[derive(Debug, Deserialize)]
pub struct Win {
    id: u8,
    typ: u8,
    en: u8,
}

#[derive(Debug, Deserialize)]
pub struct Wire {
    id: u8,
    idx: u8,
    wnt: u8,
    #[serde(rename = "in")]
    inny: u8,
    out: u8,
    esw: u8,
}

#[derive(Debug, Deserialize, Default)]
pub struct Lin {
    en: u8,
    baud: u8,
}

pub fn get_poco_config(uri: &str) -> Result<PocoConfig, Box<dyn std::error::Error>> {
    let headers: HashMap<String, String> = HashMap::from([
        ("Accept".to_string(), "*/*".to_string()),
        (
            "Accept-Encoding".to_string(),
            "gzip, deflate, br".to_string(),
        ),
        ("Accept-Language".to_string(), "en-US,en;q=0.9".to_string()),
    ]);

    let builder = reqwest::blocking::ClientBuilder::new()
        .default_headers(
            headers
                .iter()
                .map(|(k, v)| {
                    (
                        HeaderName::from_bytes(k.as_bytes()).unwrap(),
                        v.parse().unwrap(),
                    )
                })
                .collect(),
        )
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .timeout(std::time::Duration::from_secs(2));

    let client = builder.build()?;

    let response = client.get(uri).send()?;

    if response.status().is_success() {
        let poco_config: PocoConfig = match serde_json::from_str(&response.text()?) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to deserialize JSON: {}", e);
                return Err(e.into());
            }
        };
        Ok(poco_config)
    } else {
        Err(format!("Failed to fetch config: {}", response.status()).into())
    }
}
