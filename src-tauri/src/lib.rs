use std::{collections::HashMap, error::Error, net::IpAddr};

use remarkable::Remarkable;
use scan::scan_network;
use serde::{Deserialize, Serialize};
use state::State;

mod arp_table;
mod filesystem;
mod remarkable;
mod scan;
mod state;

pub use filesystem::Folder;
pub use state::Device;

pub type RemarkableError = Box<dyn Error>;

const PRODUCT_NAME: &'static str = "rmclient";

pub struct RemarkableClient {
    pub state: State,
    connections: HashMap<String, Remarkable>,
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
        RemarkableClient {
            state,
            connections: HashMap::new(),
        }
    }

    /// Scan local network for remarkable devices
    pub async fn scan(&self) -> Result<Vec<ScanEntry>, RemarkableError> {
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

    /// Add a valid device to state
    pub fn add_device(&mut self, mac: String, device: Device) -> Result<(), RemarkableError> {
        Remarkable::connect(device.ip, &device.username, &device.password)?;
        self.state.devices.insert(mac, device);
        self.state.save()?;
        Ok(())
    }

    /// Connect to a device and save the connection for future interactions
    pub async fn connect(&mut self, mac: String) -> Result<(), Box<dyn Error>> {
        match self.state.devices.get(&mac) {
            Some(device) => {
                let remarkable =
                    Remarkable::connect(device.ip, &device.username, &device.password)?;
                self.connections.insert(mac, remarkable);
            }
            None => return Err(format!("Please add device '{mac}' first before connecting").into()),
        }

        Ok(())
    }

    pub async fn fs(&self, mac: &String) -> Result<Folder, RemarkableError> {
        match self.connections.get(mac) {
            Some(c) => {
                // c.sync(),
                todo!()
            }
            None => {
                return Err(
                    "Please connect to the device first before requesting filesystem".into(),
                )
            }
        }
    }
}
