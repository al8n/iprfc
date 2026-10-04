use core::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 88, 99, 0), 24);
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(8194, 49240, 25345, 0, 0, 0, 0, 0), 128);

/// [RFC 3068] An Anycast Prefix for 6to4 Relay Routers (obsolete by RFC7526)
///
/// **IPv4:**
///
/// 1. `192.88.99.0/24`: 6to4 relay anycast prefix
///
/// **IPv6:**
///
/// 1. `2002:c058:6301::/128`: section 2.5 6to4 IPv6 relay anycast address
///
/// [RFC 3068]: https://datatracker.ietf.org/doc/rfc3068/
pub const RFC3068: RFC = RFC {
  id: 3068,
  ip_nets: &[IpNet::V4(IPV4_1), IpNet::V6(IPV6_1)],
  ipv4_nets: &[IPV4_1],
  ipv6_nets: &[IPV6_1],
};

#[test]
fn relay_anycast_prefix_and_address_match_rfc3068() {
  let addr: Ipv4Net = "192.88.99.0/24".parse().unwrap();
  assert_eq!(IPV4_1, addr);

  let addr: Ipv6Net = "2002:c058:6301::/128".parse().unwrap();
  assert_eq!(IPV6_1, addr);

  let anycast = Ipv6Addr::new(0x2002, 0xc058, 0x6301, 0, 0, 0, 0, 0);
  assert!(RFC3068.contains(&anycast));

  for adjacent in [
    Ipv6Addr::new(
      0x2002, 0xc058, 0x6300, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
    ),
    Ipv6Addr::new(0x2002, 0xc058, 0x6301, 0, 0, 0, 0, 1),
  ] {
    assert!(!RFC3068.contains(&adjacent));
  }

  let broad: Ipv6Net = "2002:c058:6301::/120".parse().unwrap();
  assert!(!RFC3068.contains(&broad));
}
