use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(169, 254, 0, 0), 16);

/// [RFC 3927] Dynamic Configuration of IPv4 Link-Local Addresses
///
/// **Addresses:**
/// - **IPv4:**
///   1. `169.254.0.0/16`: §2.1 Link-Local Address Selection
///
/// [RFC 3927]: https://datatracker.ietf.org/doc/rfc3927/
pub const RFC3927: RFC = RFC {
  id: 3927,
  ip_nets: &[IpNet::V4(IPV4_1)],
  ipv4_nets: &[IPV4_1],
  ipv6_nets: &[],
};

#[test]
fn t() {
  let addr: Ipv4Net = "169.254.0.0/16".parse().unwrap();
  assert_eq!(IPV4_1, addr);
}
