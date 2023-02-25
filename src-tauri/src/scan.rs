use std::net::IpAddr;

use local_ip_address::local_ip;

pub fn scan_network() -> Vec<IpAddr> {
    let my_local_ip = local_ip().unwrap();
    println!("{my_local_ip:?}");
    todo!()
}
