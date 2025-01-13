use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// 64:ff9b::/96
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x64, 0xff9b, 0, 0, 0, 0, 0, 0), 96);

/// [RFC 6052] IPv6 Addressing of IPv4/IPv6 Translators
///
/// **Addresses:**
/// - **IPv6:**
///   1. `64:ff9b::/96`: §2.1. Well-Known Prefix
///
/// [RFC 6052]: https://datatracker.ietf.org/doc/rfc6052/
pub const RFC6052: RFC = RFC {
  id: 6052,
  ip_addresses: &[IpNet::V6(IPV6_1)],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "64:ff9b::/96".parse().unwrap(); 
  assert_eq!(IPV6_1, addr);
}
