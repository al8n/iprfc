use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// 3fff::/20
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x3fff, 0, 0, 0, 0, 0, 0, 0), 20);

/// [RFC 9637] Expanding the IPv6 Documentation Space
///
/// **Addresses:**
/// - **IPv6:**
///   1. `3fff::/20`: §3. IPv6 Documentation Prefix
///
/// [RFC 9637]: https://datatracker.ietf.org/doc/rfc9637/
pub const RFC9637: RFC = RFC {
  id: 9637,
  ip_nets: &[IpNet::V6(IPV6_1)],
  ipv4_nets: &[],
  ipv6_nets: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "3fff::/20".parse().unwrap();
  assert_eq!(IPV6_1, addr);
}
