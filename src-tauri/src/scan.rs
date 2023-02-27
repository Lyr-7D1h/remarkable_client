use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use futures::{future, lock::Mutex, stream, StreamExt};
use local_ip_address::local_ip;
use tokio::net::TcpStream;

use crate::arp_table::ArpTable;

const TIMEOUT: u64 = 3;
const MAC_PREFIX: &'static str = "70:F7:54";

async fn is_remarkable_device(target: IpAddr, table: &ArpTable) -> Option<(IpAddr, String)> {
    let timeout = Duration::from_secs(TIMEOUT);
    let socket_address = SocketAddr::new(target.clone(), 22);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => {
            if let Some(mac) = table.get(&target) {
                let mac = mac.to_uppercase();
                if mac.to_uppercase().starts_with(MAC_PREFIX) {
                    return Some((target, mac));
                }
            }
        }
        _ => {}
    }

    return None;
}

/// Scan your local network for remarkable devices, returns ip and mac address
pub async fn scan_network() -> Vec<(IpAddr, String)> {
    let local_ip = local_ip().unwrap();
    if local_ip.is_ipv6() {
        panic!("Ipv6 not supported");
    }

    let gateway = local_ip
        .to_string()
        .split(".")
        .take(3)
        .collect::<Vec<&str>>()
        .join(".");

    let table = ArpTable::load().await;

    let entries = future::join_all((0..255).into_iter().map(|i| {
        let ip = format!("{gateway}.{i}").parse::<IpAddr>().unwrap();
        return is_remarkable_device(ip, &table);
    }))
    .await;

    return entries.into_iter().filter_map(|e| e).collect();
}
