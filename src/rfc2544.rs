use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 18, 0, 0), 15);

/// [RFC 2544] Benchmarking Methodology for Network Interconnect Devices
///
/// **Addresses:**
/// - **IPv4:**
///   1. `198.18.0.0/15`
///
/// [RFC 2544]: https://datatracker.ietf.org/doc/rfc2544/
pub const RFC2544: RFC = RFC {
  id: 2544,
  ip_nets: &[IpNet::V4(IPV4_1)],
  ipv4_nets: &[IPV4_1],
  ipv6_nets: &[],
};

#[test]
fn t() {
  let addr: Ipv4Net = "198.18.0.0/15".parse().unwrap();
  assert_eq!(IPV4_1, addr);
}
