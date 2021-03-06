//! Resolves IP of an address if it is neither IPv4 nor IPv6

use crate::global::Result;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use anyhow::bail;
use covers::*;

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

#[mocked(tests::lookup_host)]
fn lookup_host(host: &str) -> std::io::Result<Vec<IpAddr>> {
    dns_lookup::lookup_host(host)
}

fn is_ip_v4(hostname: &str) -> bool {
    let result: std::result::Result<Ipv4Addr, std::net::AddrParseError> = hostname.parse();
    result.is_ok()
}

fn is_ip_v6(hostname: &str) -> bool {
    let result: std::result::Result<Ipv6Addr, std::net::AddrParseError> = hostname.parse();
    result.is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dig_ipv4() {
        assert_eq!(dig("192.168.0.1").unwrap(), "192.168.0.1")
    }

    #[test]
    fn dig_ipv6() {
        assert_eq!(
            dig("2001:0DB8:3C4D:7777:0260:3EFF:FE15:9501").unwrap(),
            "2001:0DB8:3C4D:7777:0260:3EFF:FE15:9501"
        )
    }

    #[test]
    fn dig_host() {
        assert_eq!(dig("example.com").unwrap(), "127.1.1.1")
    }

    #[mock]
    fn lookup_host(_host: &str) -> std::io::Result<Vec<IpAddr>> {
        Ok(vec![IpAddr::from(Ipv4Addr::new(127, 1, 1, 1))])
    }
}
