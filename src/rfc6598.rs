use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(100, 64, 0, 0), 10);

/// [RFC 6598] IANA-Reserved IPv4 Prefix for Shared Address Space
///
/// **Addresses:**
/// - **IPv4:**
///   1. `100.64.0.0/10`
///
/// [RFC 6598]: https://datatracker.ietf.org/doc/rfc6598/
pub const RFC6598: RFC = RFC {
  id: 6598,
  ip_nets: &[IpNet::V4(IPV4_1)],
  ipv4_nets: &[IPV4_1],
  ipv6_nets: &[],
};

#[test]
fn t() {
  let addr: Ipv4Net = "100.64.0.0/10".parse().unwrap();
  assert_eq!(IPV4_1, addr);
}
