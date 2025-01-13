use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// 2001:db8::/32
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(8193, 3512, 0, 0, 0, 0, 0, 0), 32);

/// [RFC 3849] IPv6 Address Prefix Reserved for Documentation
///
/// **Addresses:**
/// - **IPv6:**
///   1. `2001:db8::/32`: §4 IANA Considerations
///
/// [RFC 3849]: https://datatracker.ietf.org/doc/rfc3849/
pub const RFC3849: RFC = RFC {
  id: 3849,
  ip_addresses: &[IpNet::V6(IPV6_1)],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "2001:db8::/32".parse().unwrap();
  assert_eq!(IPV6_1, addr);
}
