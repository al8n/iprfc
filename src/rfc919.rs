use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::BROADCAST, 32);

/// [RFC 919] Broadcasting Internet Datagrams
///
/// **Addresses:**
/// - **IPv4:**
///   1. `255.255.255.255/32`: §7 Broadcast IP Addressing - Proposed Standards
///
/// [RFC 919]: https://datatracker.ietf.org/doc/rfc919/
pub const RFC919: RFC = RFC {
  id: 919,
  ip_addresses: &[IpNet::V4(IPV4_1)],
  ipv4_addresses: &[IPV4_1],
  ipv6_addresses: &[],
};

#[test]
fn t() {
  let addr: Ipv4Net = "255.255.255.255/32".parse().unwrap();
  assert_eq!(IPV4_1, addr);
}
