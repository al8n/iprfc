use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// 2001:0200::/48
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0200, 0, 0, 0, 0, 0, 0), 48);

/// [RFC 5180] IPv6 Benchmarking Methodology for Network Interconnect Devices
///
/// **Addresses:**
/// - **IPv6:**
///   1. `2001:0200::/48`: §8 IANA Considerations
///
/// [RFC 5180]: https://datatracker.ietf.org/doc/rfc5180/
pub const RFC5180: RFC = RFC {
  id: 5180,
  ip_addresses: &[IpNet::V6(IPV6_1)],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "2001:0200::/48".parse().unwrap(); 
  assert_eq!(IPV6_1, addr);
}
