use core::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(0, 0, 0, 0), 8);
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(127, 0, 0, 0), 8);
const IPV4_3: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(169, 254, 0, 0), 16);
const IPV4_4: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 0), 24);
const IPV4_5: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 2, 0), 24);
const IPV4_6: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 51, 100, 0), 24);
const IPV4_7: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(203, 0, 113, 0), 24);
const IPV4_8: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(240, 0, 0, 0), 4);
const IPV4_9: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::BROADCAST, 32);

const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::UNSPECIFIED, 128);
const IPV6_2: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::LOCALHOST, 128);
/// ::ffff:0:0/96
const IPV6_3: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 0), 96);
/// 2001:db8::/32
const IPV6_4: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0), 32);
/// 2001:10::/28
const IPV6_5: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x10, 0, 0, 0, 0, 0, 0), 28);
/// fe80::/10
const IPV6_6: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0), 10);

/// The id of the [`FORWARDING_BLACKLIST`] RFC.
pub const FORWARDING_BLACKLIST_ID: u32 = u32::MAX;

/// Pseudo-RFC: Blacklist of non-forwardable IP blocks taken from [RFC 6890]
///
/// [RFC 6890]: https://datatracker.ietf.org/doc/rfc6890/
pub const FORWARDING_BLACKLIST: RFC = RFC {
  id: FORWARDING_BLACKLIST_ID,
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
    IpNet::V6(IPV6_1),
    IpNet::V6(IPV6_2),
    IpNet::V6(IPV6_3),
    IpNet::V6(IPV6_4),
    IpNet::V6(IPV6_5),
    IpNet::V6(IPV6_6),
  ],
  ipv4_nets: &[
    IPV4_1, IPV4_2, IPV4_3, IPV4_4, IPV4_5, IPV4_6, IPV4_7, IPV4_8, IPV4_9,
  ],
  ipv6_nets: &[IPV6_1, IPV6_2, IPV6_3, IPV6_4, IPV6_5, IPV6_6],
};

#[test]
fn t() {
  for (idx, s) in [
    "0.0.0.0/8",
    "127.0.0.0/8",
    "169.254.0.0/16",
    "192.0.0.0/24",
    "192.0.2.0/24",
    "198.51.100.0/24",
    "203.0.113.0/24",
    "240.0.0.0/4",
    "255.255.255.255/32",
  ]
  .iter()
  .enumerate()
  {
    let addr: Ipv4Net = s.parse().unwrap();
    assert_eq!(FORWARDING_BLACKLIST.ipv4_nets[idx], addr);
  }

  for (idx, s) in [
    "::/128",
    "::1/128",
    "::ffff:0:0/96",
    "2001:db8::/32",
    "2001:10::/28",
    "fe80::/10",
  ]
  .iter()
  .enumerate()
  {
    let addr: Ipv6Net = s.parse().unwrap();
    assert_eq!(FORWARDING_BLACKLIST.ipv6_nets[idx], addr);
  }
}
