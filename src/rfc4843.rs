use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// 2001:10::/28
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x10, 0, 0, 0, 0, 0, 0), 28);

/// [RFC 4843] An IPv6 Prefix for Overlay Routable Cryptographic Hash Identifiers (ORCHID)
///
/// **Addresses:**
/// - **IPv6:**
///   1. `2001:10::/28`: §7 IANA Considerations
///
/// [RFC 4843]: https://datatracker.ietf.org/doc/rfc4843/
pub const RFC4843: RFC = RFC {
  id: 4843,
  ip_nets: &[IpNet::V6(IPV6_1)],
  ipv4_nets: &[],
  ipv6_nets: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "2001:10::/28".parse().unwrap(); 
  assert_eq!(IPV6_1, addr);
}
