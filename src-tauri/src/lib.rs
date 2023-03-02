use std::{error::Error, net::IpAddr};

use remarkable::Remarkable;
use scan::scan_network;
use serde::{Deserialize, Serialize};
use state::State;

mod arp_table;
mod remarkable;
mod scan;
mod state;

pub use state::Device;

pub struct RemarkableClient {
    pub state: State,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanEntry {
    known: bool,
    ip: IpAddr,
    mac: String,
}

impl RemarkableClient {
    pub fn new() -> RemarkableClient {
        let state = State::load();
        println!("{state:?}");
        RemarkableClient { state }
    }

    /// Scan local network for remarkable devices
    pub async fn devices(&self) -> Result<Vec<ScanEntry>, Box<dyn Error>> {
        let entries = scan_network().await?;
        return Ok(entries
            .into_iter()
            .map(|entry| ScanEntry {
                known: self.state.devices.contains_key(&entry.1),
                ip: entry.0,
                mac: entry.1,
            })
            .collect());
    }

    pub async fn connect(&self, mac: &String) -> Result<(), ()> {
        match self.state.devices.get(mac) {
            Some(device) => {
                let remarkable = Remarkable::connect(&device);
            }
            None => {}
        }

        todo!()
    }
}
