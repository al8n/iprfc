use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 2, 0), 24);
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 51, 100, 0), 24);
const IPV4_3: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(203, 0, 113, 0), 24);

/// [RFC 5737] IPv4 Address Blocks Reserved for Documentation
///
/// **Addresses:**
/// - **IPv4:**
///   1. `192.0.2.0/24`: TEST-NET-1
///   2. `198.51.100.0/24`: TEST-NET-2
///   3. `203.0.113.0/24`: TEST-NET-3
///
/// [RFC 5737]: https://datatracker.ietf.org/doc/rfc5737/
pub const RFC5737: RFC = RFC {
  id: 5737,
  ip_addresses: &[IpNet::V4(IPV4_1), IpNet::V4(IPV4_2), IpNet::V4(IPV4_3)],
  ipv4_addresses: &[IPV4_1, IPV4_2, IPV4_3],
  ipv6_addresses: &[],
};

#[test]
fn t() {
  for (idx, s) in ["192.0.2.0/24", "198.51.100.0/24", "203.0.113.0/24"]
    .iter()
    .enumerate()
  {
    let addr: Ipv4Net = s.parse().unwrap();
    assert_eq!(RFC5737.ipv4_addresses[idx], addr);
  }
}
