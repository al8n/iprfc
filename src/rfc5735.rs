use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(0, 0, 0, 0), 8);
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(10, 0, 0, 0), 8);
const IPV4_3: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(127, 0, 0, 0), 8);
const IPV4_4: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(169, 254, 0, 0), 16);
const IPV4_5: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(172, 16, 0, 0), 12);
const IPV4_6: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 0), 24);
const IPV4_7: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 2, 0), 24);
const IPV4_8: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 88, 99, 0), 24);
const IPV4_9: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 168, 0, 0), 16);
const IPV4_10: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 18, 0, 0), 15);
const IPV4_11: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 51, 100, 0), 24);
const IPV4_12: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(203, 0, 113, 0), 24);
const IPV4_13: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(224, 0, 0, 0), 4);
const IPV4_14: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(240, 0, 0, 0), 4);
const IPV4_15: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::BROADCAST, 32);

/// [RFC 5735] special-use IPv4 addresses.
///
/// **IPv4 blocks, in RFC 5735 section 4 order:**
///
/// 1. `0.0.0.0/8`: "This" network
/// 2. `10.0.0.0/8`: private-use networks
/// 3. `127.0.0.0/8`: loopback
/// 4. `169.254.0.0/16`: link local
/// 5. `172.16.0.0/12`: private-use networks
/// 6. `192.0.0.0/24`: IETF protocol assignments
/// 7. `192.0.2.0/24`: TEST-NET-1
/// 8. `192.88.99.0/24`: 6to4 relay anycast
/// 9. `192.168.0.0/16`: private-use networks
/// 10. `198.18.0.0/15`: network interconnect device benchmark testing
/// 11. `198.51.100.0/24`: TEST-NET-2
/// 12. `203.0.113.0/24`: TEST-NET-3
/// 13. `224.0.0.0/4`: multicast
/// 14. `240.0.0.0/4`: reserved for future use
/// 15. `255.255.255.255/32`: limited broadcast
///
/// [RFC 5735]: https://www.rfc-editor.org/rfc/rfc5735#section-4
pub const RFC5735: RFC = RFC {
  id: 5735,
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
  ],
  ipv4_nets: &[
    IPV4_1, IPV4_2, IPV4_3, IPV4_4, IPV4_5, IPV4_6, IPV4_7, IPV4_8, IPV4_9, IPV4_10, IPV4_11,
    IPV4_12, IPV4_13, IPV4_14, IPV4_15,
  ],
  ipv6_nets: &[],
};

#[test]
fn special_use_ipv4_table_matches_rfc5735_section_4() {
  let expected = [
    "0.0.0.0/8",
    "10.0.0.0/8",
    "127.0.0.0/8",
    "169.254.0.0/16",
    "172.16.0.0/12",
    "192.0.0.0/24",
    "192.0.2.0/24",
    "192.88.99.0/24",
    "192.168.0.0/16",
    "198.18.0.0/15",
    "198.51.100.0/24",
    "203.0.113.0/24",
    "224.0.0.0/4",
    "240.0.0.0/4",
    "255.255.255.255/32",
  ];

  assert_eq!(RFC5735.ipv4_nets().len(), expected.len());
  assert_eq!(RFC5735.ip_nets().len(), expected.len());

  for (actual, expected) in RFC5735.ipv4_nets().iter().zip(expected) {
    let expected: Ipv4Net = expected.parse().unwrap();
    assert_eq!(*actual, expected);
  }

  for (actual, expected) in RFC5735.ip_nets().iter().zip(expected) {
    let expected: IpNet = expected.parse().unwrap();
    assert_eq!(*actual, expected);
  }
}

#[test]
fn special_use_ipv4_addresses_and_network_boundaries_are_contained() {
  for net in RFC5735.ipv4_nets() {
    assert!(RFC5735.contains(&net.network()));
    assert!(RFC5735.contains(&net.broadcast()));
    assert!(RFC5735.contains(net));
  }

  let contained: Ipv4Net = "198.18.0.0/16".parse().unwrap();
  let overlapping: Ipv4Net = "198.16.0.0/14".parse().unwrap();
  assert!(RFC5735.contains(&contained));
  assert!(!RFC5735.contains(&overlapping));

  for address in [
    Ipv4Addr::new(9, 255, 255, 255),
    Ipv4Addr::new(11, 0, 0, 0),
    Ipv4Addr::new(169, 253, 255, 255),
    Ipv4Addr::new(169, 255, 0, 0),
    Ipv4Addr::new(223, 255, 255, 255),
  ] {
    assert!(!RFC5735.contains(&address));
  }
}
