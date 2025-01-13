use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// fc00::/7
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(64512, 0, 0, 0, 0, 0, 0, 0), 7);

/// [RFC 4193] Unique Local IPv6 Unicast Addresses
///
/// **Addresses:**
/// - **IPv6:**
///   1. `fc00::/7`
///
/// [RFC 4193]: https://datatracker.ietf.org/doc/rfc4193/
pub const RFC4193: RFC = RFC {
  id: 4193,
  ip_nets: &[IpNet::V6(IPV6_1)],
  ipv4_nets: &[],
  ipv6_nets: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "fc00::/7".parse().unwrap(); 
  assert_eq!(IPV6_1, addr);
}
