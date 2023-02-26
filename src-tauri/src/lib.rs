use std::net::IpAddr;

use scan::scan_network;
use state::State;

mod remarkable;
mod scan;
mod state;

pub struct RemarkableClient {
    pub state: State,
}

impl RemarkableClient {
    pub fn new() -> RemarkableClient {
        let state = State::load();
        println!("{state:?}");
        RemarkableClient { state }
    }

    /// Scan local network for remarkable devices
    pub async fn scan(&self) -> Vec<IpAddr> {
        scan_network().await;
        todo!();
    }
}
