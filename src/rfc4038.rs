use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// 0:0:0:0:0:ffff::/96
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 0), 96);

/// [RFC 4038] Application Aspects of IPv6 Transition
///
/// **Addresses:**
/// - **IPv6:**
///   1. `0:0:0:0:0:ffff::/96`: §4.2. IPv6 Applications in a Dual-Stack Node
///
/// [RFC 4038]: https://datatracker.ietf.org/doc/rfc4038/
pub const RFC4038: RFC = RFC {
  id: 4038,
  ip_nets: &[IpNet::V6(IPV6_1)],
  ipv4_nets: &[],
  ipv6_nets: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "0:0:0:0:0:ffff::/96".parse().unwrap();
  assert_eq!(IPV6_1, addr);
}
