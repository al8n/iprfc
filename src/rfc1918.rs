use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(10, 0, 0, 0), 8);
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(172, 16, 0, 0), 12);
const IPV4_3: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 168, 0, 0), 16);

/// [RFC 1918] Address Allocation for Private Internets
///
/// **Addresses:**
/// - **IPv4:**
///   1. `10.0.0.0/8`
///   2. `172.16.0.0/12`
///   3. `192.168.0.0/16`
///
/// [RFC 1918]: https://datatracker.ietf.org/doc/rfc1918/
pub const RFC1918: RFC = RFC {
  id: 1918,
  ip_addresses: &[IpNet::V4(IPV4_1), IpNet::V4(IPV4_2), IpNet::V4(IPV4_3)],
  ipv4_addresses: &[IPV4_1, IPV4_2, IPV4_3],
  ipv6_addresses: &[],
};

#[test]
fn t() {
  for (idx, s) in ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"]
    .iter()
    .enumerate()
  {
    let addr: Ipv4Net = s.parse().unwrap();
    assert_eq!(RFC1918.ipv4_addresses[idx], addr);
  }
}
