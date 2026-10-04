use core::net::{Ipv4Addr, Ipv6Addr};

#[cfg(test)]
use core::net::IpAddr;

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

use super::RFC;

const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(0, 0, 0, 0), 8);
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(127, 0, 0, 0), 8);
const IPV4_3: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(169, 254, 0, 0), 16);
const IPV4_4: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 8), 29);
const IPV4_5: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 16), 28);
const IPV4_6: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 32), 27);
const IPV4_7: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 64), 26);
const IPV4_8: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 128), 25);
const IPV4_9: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 2, 0), 24);
const IPV4_10: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 51, 100, 0), 24);
const IPV4_11: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(203, 0, 113, 0), 24);
const IPV4_12: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(240, 0, 0, 0), 4);

const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::UNSPECIFIED, 128);
const IPV6_2: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::LOCALHOST, 128);
const IPV6_3: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 0), 96);
const IPV6_4: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0001, 0, 0, 0, 0, 0, 0), 32);
const IPV6_5: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0001, 0, 0, 0, 0, 0), 48);
const IPV6_6: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0002, 0, 0, 0, 0, 0), 47);
const IPV6_7: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0004, 0, 0, 0, 0, 0), 46);
const IPV6_8: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0008, 0, 0, 0, 0, 0), 45);
const IPV6_9: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0010, 0, 0, 0, 0, 0), 44);
const IPV6_10: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0020, 0, 0, 0, 0, 0), 43);
const IPV6_11: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0040, 0, 0, 0, 0, 0), 42);
const IPV6_12: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0080, 0, 0, 0, 0, 0), 41);
const IPV6_13: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0100, 0, 0, 0, 0, 0), 40);
const IPV6_14: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0200, 0, 0, 0, 0, 0), 39);
const IPV6_15: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0400, 0, 0, 0, 0, 0), 38);
const IPV6_16: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x0800, 0, 0, 0, 0, 0), 37);
const IPV6_17: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x1000, 0, 0, 0, 0, 0), 36);
const IPV6_18: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x2000, 0, 0, 0, 0, 0), 35);
const IPV6_19: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x4000, 0, 0, 0, 0, 0), 34);
const IPV6_20: Ipv6Net =
  Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0002, 0x8000, 0, 0, 0, 0, 0), 33);
const IPV6_21: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0003, 0, 0, 0, 0, 0, 0), 32);
const IPV6_22: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0004, 0, 0, 0, 0, 0, 0), 30);
const IPV6_23: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0008, 0, 0, 0, 0, 0, 0), 29);
const IPV6_24: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0010, 0, 0, 0, 0, 0, 0), 28);
const IPV6_25: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0020, 0, 0, 0, 0, 0, 0), 27);
const IPV6_26: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0040, 0, 0, 0, 0, 0, 0), 26);
const IPV6_27: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0080, 0, 0, 0, 0, 0, 0), 25);
const IPV6_28: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0100, 0, 0, 0, 0, 0, 0), 24);
const IPV6_29: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 0), 32);
const IPV6_30: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0), 10);

/// The id of the [`FORWARDING_BLACKLIST`] pseudo-RFC.
pub const FORWARDING_BLACKLIST_ID: u32 = u32::MAX;

