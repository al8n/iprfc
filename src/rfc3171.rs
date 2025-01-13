use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(224, 0, 0, 0), 4);

/// [RFC 3171] IANA Guidelines for IPv4 Multicast Address Assignments
///
/// **Addresses:**
/// - **IPv4:**
///   1. `224.0.0.0/4`
///
/// [RFC 3171]: https://datatracker.ietf.org/doc/rfc3171/
pub const RFC3171: RFC = RFC {
  id: 3171,
  ip_nets: &[IpNet::V4(IPV4_1)],
  ipv4_nets: &[IPV4_1],
  ipv6_nets: &[],
};

#[test]
fn t() {
  let addr: Ipv4Net = "224.0.0.0/4".parse().unwrap();
  assert_eq!(IPV4_1, addr);
}
