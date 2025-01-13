use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 0), 96);

/// [RFC 2765] Stateless IP/ICMP Translation Algorithm
/// (SIIT) (obsoleted by RFCs 6145, which itself was
/// later obsoleted by 7915).
///
/// **Addresses:**
/// - **IPv6:**
///   1. `0:0:0:0:0:ffff:0:0/96`
///
/// [RFC 2765]: https://datatracker.ietf.org/doc/rfc2765/
pub const RFC2765: RFC = RFC {
  id: 2765,
  ip_addresses: &[IpNet::V6(IPV6_1)],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1],
};

#[test]
fn t() {
  let addr: Ipv6Net = "0:0:0:0:0:ffff:0:0/96".parse().unwrap();
  assert_eq!(IPV6_1, addr);
}
