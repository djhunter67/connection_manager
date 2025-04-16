use std::sync::{Arc, Mutex};

use connection_manager::{PocoConfig, get_poco_config};
use mdns_scanner::mdns_scan;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let scan_results = mdns_scan();

    // println!("mDns scan results: {:#?}", scan_results.unwrap());

    let pocos_ip: Vec<String> = mdns_scan()?
        .iter()
        .filter(|poco_scan_res| poco_scan_res.name().contains("poco"))
        .map(|poco_scan_res| {
            println!("Name: {}", poco_scan_res.name());
            println!("Address: {}", poco_scan_res.address());
            poco_scan_res.address()
        })
        .collect();

    const ENDPOINT: &str = "/cfg/confc.json";
    // let ws: String = format!("ws://{}/websocket/ws.cgi", URI);

    println!("Poco IPs: {:#?}", pocos_ip);

    let return_val: Arc<Mutex<Vec<PocoConfig>>> = Arc::new(Mutex::new(Vec::new()));

    pocos_ip.into_par_iter().for_each(|uri| {
        let uri = format!("http://{}{}", uri, ENDPOINT);
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

    return_val.lock().unwrap().iter().for_each(|poco_config| {
        println!("PocoConfig schema: {}", poco_config.schema);
        println!("PocoConfig compat: {}", poco_config.compat);
    });

    Ok(())
}
