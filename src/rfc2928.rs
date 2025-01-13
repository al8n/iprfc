use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(8193, 0, 0, 0, 0, 0, 0, 0), 16);

/// [RFC 2928] Initial IPv6 Sub-TLA ID Assignments
///
/// **Addresses:**
/// - **IPv6:**
///   1. `2001::/16`: Superblock
///
/// [RFC 2928]: https://datatracker.ietf.org/doc/rfc2928/
pub const RFC2928: RFC = RFC {
  id: 2928,
  ip_addresses: &[IpNet::V6(IPV6_1)],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "2001::/16".parse().unwrap();
  assert_eq!(IPV6_1, addr);
}
