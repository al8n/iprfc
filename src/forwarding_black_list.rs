use super::{
  iana_special_registry_data::{
    FORWARDING_BLACKLIST_IPV4_NETS, FORWARDING_BLACKLIST_IPV6_NETS, FORWARDING_BLACKLIST_IP_NETS,
  },
  RFC,
};

/// The id of the FORWARDING_BLACKLIST pseudo-RFC.
pub const FORWARDING_BLACKLIST_ID: u32 = u32::MAX;

/// Canonical forwarding blacklist derived from the checked-in IANA snapshots.
///
/// A destination is included only when its most-specific matching IANA
/// registry row has Forwardable exactly False. A more-specific True row is an
/// allow exception, and a blank or N/A Forwardable value is an explicit unknown
/// state that also overrides a less-specific False row. Deprecated and
/// terminated rows remain in the aggregate snapshot, but their blank
/// forwarding property does not become a guessed blacklist entry.
///
/// The generated CIDRs are canonical, sorted, and disjoint. A network query
/// succeeds only when one listed CIDR entirely contains the input network; it
/// does not assemble adjacent entries into a union. This is registry metadata,
/// not a firewall, SSRF defense, routing decision, or global-routability
/// guarantee.
///
/// See [IANA IPv4 Special-Purpose Address Space](https://www.iana.org/assignments/iana-ipv4-special-registry/)
/// and [IANA IPv6 Special-Purpose Address Space](https://www.iana.org/assignments/iana-ipv6-special-registry/).
pub const FORWARDING_BLACKLIST: RFC = RFC {
  id: FORWARDING_BLACKLIST_ID,
  ip_nets: FORWARDING_BLACKLIST_IP_NETS,
  ipv4_nets: FORWARDING_BLACKLIST_IPV4_NETS,
  ipv6_nets: FORWARDING_BLACKLIST_IPV6_NETS,
};

#[cfg(test)]
mod tests {
  use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

  use ipnet::{IpNet, Ipv4Net, Ipv6Net};

  use super::{FORWARDING_BLACKLIST, FORWARDING_BLACKLIST_ID};

  fn assert_ipv4(address: &str, expected: bool) {
    let address: Ipv4Addr = address.parse().unwrap();
    assert_eq!(FORWARDING_BLACKLIST.contains(&address), expected);
    assert_eq!(
      FORWARDING_BLACKLIST.contains(&IpAddr::V4(address)),
      expected
    );
  }

  fn assert_ipv6(address: &str, expected: bool) {
    let address: Ipv6Addr = address.parse().unwrap();
    assert_eq!(FORWARDING_BLACKLIST.contains(&address), expected);
    assert_eq!(
      FORWARDING_BLACKLIST.contains(&IpAddr::V6(address)),
      expected
    );
  }

  #[test]
  fn generated_blacklist_has_exact_counts_and_consistent_typed_views() {
    assert_eq!(FORWARDING_BLACKLIST.id(), FORWARDING_BLACKLIST_ID);
    assert_eq!(FORWARDING_BLACKLIST.ipv4_nets().len(), 14);
    assert_eq!(FORWARDING_BLACKLIST.ipv6_nets().len(), 139);
    assert_eq!(FORWARDING_BLACKLIST.ip_nets().len(), 153);

    for (index, net) in FORWARDING_BLACKLIST.ipv4_nets().iter().enumerate() {
      assert_eq!(FORWARDING_BLACKLIST.ip_nets()[index], IpNet::V4(*net));
      assert!(FORWARDING_BLACKLIST.contains(net));
      assert!(FORWARDING_BLACKLIST.contains(&IpNet::V4(*net)));
    }
    for (index, net) in FORWARDING_BLACKLIST.ipv6_nets().iter().enumerate() {
      let ip_index = FORWARDING_BLACKLIST.ipv4_nets().len() + index;
      assert_eq!(FORWARDING_BLACKLIST.ip_nets()[ip_index], IpNet::V6(*net));
      assert!(FORWARDING_BLACKLIST.contains(net));
      assert!(FORWARDING_BLACKLIST.contains(&IpNet::V6(*net)));
    }
  }

  #[test]
  fn current_registry_forwarding_exceptions_and_unknown_holes_match() {
    for (address, expected) in [
      ("192.0.0.8", true),
      ("192.0.0.9", false),
      ("192.0.0.10", false),
      ("192.0.0.11", true),
      ("192.88.99.0", false),
      ("192.88.99.2", false),
      ("198.18.0.1", false),
      ("2001:1::", true),
      ("2001:1::1", false),
      ("2001:1::2", false),
      ("2001:1::3", false),
      ("2001:1::4", true),
      ("2001::1", false),
      ("2001:2::1", false),
      ("2001:3::1", false),
      ("2001:4::1", true),
      ("2001:4:112::1", false),
      ("2001:10::1", false),
      ("2001:20::1", false),
      ("2001:30::1", false),
      ("2001:db8::1", true),
      ("3fff::1", true),
      ("5f00::1", false),
      ("100:0:0:1::1", true),
      ("fe80::1", true),
    ] {
      if address.contains(':') {
        assert_ipv6(address, expected);
      } else {
        assert_ipv4(address, expected);
      }
    }
  }

  #[test]
  fn network_queries_require_one_canonical_blacklist_cidr() {
    let blocked_ipv4: Ipv4Net = "192.0.0.8/32".parse().unwrap();
    let crossing_ipv4: Ipv4Net = "192.0.0.0/24".parse().unwrap();
    let blocked_ipv6: Ipv6Net = "2001:1::/128".parse().unwrap();
    let crossing_ipv6: Ipv6Net = "2001::/23".parse().unwrap();

    assert!(FORWARDING_BLACKLIST.contains(&blocked_ipv4));
    assert!(FORWARDING_BLACKLIST.contains(&IpNet::V4(blocked_ipv4)));
    assert!(!FORWARDING_BLACKLIST.contains(&crossing_ipv4));
    assert!(FORWARDING_BLACKLIST.contains(&blocked_ipv6));
    assert!(FORWARDING_BLACKLIST.contains(&IpNet::V6(blocked_ipv6)));
    assert!(!FORWARDING_BLACKLIST.contains(&crossing_ipv6));
  }
}
