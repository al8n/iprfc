use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::UNSPECIFIED, 8);
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(127, 0, 0, 0), 8);

/// [RFC 1122] Requirements for Internet Hosts -- Communication Layers
///
/// **Addresses:**
/// - **IPv4:**
///   1. `0.0.0.0/8`: §3.2.1.3
///   2. `127.0.0.0/8`: §3.2.1.3
///
/// [RFC 1122]: https://datatracker.ietf.org/doc/rfc1122/
pub const RFC1122: RFC = RFC {
  id: 1122,
  ip_nets: &[IpNet::V4(IPV4_1), IpNet::V4(IPV4_2)],
  ipv4_nets: &[IPV4_1, IPV4_2],
  ipv6_nets: &[],
};

#[test]
fn t() {
  for (idx, s) in ["0.0.0.0/8", "127.0.0.0/8"].iter().enumerate() {
    let addr: Ipv4Net = s.parse().unwrap();
    assert_eq!(RFC1122.ipv4_nets[idx], addr, "{s}");
  }
}
