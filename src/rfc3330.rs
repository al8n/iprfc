use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

/// 0.0.0.0/8
const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(0, 0, 0, 0), 8);
/// 10.0.0.0/8
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(10, 0, 0, 0), 8);
/// 14.0.0.0/8
const IPV4_3: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(14, 0, 0, 0), 8);
/// 24.0.0.0/8
const IPV4_4: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(24, 0, 0, 0), 8);
/// 39.0.0.0/8
const IPV4_5: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(39, 0, 0, 0), 8);
/// 127.0.0.0/8
const IPV4_6: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(127, 0, 0, 0), 8);
/// 128.0.0.0/16
const IPV4_7: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(128, 0, 0, 0), 16);
/// 169.254.0.0/16
const IPV4_8: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(169, 254, 0, 0), 16);
/// 172.16.0.0/12
const IPV4_9: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(172, 16, 0, 0), 12);
/// 191.255.0.0/16
const IPV4_10: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(191, 255, 0, 0), 16);
/// 192.0.0.0/24
const IPV4_11: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 0), 24);
/// 192.0.2.0/24
const IPV4_12: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 2, 0), 24);
/// 192.88.99.0/24
const IPV4_13: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 88, 99, 0), 24);
/// 192.168.0.0/16
const IPV4_14: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 168, 0, 0), 16);
/// 198.18.0.0/15
const IPV4_15: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 18, 0, 0), 15);
/// 223.255.255.0/24
const IPV4_16: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(223, 255, 255, 0), 24);
/// 224.0.0.0/4
const IPV4_17: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(224, 0, 0, 0), 4);
/// 240.0.0.0/4
const IPV4_18: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(240, 0, 0, 0), 4);

/// [RFC 3330] Special-Use IPv4 Addresses.
///
/// This value preserves RFC 3330's September 2002 Section 3 summary table,
/// including rows that the document said were already subject to allocation.
/// RFC 3330 was obsoleted by [RFC 5735]; this historical description does not
/// assert that a block is currently special-use or otherwise unsuitable for a
/// caller's policy.
///
/// The 18 networks are in the RFC's table order.
///
/// [RFC 3330]: https://www.rfc-editor.org/rfc/rfc3330.html
/// [RFC 5735]: https://www.rfc-editor.org/rfc/rfc5735.html
pub const RFC3330: RFC = RFC {
  id: 3330,
  ip_nets: &[
    IpNet::V4(IPV4_1),
    IpNet::V4(IPV4_2),
    IpNet::V4(IPV4_3),
    IpNet::V4(IPV4_4),
    IpNet::V4(IPV4_5),
    IpNet::V4(IPV4_6),
    IpNet::V4(IPV4_7),
    IpNet::V4(IPV4_8),
    IpNet::V4(IPV4_9),
    IpNet::V4(IPV4_10),
    IpNet::V4(IPV4_11),
    IpNet::V4(IPV4_12),
    IpNet::V4(IPV4_13),
    IpNet::V4(IPV4_14),
    IpNet::V4(IPV4_15),
    IpNet::V4(IPV4_16),
    IpNet::V4(IPV4_17),
    IpNet::V4(IPV4_18),
  ],
  ipv4_nets: &[
    IPV4_1, IPV4_2, IPV4_3, IPV4_4, IPV4_5, IPV4_6, IPV4_7, IPV4_8, IPV4_9, IPV4_10, IPV4_11,
    IPV4_12, IPV4_13, IPV4_14, IPV4_15, IPV4_16, IPV4_17, IPV4_18,
  ],
  ipv6_nets: &[],
};

#[test]
fn summary_table_is_complete_and_in_document_order() {
  let expected = [
    "0.0.0.0/8",
    "10.0.0.0/8",
    "14.0.0.0/8",
    "24.0.0.0/8",
    "39.0.0.0/8",
    "127.0.0.0/8",
    "128.0.0.0/16",
    "169.254.0.0/16",
    "172.16.0.0/12",
    "191.255.0.0/16",
    "192.0.0.0/24",
    "192.0.2.0/24",
    "192.88.99.0/24",
    "192.168.0.0/16",
    "198.18.0.0/15",
    "223.255.255.0/24",
    "224.0.0.0/4",
    "240.0.0.0/4",
  ];

  assert_eq!(RFC3330.ipv4_nets().len(), 18);
  assert_eq!(RFC3330.ipv6_nets().len(), 0);
  assert_eq!(RFC3330.ip_nets().len(), 18);

  for (index, cidr) in expected.iter().enumerate() {
    let net: Ipv4Net = cidr.parse().unwrap();
    assert_eq!(RFC3330.ipv4_nets()[index], net, "{cidr}");
    assert_eq!(RFC3330.ip_nets()[index], IpNet::V4(net), "{cidr}");
    assert!(RFC3330.contains(&net.network()), "first address of {cidr}");
    assert!(RFC3330.contains(&net.broadcast()), "last address of {cidr}");
  }
}

#[test]
fn summary_table_boundaries_and_historical_rows_are_preserved() {
  for s in [
    "13.255.255.255",
    "15.0.0.0",
    "23.255.255.255",
    "25.0.0.0",
    "38.255.255.255",
    "40.0.0.0",
    "126.255.255.255",
    "128.1.0.0",
    "169.253.255.255",
    "169.255.0.0",
    "172.15.255.255",
    "172.32.0.0",
    "191.254.255.255",
    "192.0.1.0",
    "192.0.3.0",
    "192.88.98.255",
    "192.88.100.0",
    "192.167.255.255",
    "192.169.0.0",
    "198.17.255.255",
    "198.20.0.0",
    "223.255.254.255",
  ] {
    let ip: Ipv4Addr = s.parse().unwrap();
    assert!(!RFC3330.contains(&ip), "{s} is outside every RFC3330 row");
  }

  for s in [
    "14.0.0.0",
    "24.0.0.0",
    "39.0.0.0",
    "128.0.0.0",
    "191.255.0.0",
    "223.255.255.0",
  ] {
    let ip: Ipv4Addr = s.parse().unwrap();
    assert!(RFC3330.contains(&ip), "RFC3330 includes historical {s}");
    assert!(
      !super::RFC5735.contains(&ip),
      "RFC5735 omits historical {s}"
    );
  }
}
