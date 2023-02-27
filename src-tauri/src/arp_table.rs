use std::{collections::HashMap, net::IpAddr};

use tokio::fs::read_to_string;

#[derive(Debug, Clone)]
pub struct ArpTable {
    entries: HashMap<IpAddr, String>,
}

impl ArpTable {
    pub async fn load() -> ArpTable {
        #[cfg(not(target_os = "linux"))]
        {
            unreachable!("only linux support for the arp table")
        }

        #[cfg(target_os = "linux")]
        {
            let table = read_to_string("/proc/net/arp")
                .await
                .expect("failed to get arp table");
            let mut entries = HashMap::new();
            for entry in table.split("\n").into_iter().skip(1) {
                if entry.len() == 0 {
                    continue;
                }
                let mut parts = entry.split_whitespace();
                let ip: IpAddr = parts
                    .next()
                    .unwrap()
                    .parse::<IpAddr>()
                    .expect("invalid ip found in arp table");
                let mac = parts.skip(2).next().unwrap();
                entries.insert(ip, mac.to_string());
            }
            return ArpTable { entries };
        }
    }

    // get mac address
    pub fn get(&self, ip: &IpAddr) -> Option<&String> {
        return self.entries.get(ip);
    }
}
