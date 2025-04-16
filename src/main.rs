use std::sync::{Arc, Mutex};

use connection_barrage::{PocoConfig, get_poco_config};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const URI: [&str; 2] = ["http://192.168.33.206", "http://192.168.33.239"];
    const ENDPOINT: &str = "/cfg/confc.json";
    // let ws: String = format!("ws://{}/websocket/ws.cgi", URI);

    println!("URI connection strings: {:#?}", URI.to_vec());

    let return_val: Arc<Mutex<Vec<PocoConfig>>> = Arc::new(Mutex::new(Vec::new()));

    URI.into_par_iter().for_each(|uri| {
        let uri = format!("{}{}", uri, ENDPOINT);
        return_val
            .lock()
            .unwrap()
            .push(match get_poco_config(&uri) {
                Ok(poco_config) => {
                    // println!("PocoConfig schema: {}", poco_config.schema);
                    // println!("PocoConfig compat: {}", poco_config.compat);
                    poco_config
                }
                Err(e) => {
                    println!("Error: {}", e);
                    PocoConfig::default()
                }
            });
    });

    return_val
        .lock()
        .unwrap()
        .par_iter()
        .for_each(|poco_config| {
            println!("PocoConfig schema: {}", poco_config.schema);
            println!("PocoConfig compat: {}", poco_config.compat);
        });

    Ok(())
}
