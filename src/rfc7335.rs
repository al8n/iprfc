use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 0), 29);

/// [RFC 7335] IPv4 Service Continuity Prefix
///
/// **Addresses:**
/// - **IPv4:**
///   1. `192.0.0.0/29`:  §6 IANA Considerations
///
/// [RFC 7335]: https://datatracker.ietf.org/doc/rfc7335/
pub const RFC7335: RFC = RFC {
  id: 7335,
  ip_addresses: &[IpNet::V4(IPV4_1)],
  ipv4_addresses: &[IPV4_1],
  ipv6_addresses: &[],
};

#[test]
fn t() {
  let addr: Ipv4Net = "192.0.0.0/29".parse().unwrap();
  assert_eq!(IPV4_1, addr);
}
