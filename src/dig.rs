//! Resolves IP of an address if it is neither IPv4 nor IPv6

use crate::global::Result;

use std::net::{Ipv4Addr, Ipv6Addr};

use anyhow::bail;
use dns_lookup::lookup_host;

pub fn dig(hostname: &str) -> Result<String> {
    if is_ip_v4(hostname) || is_ip_v6(hostname) {
        return Ok(hostname.to_string());
    }
    let ip_addr_list = match lookup_host(&hostname) {
        Ok(x) => x,
        Err(_) => bail!("cannot resolve IP by hostname: {}", hostname),
    };

    Ok(ip_addr_list.first().expect("no IP resolved").to_string())
}

fn is_ip_v4(hostname: &str) -> bool {
    let result: std::result::Result<Ipv4Addr, std::net::AddrParseError> = hostname.parse();
    result.is_ok()
}

fn is_ip_v6(hostname: &str) -> bool {
    let result: std::result::Result<Ipv6Addr, std::net::AddrParseError> = hostname.parse();
    result.is_ok()
}
