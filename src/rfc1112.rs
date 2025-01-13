use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(224, 0, 0, 0), 4);

/// [RFC 1112] Host Extensions for IP Multicasting
///
/// **Addresses:**
/// - **IPv4:**
///   1. `224.0.0.0/4`: §4 Host Group Addresses
///
/// [RFC 1112]: https://datatracker.ietf.org/doc/rfc1112/
pub const RFC1112: RFC = RFC {
  id: 1112,
  ip_nets: &[IpNet::V4(IPV4_1)],
  ipv4_nets: &[IPV4_1],
  ipv6_nets: &[],
};

#[test]
fn t() {
  let addr: Ipv4Net = "224.0.0.0/4".parse().unwrap();
  assert_eq!(IPV4_1, addr);
}
