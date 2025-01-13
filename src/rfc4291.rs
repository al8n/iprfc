use core::net::Ipv6Addr;

use ipnet::{IpNet, Ipv6Net};

use super::RFC;

/// ::/128
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::UNSPECIFIED, 128);

/// ::1/128
const IPV6_2: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::LOCALHOST, 128);

/// ::/96
const IPV6_3: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::UNSPECIFIED, 96);

/// ::ffff:0:0/96
const IPV6_4: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 0), 96);

/// fe80::/10
const IPV6_5: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0), 10);

/// fec0::/10
const IPV6_6: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0xfec0, 0, 0, 0, 0, 0, 0, 0), 10);

/// ff00::/8
const IPV6_7: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0), 8);

/// [RFC 4291] IP Version 6 Addressing Architecture
///
/// **Addresses:**
/// - **IPv6:**
///   1. `::/128`: §2.5.2 The Unspecified Address
///   2. `::1/128`: §2.5.3. The Loopback Address
///   3. `::/96`: §2.5.5.1.  IPv4-Compatible IPv6 Address
///   4. `::ffff:0:0/96`: §2.5.5.2.  IPv4-Mapped IPv6 Address
///   5. `fe80::/10`: §2.5.6 Link-Local IPv6 Unicast Addresses
///   6. `fec0::/10`: §2.5.7 Site-Local IPv6 Unicast Addresses (deprecated)
///   7. `ff00::/8`: §2.7. Multicast Addresses
///
/// [RFC 4291]: https://datatracker.ietf.org/doc/rfc4291/
pub const RFC4291: RFC = RFC {
  id: 4291,
  ip_addresses: &[
    IpNet::V6(IPV6_1),
    IpNet::V6(IPV6_2),
    IpNet::V6(IPV6_3),
    IpNet::V6(IPV6_4),
    IpNet::V6(IPV6_5),
    IpNet::V6(IPV6_6),
    IpNet::V6(IPV6_7),
  ],
  ipv4_addresses: &[],
  ipv6_addresses: &[IPV6_1, IPV6_2, IPV6_3, IPV6_4, IPV6_5, IPV6_6, IPV6_7],
};

#[test]
fn t() {
  for (i, addr) in [
    "::/128",
    "::1/128",
    "::/96",
    "::ffff:0:0/96",
    "fe80::/10",
    "fec0::/10",
    "ff00::/8",
  ].iter().enumerate() {
    let addr: Ipv6Net = addr.parse().unwrap();
    assert_eq!(RFC4291.ipv6_addresses[i], addr);
  }
}