/// Historical RFC 6890 forwarding blacklist.
///
/// This pseudo-RFC records destinations whose `Forwardable` field was false
/// in RFC 6890's April 2013 registry snapshot, after applying that table's
/// more-specific forwardable allocations. It is not the current IANA
/// registry, a firewall or SSRF policy, or a statement about global
/// routability. In particular, `192.0.0.0/29` is exempt while
/// `192.0.0.8` through `192.0.0.255` remain historically blacklisted;
/// RFC 9637's later `3fff::/20` is intentionally absent.
///
/// The slices use canonical, disjoint CIDRs so the more-specific exemptions
/// need no runtime special case. When queried with a network,
/// [`RFC::contains`] remains true only when one listed CIDR entirely contains
/// that network; it does not combine adjacent CIDRs into a union.
///
/// [RFC 6890]: https://www.rfc-editor.org/rfc/rfc6890.html
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
    IpNet::V4(IPV4_10),
    IpNet::V4(IPV4_11),
    IpNet::V4(IPV4_12),
    IpNet::V6(IPV6_1),
    IpNet::V6(IPV6_2),
    IpNet::V6(IPV6_3),
    IpNet::V6(IPV6_4),
    IpNet::V6(IPV6_5),
    IpNet::V6(IPV6_6),
    IpNet::V6(IPV6_7),
    IpNet::V6(IPV6_8),
    IpNet::V6(IPV6_9),
    IpNet::V6(IPV6_10),
    IpNet::V6(IPV6_11),
    IpNet::V6(IPV6_12),
    IpNet::V6(IPV6_13),
    IpNet::V6(IPV6_14),
    IpNet::V6(IPV6_15),
    IpNet::V6(IPV6_16),
    IpNet::V6(IPV6_17),
    IpNet::V6(IPV6_18),
    IpNet::V6(IPV6_19),
    IpNet::V6(IPV6_20),
    IpNet::V6(IPV6_21),
    IpNet::V6(IPV6_22),
    IpNet::V6(IPV6_23),
    IpNet::V6(IPV6_24),
    IpNet::V6(IPV6_25),
    IpNet::V6(IPV6_26),
    IpNet::V6(IPV6_27),
    IpNet::V6(IPV6_28),
    IpNet::V6(IPV6_29),
    IpNet::V6(IPV6_30),
  ],
  ipv4_nets: &[
    IPV4_1, IPV4_2, IPV4_3, IPV4_4, IPV4_5, IPV4_6, IPV4_7, IPV4_8, IPV4_9, IPV4_10, IPV4_11,
    IPV4_12,
  ],
  ipv6_nets: &[
    IPV6_1, IPV6_2, IPV6_3, IPV6_4, IPV6_5, IPV6_6, IPV6_7, IPV6_8, IPV6_9, IPV6_10, IPV6_11,
    IPV6_12, IPV6_13, IPV6_14, IPV6_15, IPV6_16, IPV6_17, IPV6_18, IPV6_19, IPV6_20, IPV6_21,
    IPV6_22, IPV6_23, IPV6_24, IPV6_25, IPV6_26, IPV6_27, IPV6_28, IPV6_29, IPV6_30,
  ],
};

#[cfg(test)]
fn rfc6890_forwardable(ip: IpAddr) -> Option<bool> {
  // The RFC 6890 table is intentionally modeled independently of the
  // canonical blacklist table above. The most-specific matching row wins.
  let mut selected: Option<(u8, bool)> = None;

  for (cidr, forwardable) in [
    ("0.0.0.0/8", false),
    ("10.0.0.0/8", true),
    ("100.64.0.0/10", true),
    ("127.0.0.0/8", false),
    ("169.254.0.0/16", false),
    ("172.16.0.0/12", true),
    ("192.0.0.0/24", false),
    ("192.0.0.0/29", true),
    ("192.0.2.0/24", false),
    ("192.88.99.0/24", true),
    ("192.168.0.0/16", true),
    ("198.18.0.0/15", true),
    ("198.51.100.0/24", false),
    ("203.0.113.0/24", false),
    ("240.0.0.0/4", false),
    ("255.255.255.255/32", false),
    ("::1/128", false),
    ("::/128", false),
    ("64:ff9b::/96", true),
    ("::ffff:0:0/96", false),
    ("100::/64", true),
    ("2001::/23", false),
    ("2001::/32", true),
    ("2001:2::/48", true),
    ("2001:db8::/32", false),
    ("2001:10::/28", false),
    ("2002::/16", true),
    ("fc00::/7", true),
    ("fe80::/10", false),
  ] {
    let net: IpNet = cidr.parse().unwrap();
    if net.contains(&ip) {
      let prefix_len = net.prefix_len();
      match selected {
        None => selected = Some((prefix_len, forwardable)),
        Some((selected_prefix_len, _)) if prefix_len > selected_prefix_len => {
          selected = Some((prefix_len, forwardable));
        }
        _ => {}
      }
    }
  }

  selected.map(|(_, forwardable)| forwardable)
}

