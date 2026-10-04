use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 0), 96);
const IPV6_2: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0, 0, 0, 0, 65535, 0, 0, 0), 96);

/// [RFC 2765] Stateless IP/ICMP Translation Algorithm
/// (SIIT) (obsoleted by RFCs 6145, which itself was
/// later obsoleted by 7915).
///
/// **IPv6:**
///
/// 1. IPv4-mapped: `::ffff:0:0/96`
/// 2. IPv4-translated: `::ffff:0:0:0/96`
///
/// IPv4-compatible `::/96` is expressly excluded because RFC 2765 does not
/// use that address form.
///
/// [RFC 2765]: https://datatracker.ietf.org/doc/rfc2765/
pub const RFC2765: RFC = RFC {
  id: 2765,
  ip_nets: &[IpNet::V6(IPV6_1), IpNet::V6(IPV6_2)],
  ipv4_nets: &[],
  ipv6_nets: &[IPV6_1, IPV6_2],
};

#[test]
fn siit_ipv6_prefixes_match_rfc2765_section_2_1() {
  let expected = ["::ffff:0:0/96", "::ffff:0:0:0/96"];

  assert_eq!(RFC2765.ipv6_nets().len(), expected.len());
  assert_eq!(RFC2765.ip_nets().len(), expected.len());

  for (actual, expected) in RFC2765.ipv6_nets().iter().zip(expected) {
    let expected: Ipv6Net = expected.parse().unwrap();
    assert_eq!(*actual, expected);
  }

  for (actual, expected) in RFC2765.ip_nets().iter().zip(expected) {
    let expected: IpNet = expected.parse().unwrap();
    assert_eq!(*actual, expected);
  }
}

#[test]
fn siit_prefixes_include_start_and_end_but_not_neighbors() {
  for (net, start, end, before, after) in [
    (
      IPV6_1,
      Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0, 0),
      Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xffff, 0xffff),
      Ipv6Addr::new(0, 0, 0, 0, 0, 0xfffe, 0xffff, 0xffff),
      Ipv6Addr::new(0, 0, 0, 0, 1, 0, 0, 0),
    ),
    (
      IPV6_2,
      Ipv6Addr::new(0, 0, 0, 0, 0xffff, 0, 0, 0),
      Ipv6Addr::new(0, 0, 0, 0, 0xffff, 0, 0xffff, 0xffff),
      Ipv6Addr::new(0, 0, 0, 0, 0xfffe, 0xffff, 0xffff, 0xffff),
      Ipv6Addr::new(0, 0, 0, 1, 0, 0, 0, 0),
    ),
  ] {
    assert_eq!(net.network(), start);
    assert_eq!(net.broadcast(), end);
    assert!(RFC2765.contains(&start));
    assert!(RFC2765.contains(&end));
    assert!(!RFC2765.contains(&before));
    assert!(!RFC2765.contains(&after));

    let host = Ipv6Net::new(start, 128).unwrap();
    assert!(RFC2765.contains(&host));
    assert!(RFC2765.contains(&core::net::IpAddr::V6(start)));
    assert!(RFC2765.contains(&IpNet::V6(host)));
  }

  let compatible: Ipv6Net = "::/96".parse().unwrap();
  assert!(!RFC2765.contains(&compatible));
  assert!(!RFC2765.contains(&IpNet::V6(compatible)));
}
