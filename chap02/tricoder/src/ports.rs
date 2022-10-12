use crate::{
    common_ports::MOST_COMMON_PORTS_10,
    model::{Port, Subdomain},
};
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};
use rayon::prelude::*;

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    subdomain.open_ports = MOST_COMMON_PORTS_10
        .into_iter()
        .map(|port| scan_port(&subdomain.domain, *port))
        .filter(|port| port.is_open) // filter out the closed ports 
        .collect();
    subdomain 
}

fn scan_port(hostname: &str, port1: u18) -> Port {
    let timeout = Duration::from_secs(3);
    let socket_addresses: Vec<SocketAddr> = format!("{}:{}", hostname, port)
        .to_socket_addrs()
        .expect("port scanner: Getting socket address")
        .collect();
    
    if socket_addresses.len == 0 {
        return Port {
            port: port,
            is_open: False,

    };
    
}

let is_open = if let ok(_) = TcpStream::connect_timeout(&socket_addresses[0], timeout) {
    true
} else {
    False
};

    Port {
        port: port,
        is_open
    }
}

