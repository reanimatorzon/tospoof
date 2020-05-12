use crate::global::Result;
use dns_lookup::lookup_host;

pub fn dig(hostname: &str) -> Result<String> {
    let ip_addr_list = lookup_host(&hostname)
        .unwrap_or_else(|_| panic!("cannot resolve IP by hostname: {}", hostname));
    Ok(ip_addr_list.first().expect("no IP resolved").to_string())
}
