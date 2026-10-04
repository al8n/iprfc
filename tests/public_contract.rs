use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use iprfc::{
  is_benchmark_ip_addr, is_benchmark_ipv4_addr, is_benchmark_ipv6_addr, is_broadcast_ip_addr,
  is_broadcast_ipv4_addr, is_documentation_ip_addr, is_documentation_ipv4_addr,
  is_documentation_ipv6_addr, is_link_local_ip_addr, is_link_local_ipv4_addr,
  is_link_local_ipv6_addr, is_loopback_ip_addr, is_loopback_ipv4_addr, is_loopback_ipv6_addr,
  is_multicast_ip_addr, is_multicast_ipv4_addr, is_multicast_ipv6_addr, is_private_ip_addr,
  is_private_ipv4_addr, is_shared_ip_addr, is_shared_ipv4_addr, is_unique_local_ipv6_addr,
  is_unspecified_ip_addr, is_unspecified_ipv4_addr, is_unspecified_ipv6_addr, Filter, IpNet,
  Ipv4Net, Ipv6Net, RFCs, Subset, FORWARDING_BLACKLIST_ID, RFC, RFC1918, RFC3849,
};

const EXPECTED_RFC_IDS: [u32; 30] = [
  919,
  1112,
  1122,
  1918,
  2544,
  2765,
  2928,
  3056,
  3068,
  3171,
  3330,
  3849,
  3927,
  4038,
  4193,
  4291,
  4380,
  4773,
  4843,
  5180,
  5735,
  5737,
  6052,
  6333,
  6598,
  6666,
  6890,
  7335,
  9637,
  FORWARDING_BLACKLIST_ID,
];

const UNKNOWN_BIT: u128 = 1u128 << 29;

const CONST_CLASSIFIERS: [bool; 25] = [
  is_loopback_ipv4_addr(Ipv4Addr::new(127, 0, 0, 1)),
  is_loopback_ipv6_addr(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
  is_loopback_ip_addr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
  is_private_ipv4_addr(Ipv4Addr::new(10, 0, 0, 1)),
  is_unique_local_ipv6_addr(Ipv6Addr::new(0xfc00, 0, 0, 0, 0, 0, 0, 1)),
  is_private_ip_addr(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))),
  is_link_local_ipv4_addr(Ipv4Addr::new(169, 254, 0, 1)),
  is_link_local_ipv6_addr(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)),
  is_link_local_ip_addr(IpAddr::V4(Ipv4Addr::new(169, 254, 0, 1))),
  is_documentation_ipv4_addr(Ipv4Addr::new(192, 0, 2, 1)),
  is_documentation_ipv6_addr(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
  is_documentation_ip_addr(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1))),
  is_benchmark_ipv4_addr(Ipv4Addr::new(198, 18, 0, 1)),
  is_benchmark_ipv6_addr(Ipv6Addr::new(0x2001, 2, 0, 0, 0, 0, 0, 1)),
  is_benchmark_ip_addr(IpAddr::V4(Ipv4Addr::new(198, 18, 0, 1))),
  is_shared_ipv4_addr(Ipv4Addr::new(100, 64, 0, 1)),
  is_shared_ip_addr(IpAddr::V4(Ipv4Addr::new(100, 64, 0, 1))),
  is_multicast_ipv4_addr(Ipv4Addr::new(224, 0, 0, 1)),
  is_multicast_ipv6_addr(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 1)),
  is_multicast_ip_addr(IpAddr::V4(Ipv4Addr::new(224, 0, 0, 1))),
  is_unspecified_ipv4_addr(Ipv4Addr::UNSPECIFIED),
  is_unspecified_ipv6_addr(Ipv6Addr::UNSPECIFIED),
  is_unspecified_ip_addr(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
  is_broadcast_ipv4_addr(Ipv4Addr::BROADCAST),
  is_broadcast_ip_addr(IpAddr::V4(Ipv4Addr::BROADCAST)),
];

const RFC_COUNT: usize = RFCs::len();
const RFC_1918_EXISTS: bool = RFCs::contains(1918);
const RFC_1918_LOOKUP: Option<&RFC> = RFCs::get(1918);
const RFC_1918_UNCHECKED: &RFC = RFCs::get_unchecked(1918);
const RFC_1918_ID: u32 = RFC_1918_UNCHECKED.id();
const RFC_1918_IP_NETS: &[IpNet] = RFC_1918_UNCHECKED.ip_nets();
const RFC_1918_IPV4_NETS: &[Ipv4Net] = RFC_1918_UNCHECKED.ipv4_nets();
const RFC_1918_IPV6_NETS: &[Ipv6Net] = RFC_1918_UNCHECKED.ipv6_nets();
const RFC_1918_SUBSET: Subset = RFCs::filter(Filter::RFC1918);

