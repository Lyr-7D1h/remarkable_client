use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use futures::{stream, StreamExt};
use local_ip_address::local_ip;
use pnet::packet::arp::{ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::Packet;
use pnet::{
    datalink::{Channel, NetworkInterface},
    packet::{
        arp::{ArpHardwareTypes, ArpOperations},
        ethernet::EtherTypes,
        MutablePacket,
    },
    util::MacAddr,
};
use tokio::net::TcpStream;

const TIMEOUT: u64 = 3;
const MAC_PREFIX: &'static str = "70:F7:54";

fn get_mac_through_arp(interface: NetworkInterface, target_ip: Ipv4Addr) -> MacAddr {
    let source_ip = interface
        .ips
        .iter()
        .find(|ip| ip.is_ipv4())
        .map(|ip| match ip.ip() {
            IpAddr::V4(ip) => ip,
            _ => unreachable!(),
        })
        .unwrap();

    let (mut sender, mut receiver) = match pnet::datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(interface.mac.unwrap());
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(interface.mac.unwrap());
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    sender
        .send_to(ethernet_packet.packet(), None)
        .unwrap()
        .unwrap();

    println!("Sent ARP request");

    while let buf = receiver.next().unwrap() {
        let arp = ArpPacket::new(&buf[MutableEthernetPacket::minimum_packet_size()..]).unwrap();
        if arp.get_sender_proto_addr() == target_ip
            && arp.get_target_hw_addr() == interface.mac.unwrap()
        {
            println!("Received reply");
            return arp.get_sender_hw_addr();
        }
    }
    panic!("Never reach here")
}

async fn scan(target: IpAddr) {
    let timeout = Duration::from_secs(TIMEOUT);
    let socket_address = SocketAddr::new(target.clone(), 22);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => {
            let interfaces = pnet::datalink::interfaces();
            let interface = interfaces
                .into_iter()
                .find(|iface| {
                    println!("{iface:?}");
                    return iface.is_broadcast() && !iface.is_loopback() && iface.is_up();
                })
                .unwrap();
            match target {
                IpAddr::V6(_) => (),
                IpAddr::V4(target) => {
                    get_mac_through_arp(interface, target);
                    println!("{target:?}",)
                }
            }
        }
        _ => {}
    }
}

/// Scan your local network for remarkable devices
pub async fn scan_network() -> Vec<IpAddr> {
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

    let ips = stream::iter(
        (0..255)
            .into_iter()
            .map(|i| format!("{gateway}.{i}").parse::<IpAddr>().unwrap()),
    );

    ips.for_each_concurrent(255, |ip| scan(ip)).await;

    println!("{local_ip:?}");
    todo!()
}
