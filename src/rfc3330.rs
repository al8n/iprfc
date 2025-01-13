use core::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};

use super::RFC;

/// 0.0.0.0/8
const IPV4_1: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(0, 0, 0, 0), 8);

/// 10.0.0.0/8
const IPV4_2: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(10, 0, 0, 0), 8);

/// 127.0.0.1/8
const IPV4_3: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(127, 0, 0, 0), 8);

/// 169.254.0.0/16
const IPV4_4: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(169, 254, 0, 0), 16);

/// 172.16.0.0/12
const IPV4_5: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(172, 16, 0, 0), 12);

/// 192.0.2.0/24
const IPV4_6: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 0, 2, 0), 24);

/// 192.88.99.0/24
const IPV4_7: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 88, 99, 0), 24);

/// 192.168.0.0/16
const IPV4_8: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(192, 168, 0, 0), 16);

/// 192.18.0.0/15
const IPV4_9: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(198, 18, 0, 0), 15);

/// 224.0.0.0/4
const IPV4_10: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(224, 0, 0, 0), 4);

/// 240.0.0.0/4
const IPV4_11: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(240, 0, 0, 0), 4);

/// [RFC 3330] Special-Use IPv4 Addresses
///
/// **Addresses:**
/// - **IPv4:**
///   1. `0.0.0.0/8`: Addresses in this block refer to source hosts on
///       "this" network.  Address 0.0.0.0/32 may be used as a
///       source address for this host on this network; other
///       addresses within 0.0.0.0/8 may be used to refer to
///       specified hosts on this network [RFC 1700, page 4].
///   2. `10.0.0.0/8`: This block is set aside for use in private networks.
///       Its intended use is documented in [RFC 1918].
///       Addresses within this block should not
///       appear on the public Internet.
///   3. `127.0.0.0/8`: This block is assigned for use as the Internet host
///       loopback address.  A datagram sent by a higher level protocol to an
///       address anywhere within this block should loop back inside the host.
///       This is ordinarily implemented using only 127.0.0.1/32 for loopback,
///       but no addresses within this block should ever appear on any network
///       anywhere [RFC 1700, page 5].
///   4. `169.254.0.0/16`: This is the "link local" block.  It
///       is allocated for communication between hosts on a
///       single link.  Hosts obtain these addresses by
///       auto-configuration, such as when a DHCP server may
///       not be found.
///   5. `172.16.0.0/12`: This block is set aside for use in
///       private networks.  Its intended use is documented in
///       [RFC 1918].  Addresses within this block should not
///       appear on the public Internet.
///   6. `192.0.2.0/24`: This block is assigned as "TEST-NET" for use in
///       documentation and example code.  It is often used in conjunction with
///       domain names `example.com` or `example.net` in vendor and protocol
///       documentation.  Addresses within this block should not appear on the
///       public Internet.
///   7. `192.88.99.0/24`: This block is allocated for use as 6to4 relay
///       anycast addresses, according to [RFC 3068].
///   8. `192.168.0.0/16`: This block is set aside for use in private networks.
///       Its intended use is documented in [RFC 1918].  Addresses within this
///       block should not appear on the public Internet.
///   9. `198.18.0.0/15`: This block has been allocated for use
///       in benchmark tests of network interconnect devices.
///       Its use is documented in [RFC 2544].
///   10. `224.0.0.0/4`: This block, formerly known as the Class
///       D address space, is allocated for use in IPv4
///       multicast address assignments.  The IANA guidelines
///       for assignments from this space are described in
///       [RFC 3171].
/// 
///   11. `240.0.0.0/4`: This block, formerly known as the Class E address
///       space, is reserved.  The "limited broadcast" destination address
///       255.255.255.255 should never be forwarded outside the (sub-)net of
///       the source.  The remainder of this space is reserved
///       for future use.  [RFC 1700, page 4]
/// 
/// [RFC 3330]: https://datatracker.ietf.org/doc/rfc3330/
/// [RFC 3068]: https://datatracker.ietf.org/doc/rfc3068/
/// [RFC 3171]: https://datatracker.ietf.org/doc/rfc3171/
/// [RFC 1700]: https://datatracker.ietf.org/doc/rfc1700/
/// [RFC 1700, page 4]: https://datatracker.ietf.org/doc/rfc1700/
/// [RFC 1700, page 5]: https://datatracker.ietf.org/doc/rfc1700/
/// [RFC 1918]: https://datatracker.ietf.org/doc/rfc1918/
/// [RFC 2544]: https://datatracker.ietf.org/doc/rfc2544/
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
  ],
  ipv4_nets: &[
    IPV4_1, IPV4_2, IPV4_3, IPV4_4, IPV4_5, IPV4_6, IPV4_7, IPV4_8, IPV4_9, IPV4_10, IPV4_11,
  ],
  ipv6_nets: &[],
};

#[test]
fn t() {
  for (idx, s) in [
    "0.0.0.0/8",
    "10.0.0.0/8",
    "127.0.0.0/8",
    "169.254.0.0/16",
    "172.16.0.0/12",
    "192.0.2.0/24",
    "192.88.99.0/24",
    "192.168.0.0/16",
    "198.18.0.0/15",
    "224.0.0.0/4",
    "240.0.0.0/4"
  ]
  .iter()
  .enumerate()
  {
    let addr: Ipv4Net = s.parse().unwrap();
    assert_eq!(RFC3330.ipv4_nets[idx], addr);
  }
}
