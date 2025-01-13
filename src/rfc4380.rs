use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// 2001:0000::/32
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0, 0, 0, 0, 0, 0, 0), 32);

/// [RFC 4380] Teredo: Tunneling IPv6 over UDP through  Network Address Translations (NATs)
///
/// **Addresses:**
/// - **IPv6:**
///   1. `2001:0000::/32`: §2.6 Global Teredo IPv6 Service Prefix
///
/// [RFC 4380]: https://datatracker.ietf.org/doc/rfc4380/
pub const RFC4380: RFC = RFC {
  id: 4380,
  ip_addresses: &[IpNet::V6(IPV6_1)],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "2001:0000::/32".parse().unwrap(); 
  assert_eq!(IPV6_1, addr);
}
