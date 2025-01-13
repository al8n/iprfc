use core::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

use super::RFC;

/// 0.0.0.0/8
const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(0, 0, 0, 0), 8);

/// 10.0.0.0/8
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(10, 0, 0, 0), 8);

/// 100.64.0.1/10
const IPV4_3: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(100, 64, 0, 0), 10);

/// 127.0.0.0/8
const IPV4_4: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(127, 0, 0, 0), 8);

/// 169.254.0.0/16
const IPV4_5: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(169, 254, 0, 0), 16);

/// 172.16.0.0/12
const IPV4_6: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(172, 16, 0, 0), 12);

/// 192.0.0.0/24
const IPV4_7: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 0), 24);

/// 192.0.0.0/29
const IPV4_8: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 0, 0), 29);

/// 192.0.2.0/24
const IPV4_9: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 2, 0), 24);

/// 192.88.99.0/24
const IPV4_10: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 88, 99, 0), 24);

/// 192.168.0.0/16
const IPV4_11: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 168, 0, 0), 16);

/// 192.18.0.0/15
const IPV4_12: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 18, 0, 0), 15);

/// 198.51.100.0/24
const IPV4_13: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 51, 100, 0), 24);

/// 203.0.113.0/24
const IPV4_14: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(203, 0, 113, 0), 24);

/// 240.0.0.0/4
const IPV4_15: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(240, 0, 0, 0), 4);

/// 255.255.255.255/32
const IPV4_16: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::BROADCAST, 32);

/// ::/128
const IPV6_1: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::UNSPECIFIED, 128);

/// ::1/128
const IPV6_2: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::LOCALHOST, 128);

/// 64:ff9b::/96
const IPV6_3: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x64, 0xff9b, 0, 0, 0, 0, 0, 0), 96);

/// ::ffff:0:0/96
const IPV6_4: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 0), 96);

/// 100::/64
const IPV6_5: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x0100, 0, 0, 0, 0, 0, 0, 0), 64);

/// 2001::/16
const IPV6_6: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2001, 0, 0, 0, 0, 0, 0, 0), 16);

/// 2002::/16
const IPV6_7: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0x2002, 0, 0, 0, 0, 0, 0, 0), 16);

/// fc00::/7
const IPV6_8: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0xfc00, 0, 0, 0, 0, 0, 0, 0), 7);

/// fe80::/10
const IPV6_9: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0), 10);