#[test]
fn iteration_order_and_lookups_are_stable() {
  assert_eq!(RFCs::len(), EXPECTED_RFC_IDS.len());

  for (&id, rfc) in EXPECTED_RFC_IDS.iter().zip(RFCs::iter()) {
    assert_eq!(rfc.id(), id);
    assert!(RFCs::contains(id));
    assert_eq!(RFCs::get(id), Some(rfc));
    assert_eq!(RFCs::get_unchecked(id), rfc);
    assert_eq!(RFCs[id], *rfc);

    let id_string = id.to_string();
    assert_eq!(RFCs[id_string.as_str()], *rfc);
  }
}

#[test]
fn filter_bits_keep_their_endpoints_and_unknown_bits() {
  assert_eq!(Filter::RFC919.bits(), 1);
  assert_eq!(Filter::RFC9637.bits(), 1u128 << 28);
  assert_eq!(Filter::FORWARDING_BLACKLIST.bits(), 1u128 << 127);

  let unknown = Filter::from_bits_retain(UNKNOWN_BIT);
  assert_eq!(unknown.bits(), UNKNOWN_BIT);

  let private = Ipv4Addr::new(10, 0, 0, 1);
  let named = RFCs::filter(Filter::RFC1918);
  let named_with_unknown = RFCs::filter(Filter::RFC1918 | unknown);
  assert_eq!(
    named.contains(&private),
    named_with_unknown.contains(&private)
  );
  assert!(!RFCs::filter(unknown).contains(&private));
}

#[test]
fn documented_const_api_is_callable_in_const_contexts() {
  assert!(CONST_CLASSIFIERS.iter().all(|&value| value));
  assert_eq!(RFC_COUNT, EXPECTED_RFC_IDS.len());
  assert_eq!(RFC_1918_EXISTS, RFCs::contains(RFC_1918_ID));
  assert_eq!(RFC_1918_LOOKUP, Some(RFC_1918_UNCHECKED));
  assert_eq!(RFC_1918_ID, 1918);
  assert!(!RFC_1918_IP_NETS.is_empty());
  assert!(!RFC_1918_IPV4_NETS.is_empty());
  assert!(RFC_1918_IPV6_NETS.is_empty());
  assert!(RFC_1918_SUBSET.contains(&Ipv4Addr::new(10, 0, 0, 1)));
}

#[test]
fn frozen_named_rfcs_accept_all_public_contains_inputs() {
  let ipv4_addr = Ipv4Addr::new(10, 0, 0, 1);
  let ipv6_addr = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
  let ipv4_net: Ipv4Net = "10.0.0.0/24".parse().unwrap();
  let ipv6_net: Ipv6Net = "2001:db8::/48".parse().unwrap();

  assert!(RFC1918.contains(&ipv4_addr));
  assert!(RFC3849.contains(&ipv6_addr));
  assert!(RFC1918.contains(&IpAddr::V4(ipv4_addr)));
  assert!(RFC1918.contains(&ipv4_net));
  assert!(RFC3849.contains(&ipv6_net));
  assert!(RFC1918.contains(&IpNet::V4(ipv4_net)));
}

#[cfg(feature = "serde")]
#[test]
fn readable_filter_serde_round_trips_named_and_unknown_bits() {
  use serde_test::{assert_tokens, Configure, Token};

  let named = Filter::RFC1918 | Filter::FORWARDING_BLACKLIST;
  assert_tokens(
    &named.readable(),
    &[
      Token::NewtypeStruct { name: "Filter" },
      Token::Str("RFC1918 | FORWARDING_BLACKLIST"),
    ],
  );

  let retained = Filter::from_bits_retain(named.bits() | UNKNOWN_BIT);
  assert_tokens(
    &retained.readable(),
    &[
      Token::NewtypeStruct { name: "Filter" },
      Token::Str("RFC1918 | FORWARDING_BLACKLIST | 0x20000000"),
    ],
  );
  assert_eq!(retained.bits(), named.bits() | UNKNOWN_BIT);

  let private = Ipv4Addr::new(10, 0, 0, 1);
  assert_eq!(
    RFCs::filter(retained).contains(&private),
    RFCs::filter(named).contains(&private)
  );
  assert!(!RFCs::filter(Filter::from_bits_retain(UNKNOWN_BIT)).contains(&private));
}
