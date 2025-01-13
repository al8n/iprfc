use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// 0100::/64
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x0100, 0, 0, 0, 0, 0, 0, 0), 64);

/// [RFC 6666] A Discard Prefix for IPv6
///
/// **Addresses:**
/// - **IPv6:**
///   1. `0100::/64`
///
/// [RFC 6666]: https://datatracker.ietf.org/doc/rfc6666/
pub const RFC6666: RFC = RFC {
  id: 6666,
  ip_addresses: &[IpNet::V6(IPV6_1)],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "0100::/64".parse().unwrap(); 
  assert_eq!(IPV6_1, addr);
}
