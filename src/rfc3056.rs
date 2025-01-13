use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(8194, 0, 0, 0, 0, 0, 0, 0), 16);

/// [RFC 3056]  Connection of IPv6 Domains via IPv4 Clouds
///
/// **Addresses:**
/// - **IPv6:**
///   1. `2002::/16`: §2 IPv6 Prefix Allocation
///
/// [RFC 3056]: https://datatracker.ietf.org/doc/rfc3056/
pub const RFC3056: RFC = RFC {
  id: 3056,
  ip_addresses: &[IpNet::V6(IPV6_1)],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "2002::/16".parse().unwrap();
  assert_eq!(IPV6_1, addr);
}