/// [RFC 6890] Special-Purpose IP Address Registries
///
/// 
/// From "RFC6890 §2.2.1 Information Requirements":
///
/// The IPv4 and IPv6 Special-Purpose Address Registries maintain the
/// following information regarding each entry:
///
/// -  Address Block - A block of IPv4 or IPv6 addresses that has been
///    registered for a special purpose.
///
/// -  Name - A descriptive name for the special-purpose address block.
///
/// -  RFC - The RFC through which the special-purpose address block was
///    requested.
///
/// -  Allocation Date - The date upon which the special-purpose address
///    block was allocated.
///
/// -  Termination Date - The date upon which the allocation is to be
///    terminated.  This field is applicable for limited-use allocations
///    only.
///
/// -  Source - A boolean value indicating whether an address from the
///    allocated special-purpose address block is valid when used as the
///    source address of an IP datagram that transits two devices.
///
/// -  Destination - A boolean value indicating whether an address from
///    the allocated special-purpose address block is valid when used as
///    the destination address of an IP datagram that transits two
///    devices.
///
/// -  Forwardable - A boolean value indicating whether a router may
///    forward an IP datagram whose destination address is drawn from the
///    allocated special-purpose address block between external
///    interfaces.
///
/// -  Global - A boolean value indicating whether an IP datagram whose
///    destination address is drawn from the allocated special-purpose
///    address block is forwardable beyond a specified administrative
///    domain.
///
/// -  Reserved-by-Protocol - A boolean value indicating whether the
///    special-purpose address block is reserved by IP, itself.  This
///    value is "TRUE" if the RFC that created the special-purpose
///    address block requires all compliant IP implementations to behave
///    in a special way when processing packets either to or from
///    addresses contained by the address block.
///
/// If the value of "Destination" is FALSE, the values of "Forwardable"
/// and "Global" must also be false.
///
/// **Addresses:**
/// - **IPv4:**
/// 
///   | Address Block | Name | RFC | Allocation Date | Termination Date | Source | Destination | Forwardable | Global | Reserved-by-Protocol |
///   |--------------|------|-----|-----------------|------------------|--------|-------------|-------------|--------|-------------------|
///   | `0.0.0.0/8` | "This host on this network" | [RFC 1122], Section 3.2.1.3 | September 1981 | N/A | true | false | false | false | true |
///   | `10.0.0.0/8` | Private-Use | [RFC 1918] | February 1996 | N/A | true | true | true | false | false |
///   | `100.64.0.0/10` | Shared Address Space | [RFC 6598] | April 2012 | N/A | true | true | true | false | false |
///   | `127.0.0.0/8` | Loopback¹ | [RFC 1122], Section 3.2.1.3 | September 1981 | N/A | false | false | false | false | true |
///   | `169.254.0.0/16` | Link Local | [RFC 3927] | May 2005 | N/A | true | true | false | false | true |
///   | `172.16.0.0/12` | Private-Use | [RFC 1918] | February 1996 | N/A | true | true | true | false | false |
///   | `192.0.0.0/24`² | IETF Protocol Assignments | Section 2.1 of this document | January 2010 | N/A | false | false | false | false | false |
///   | `192.0.0.0/29` | IPv4 Service Continuity Prefix | [RFC 6333], [RFC 7335] | June 2011 | N/A | true | true | true | false | false |
///   | `192.0.2.0/24` | Documentation (TEST-NET-1) | [RFC 5737] | January 2010 | N/A | false | false | false | false | false |
///   | `192.88.99.0/24` | 6to4 Relay Anycast | [RFC 3068] | June 2001 | N/A | true | true | true | true | false |
///   | `192.168.0.0/16` | Private-Use | [RFC 1918] | February 1996 | N/A | true | true | true | false | false |
///   | `198.18.0.0/15` | Benchmarking | [RFC 2544] | March 1999 | N/A | true | true | true | false | false |
///   | `198.51.100.0/24` | Documentation (TEST-NET-2) | [RFC 5737] | January 2010 | N/A | false | false | false | false | false |
///   | `203.0.113.0/24` | Documentation (TEST-NET-3) | [RFC 5737] | January 2010 | N/A | false | false | false | false | false |
///   | `240.0.0.0/4` | Reserved | [RFC 1112], Section 4 | August 1989 | N/A | false | false | false | false | true |
///   | `255.255.255.255/32` | Limited Broadcast | [RFC 919], Section 7 | October 1984 | N/A | false | true | false | false | false |
/// 
///   ¹ Several protocols have been granted exceptions to this rule. For examples, see [RFC 4379] and [RFC 5884].
/// 
///   ² Not usable unless by virtue of a more specific reservation.
/// 
/// - **IPv6:**
/// 
///   | Address Block | Name | RFC | Allocation Date | Termination Date | Source | Destination | Forwardable | Global | Reserved-by-Protocol |
///   |--------------|------|-----|-----------------|------------------|--------|-------------|-------------|--------|-------------------|
///   | `::1/128` | Loopback Address | [RFC 4291] | February 2006 | N/A | false | false | false | false | true |
///   | `::/128` | Unspecified Address | [RFC 4291] | February 2006 | N/A | true | false | false | false | true |
///   | `64:ff9b::/96` | IPv4-IPv6 Translat. | [RFC 6052] | October 2010 | N/A | true | true | true | true | false |
///   | `::ffff:0:0/96` | IPv4-mapped Address | [RFC 4291] | February 2006 | N/A | false | false | false | false | true |
///   | `100::/64` | Discard-Only Address Block | [RFC 6666] | June 2012 | N/A | true | true | true | false | false |
///   | `2001::/23`¹ | IETF Protocol Assignments | [RFC 2928] | September 2000 | N/A | false | false | false | false | false |
///   | `2001::/32` | TEREDO | [RFC 4380] | January 2006 | N/A | true | true | true | false | false |
///   | `2001:2::/48` | Benchmarking | [RFC 5180] | April 2008 | N/A | true | true | true | false | false |
///   | `2001:db8::/32` | Documentation | [RFC 3849] | July 2004 | N/A | false | false | false | false | false |
///   | `2001:10::/28` | ORCHID | [RFC 4843] | March 2007 | March 2014 | false | false | false | false | false |
///   | `2002::/16`² | 6to4 | [RFC 3056] | February 2001 | N/A | true | true | true | N/A | false |
///   | `fc00::/7` | Unique-Local | [RFC 4193] | October 2005 | N/A | true | true | true | false | false |
///   | `fe80::/10` | Linked-Scoped Unicast | [RFC 4291] | February 2006 | N/A | true | true | false | false | true |
/// 
/// 
///   ¹ Unless allowed by a more specific allocation.  
///   ² See [RFC 3056] for details.
/// 
/// [RFC 919]: https://datatracker.ietf.org/doc/rfc919/
/// [RFC 1112]: https://datatracker.ietf.org/doc/rfc1112/
/// [RFC 1122]: https://datatracker.ietf.org/doc/rfc1122/
/// [RFC 1700]: https://datatracker.ietf.org/doc/rfc1700/
/// [RFC 1918]: https://datatracker.ietf.org/doc/rfc1918/
/// [RFC 2544]: https://datatracker.ietf.org/doc/rfc2544/
/// [RFC 2928]: https://datatracker.ietf.org/doc/rfc2928/
/// [RFC 3056]: https://datatracker.ietf.org/doc/rfc3056/
/// [RFC 3171]: https://datatracker.ietf.org/doc/rfc3171/
/// [RFC 3849]: https://datatracker.ietf.org/doc/rfc3849/
/// [RFC 3927]: https://datatracker.ietf.org/doc/rfc3927/
/// [RFC 4193]: https://datatracker.ietf.org/doc/rfc4193/
/// [RFC 4291]: https://datatracker.ietf.org/doc/rfc4291/
/// [RFC 4379]: https://datatracker.ietf.org/doc/rfc4379/
/// [RFC 4380]: https://datatracker.ietf.org/doc/rfc4380/
/// [RFC 4843]: https://datatracker.ietf.org/doc/rfc4843/
/// [RFC 5180]: https://datatracker.ietf.org/doc/rfc5180/
/// [RFC 5737]: https://datatracker.ietf.org/doc/rfc5737/
/// [RFC 5884]: https://datatracker.ietf.org/doc/rfc5884/
/// [RFC 6052]: https://datatracker.ietf.org/doc/rfc6052/
/// [RFC 6333]: https://datatracker.ietf.org/doc/rfc6333/
/// [RFC 6598]: https://datatracker.ietf.org/doc/rfc6666/
/// [RFC 6666]: https://datatracker.ietf.org/doc/rfc6666/
/// [RFC 6890]: https://datatracker.ietf.org/doc/rfc6890/
pub const RFC6890: RFC = RFC {
  id: 6890,
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

    IpNet::V6(IPV6_1),
    IpNet::V6(IPV6_2),
    IpNet::V6(IPV6_3),
    IpNet::V6(IPV6_4),
    IpNet::V6(IPV6_5),
    IpNet::V6(IPV6_6),
    IpNet::V6(IPV6_7),
    IpNet::V6(IPV6_8),
    IpNet::V6(IPV6_9),
  ],
  ipv4_nets: &[
    IPV4_1, IPV4_2, IPV4_3, IPV4_4, IPV4_5, IPV4_6, IPV4_7, IPV4_8, IPV4_9, IPV4_10, IPV4_11, IPV4_12, IPV4_13, IPV4_14, IPV4_15, IPV4_16,
  ],
  ipv6_nets: &[
    IPV6_1,
    IPV6_2,
    IPV6_3,
    IPV6_4,
    IPV6_5,
    IPV6_6,
    IPV6_7,
    IPV6_8,
    IPV6_9,
  ],
};

#[test]
fn t() {
  for (idx, s) in [
    "0.0.0.0/8",
    "10.0.0.0/8",
    "100.64.0.0/10",
    "127.0.0.0/8",
    "169.254.0.0/16",
    "172.16.0.0/12",
    "192.0.0.0/24",
    "192.0.0.0/29",
    "192.0.2.0/24",
    "192.88.99.0/24",
    "192.168.0.0/16",
    "198.18.0.0/15",
    "198.51.100.0/24",
    "203.0.113.0/24",
    "240.0.0.0/4",
    "255.255.255.255/32",
  ]
  .iter().enumerate() {
    let addr: Ipv4Net = s.parse().unwrap();
    assert_eq!(RFC6890.ipv4_nets[idx], addr);
  }

  for (idx, s) in [
    "::/128",
    "::1/128",
    "64:ff9b::/96",
    "::ffff:0:0/96",
    "100::/64",
    "2001::/16",
    "2002::/16",
    "fc00::/7",
    "fe80::/10",
  ].iter().enumerate() {
    let addr: Ipv6Net = s.parse().unwrap();
    assert_eq!(RFC6890.ipv6_nets[idx], addr);
  }
}
