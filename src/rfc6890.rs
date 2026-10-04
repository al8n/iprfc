use super::{
  iana_special_registry_data::{
    RFC6890_IP_NETS, RFC6890_IPV4_NETS, RFC6890_IPV6_NETS,
  },
  RFC,
};

/// Compatibility aggregate of the checked-in IANA Special-Purpose Address
/// Space registries.
///
/// RFC 6890 remains a historical document, but this compatibility constant is
/// intentionally a version-pinned view of the continuously maintained IANA
/// IPv4 and IPv6 registries. It includes every prefix in the checked-in source
/// rows, including deprecated or terminated rows. A match is registry
/// membership only; it does not establish routing, firewall, SSRF, or global
/// reachability policy.
///
/// The checked-in XML snapshots, source URLs, update dates, and SHA-256 hashes
/// are recorded in the generated private data module and data/iana/README.md.
/// Maintainers refresh them deliberately before release with the developer
/// updater; crate builds and runtime remain offline.
///
/// IANA registry pages:
///
/// - [IPv4 Special-Purpose Address Space](https://www.iana.org/assignments/iana-ipv4-special-registry/)
/// - [IPv6 Special-Purpose Address Space](https://www.iana.org/assignments/iana-ipv6-special-registry/)
pub const RFC6890: RFC = RFC {
  id: 6890,
  ip_nets: RFC6890_IP_NETS,
  ipv4_nets: RFC6890_IPV4_NETS,
  ipv6_nets: RFC6890_IPV6_NETS,
};

#[cfg(test)]
mod tests {
  use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

  use ipnet::{IpNet, Ipv4Net, Ipv6Net};

  use super::RFC6890;

  fn assert_ipv4_prefix(cidr: &str) {
    let net: Ipv4Net = cidr.parse().unwrap();
    assert!(RFC6890.contains(&net), "{cidr}");
    assert!(RFC6890.contains(&IpNet::V4(net)), "{cidr}");
    assert!(RFC6890.contains(&net.network()), "{cidr}");
    assert!(RFC6890.contains(&IpAddr::V4(net.network())), "{cidr}");
  }

  fn assert_ipv6_prefix(cidr: &str) {
    let net: Ipv6Net = cidr.parse().unwrap();
    assert!(RFC6890.contains(&net), "{cidr}");
    assert!(RFC6890.contains(&IpNet::V6(net)), "{cidr}");
    assert!(RFC6890.contains(&net.network()), "{cidr}");
    assert!(RFC6890.contains(&IpAddr::V6(net.network())), "{cidr}");
  }

  #[test]
  fn current_iana_snapshot_has_exact_family_counts_and_consistent_views() {
    assert_eq!(RFC6890.id(), 6890);
    assert_eq!(RFC6890.ipv4_nets().len(), 26);
    assert_eq!(RFC6890.ipv6_nets().len(), 25);
    assert_eq!(RFC6890.ip_nets().len(), 51);

    for (index, net) in RFC6890.ipv4_nets().iter().enumerate() {
      assert_eq!(RFC6890.ip_nets()[index], IpNet::V4(*net));
      assert!(RFC6890.contains(net));
      assert!(RFC6890.contains(&IpNet::V4(*net)));
    }
    for (index, net) in RFC6890.ipv6_nets().iter().enumerate() {
      let ip_index = RFC6890.ipv4_nets().len() + index;
      assert_eq!(RFC6890.ip_nets()[ip_index], IpNet::V6(*net));
      assert!(RFC6890.contains(net));
      assert!(RFC6890.contains(&IpNet::V6(*net)));
    }
  }

  #[test]
  fn newer_iana_entries_and_terminated_rows_are_present() {
    for cidr in [
      "192.0.0.170/32",
      "192.0.0.171/32",
      "192.31.196.0/24",
      "192.52.193.0/24",
      "192.175.48.0/24",
      "192.88.99.0/24",
    ] {
      assert_ipv4_prefix(cidr);
    }

    for cidr in [
      "64:ff9b:1::/48",
      "100:0:0:1::/64",
      "2001:1::1/128",
      "2001:1::2/128",
      "2001:1::3/128",
      "2001:3::/32",
      "2001:4:112::/48",
      "2001:10::/28",
      "2001:20::/28",
      "2001:30::/28",
      "2620:4f:8000::/48",
      "3fff::/20",
      "5f00::/16",
    ] {
      assert_ipv6_prefix(cidr);
    }
  }

  #[test]
  fn aggregate_does_not_overreach_unlisted_global_space() {
    for address in [
      Ipv4Addr::new(8, 8, 8, 8),
      Ipv4Addr::new(1, 1, 1, 1),
    ] {
      assert!(!RFC6890.contains(&address));
      assert!(!RFC6890.contains(&IpAddr::V4(address)));
    }
    for address in [
      "2001:4860:4860::8888".parse::<Ipv6Addr>().unwrap(),
      "2606:4700:4700::1111".parse::<Ipv6Addr>().unwrap(),
    ] {
      assert!(!RFC6890.contains(&address));
      assert!(!RFC6890.contains(&IpAddr::V6(address)));
    }
  }
}