#[cfg(test)]
fn assert_address_matches_oracle(s: &str) {
  let ip: IpAddr = s.parse().unwrap();
  let expected = matches!(rfc6890_forwardable(ip), Some(false));

  assert_eq!(FORWARDING_BLACKLIST.contains(&ip), expected, "enum {s}");
  match ip {
    IpAddr::V4(ip) => assert_eq!(FORWARDING_BLACKLIST.contains(&ip), expected, "IPv4 {s}"),
    IpAddr::V6(ip) => assert_eq!(FORWARDING_BLACKLIST.contains(&ip), expected, "IPv6 {s}"),
  }
}

#[cfg(test)]
fn assert_net_membership(s: &str, expected: bool) {
  let net: IpNet = s.parse().unwrap();
  assert_eq!(FORWARDING_BLACKLIST.contains(&net), expected, "enum {s}");
  match net {
    IpNet::V4(net) => assert_eq!(FORWARDING_BLACKLIST.contains(&net), expected, "IPv4 {s}"),
    IpNet::V6(net) => assert_eq!(FORWARDING_BLACKLIST.contains(&net), expected, "IPv6 {s}"),
  }
}

#[cfg(test)]
fn assert_canonical_and_disjoint_ipv4(nets: &[Ipv4Net]) {
  for (index, net) in nets.iter().enumerate() {
    assert_eq!(net.addr(), net.network(), "{net} is not canonical");
    if let Some(next) = nets.get(index + 1) {
      assert!(net.network() < next.network(), "{net} is out of order");
    }
    for later in &nets[index + 1..] {
      assert!(
        !net.contains(&later.network()) && !later.contains(&net.network()),
        "{net} overlaps {later}"
      );
    }
  }
}

#[cfg(test)]
fn assert_canonical_and_disjoint_ipv6(nets: &[Ipv6Net]) {
  for (index, net) in nets.iter().enumerate() {
    assert_eq!(net.addr(), net.network(), "{net} is not canonical");
    if let Some(next) = nets.get(index + 1) {
      assert!(net.network() < next.network(), "{net} is out of order");
    }
    for later in &nets[index + 1..] {
      assert!(
        !net.contains(&later.network()) && !later.contains(&net.network()),
        "{net} overlaps {later}"
      );
    }
  }
}

