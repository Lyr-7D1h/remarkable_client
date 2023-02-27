use std::net::IpAddr;

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
    pub async fn devices(&self) -> Vec<ScanEntry> {
        let entries = scan_network().await;
        return entries
            .into_iter()
            .map(|entry| ScanEntry {
                known: self.state.devices.contains_key(&entry.1),
                ip: entry.0,
                mac: entry.1,
            })
            .collect();
    }
}
