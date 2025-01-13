use core::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 88, 99, 0), 24);
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(8194, 49240, 25345, 0, 0, 0, 0, 0), 120);

/// [RFC 3068] An Anycast Prefix for 6to4 Relay Routers (obsolete by RFC7526)
///
/// **Addresses:**
/// - **IPv4:**
///   1. `192.88.99.0/24`: § 6to4 Relay anycast address
///   2. `2002:c058:6301::/120`: §2.5 6to4 IPv6 relay anycast address
///
/// [RFC 3068]: https://datatracker.ietf.org/doc/rfc3068/
pub const RFC3068: RFC = RFC {
  id: 3068,
  ip_addresses: &[IpNet::V4(IPV4_1), IpNet::V6(IPV6_1)],
  ipv4_addresses: &[IPV4_1],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv4Net = "192.88.99.0/24".parse().unwrap();
  assert_eq!(IPV4_1, addr);

  let addr: Ipv6Net = "2002:c058:6301::/120".parse().unwrap();
  assert_eq!(IPV6_1, addr);
}
