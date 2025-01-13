use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// 2001:0000::/23
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0, 0, 0, 0, 0, 0, 0), 23);

/// [RFC 4773] Administration of the IANA Special Purpose IPv6 Address Block
///
/// **Addresses:**
/// - **IPv6:**
///   1. `2001:0000::/23`: IANA
///
/// [RFC 4773]: https://datatracker.ietf.org/doc/rfc4773/
pub const RFC4773: RFC = RFC {
  id: 4773,
  ip_addresses: &[IpNet::V6(IPV6_1)],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "2001:0000::/23".parse().unwrap(); 
  assert_eq!(IPV6_1, addr);
}