#[test]
fn canonical_slices_are_exact_and_disjoint() {
  let ipv4 = [
    "0.0.0.0/8",
    "127.0.0.0/8",
    "169.254.0.0/16",
    "192.0.0.8/29",
    "192.0.0.16/28",
    "192.0.0.32/27",
    "192.0.0.64/26",
    "192.0.0.128/25",
    "192.0.2.0/24",
    "198.51.100.0/24",
    "203.0.113.0/24",
    "240.0.0.0/4",
  ];
  let ipv6 = [
    "::/128",
    "::1/128",
    "::ffff:0:0/96",
    "2001:1::/32",
    "2001:2:1::/48",
    "2001:2:2::/47",
    "2001:2:4::/46",
    "2001:2:8::/45",
    "2001:2:10::/44",
    "2001:2:20::/43",
    "2001:2:40::/42",
    "2001:2:80::/41",
    "2001:2:100::/40",
    "2001:2:200::/39",
    "2001:2:400::/38",
    "2001:2:800::/37",
    "2001:2:1000::/36",
    "2001:2:2000::/35",
    "2001:2:4000::/34",
    "2001:2:8000::/33",
    "2001:3::/32",
    "2001:4::/30",
    "2001:8::/29",
    "2001:10::/28",
    "2001:20::/27",
    "2001:40::/26",
    "2001:80::/25",
    "2001:100::/24",
    "2001:db8::/32",
    "fe80::/10",
  ];

  assert_eq!(FORWARDING_BLACKLIST.id(), FORWARDING_BLACKLIST_ID);
  assert_eq!(FORWARDING_BLACKLIST.ipv4_nets().len(), 12);
  assert_eq!(FORWARDING_BLACKLIST.ipv6_nets().len(), 30);
  assert_eq!(FORWARDING_BLACKLIST.ip_nets().len(), 42);

  for (index, cidr) in ipv4.iter().enumerate() {
    let net: Ipv4Net = cidr.parse().unwrap();
    assert_eq!(FORWARDING_BLACKLIST.ipv4_nets()[index], net, "{cidr}");
    assert_eq!(
      FORWARDING_BLACKLIST.ip_nets()[index],
      IpNet::V4(net),
      "{cidr}"
    );
  }
  for (index, cidr) in ipv6.iter().enumerate() {
    let net: Ipv6Net = cidr.parse().unwrap();
    assert_eq!(FORWARDING_BLACKLIST.ipv6_nets()[index], net, "{cidr}");
    assert_eq!(
      FORWARDING_BLACKLIST.ip_nets()[ipv4.len() + index],
      IpNet::V6(net),
      "{cidr}"
    );
  }

  assert_canonical_and_disjoint_ipv4(FORWARDING_BLACKLIST.ipv4_nets());
  assert_canonical_and_disjoint_ipv6(FORWARDING_BLACKLIST.ipv6_nets());
}

#[test]
fn membership_matches_rfc6890_longest_prefix_oracle() {
  for s in [
    "0.0.0.0",
    "0.255.255.255",
    "1.0.0.0",
    "10.0.0.1",
    "100.64.0.1",
    "172.16.0.1",
    "126.255.255.255",
    "127.0.0.0",
    "127.255.255.255",
    "128.0.0.0",
    "169.253.255.255",
    "169.254.0.0",
    "169.254.255.255",
    "169.255.0.0",
    "192.0.0.0",
    "192.0.0.7",
    "192.0.0.8",
    "192.0.0.9",
    "192.0.0.10",
    "192.0.0.15",
    "192.0.0.16",
    "192.0.0.255",
    "192.0.1.0",
    "192.0.1.255",
    "192.0.2.0",
    "192.0.2.255",
    "192.0.3.0",
    "192.168.1.1",
    "198.18.0.0",
    "198.51.100.0",
    "198.51.100.255",
    "203.0.113.0",
    "203.0.113.255",
    "239.255.255.255",
    "240.0.0.0",
    "255.255.255.255",
    "::",
    "::1",
    "::2",
    "::ffff:192.0.2.1",
    "::fffe:ffff:ffff",
    "100::1",
    "2001::1",
    "2001:1::",
    "2001:1:ffff:ffff:ffff:ffff:ffff:ffff",
    "2001:2::1",
    "2001:2:0:ffff:ffff:ffff:ffff:ffff",
    "2001:2:1::",
    "2001:10::1",
    "2001:100::1",
    "2001:1ff:ffff:ffff:ffff:ffff:ffff:ffff",
    "2001:200::",
    "2001:db7:ffff::",
    "2001:db8::",
    "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff",
    "2001:db9::",
    "3fff::1",
    "fc00::",
    "fe7f:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
    "fe80::",
    "febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
    "fec0::",
  ] {
    assert_address_matches_oracle(s);
  }
}

#[test]
fn network_containment_requires_one_canonical_cidr() {
  assert_net_membership("192.0.0.8/32", true);
  assert_net_membership("2001:2:1::/64", true);

  // Both supernets cross a more-specific forwardable exception, so they are
  // not contained by one listed false-forwardable CIDR.
  assert_net_membership("192.0.0.0/24", false);
  assert_net_membership("2001::/23", false);
}
