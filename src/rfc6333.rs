use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 0), 29);

/// [RFC 6333] Dual-Stack Lite Broadband Deployments Following IPv4 Exhaustion
///
/// **Addresses:**
/// - **IPv4:**
///   1. `192.0.0.0/29`: §5.7 Well-Known IPv4 Address
///
/// [RFC 6333]: https://datatracker.ietf.org/doc/rfc6333/
pub const RFC6333: RFC = RFC {
  id: 6333,
  ip_addresses: &[IpNet::V4(IPV4_1)],
  ipv4_addresses: &[IPV4_1],
  ipv6_addresses: &[],
};

#[test]
fn t() {
  let addr: Ipv4Net = "192.0.0.0/29".parse().unwrap();
  assert_eq!(IPV4_1, addr);
}
