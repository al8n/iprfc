use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 2, 0), 24);
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 51, 100, 0), 24);
const IPV4_3: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(203, 0, 113, 0), 24);
const IPV4_4: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 18, 0, 0), 15);

/// [RFC 5735] Special Use IPv4 Addresses
///
/// **Addresses:**
/// - **IPv4:**
///   1. `192.0.2.0/24`: TEST-NET-1
///   2. `198.51.100.0/24`: TEST-NET-2
///   3. `203.0.113.0/24`: TEST-NET-3
///   4. `198.18.0.0/15`: Benchmarks
///
/// [RFC 5735]: https://datatracker.ietf.org/doc/rfc5735/
pub const RFC5735: RFC = RFC {
  id: 5735,
  ip_addresses: &[IpNet::V4(IPV4_1), IpNet::V4(IPV4_2), IpNet::V4(IPV4_3), IpNet::V4(IPV4_4)],
  ipv4_addresses: &[IPV4_1, IPV4_2, IPV4_3, IPV4_4],
  ipv6_addresses: &[],
};

#[test]
fn t() {
  for (idx, s) in ["192.0.2.0/24", "198.51.100.0/24", "203.0.113.0/24", "198.18.0.0/15"]
    .iter()
    .enumerate()
  {
    let addr: Ipv4Net = s.parse().unwrap();
    assert_eq!(RFC5735.ipv4_addresses[idx], addr);
  }
}
