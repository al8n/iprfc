use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Returns `true` if the IPv4 address is in `127.0.0.0/8`.
#[inline]
pub const fn is_loopback_ipv4_addr(ip: Ipv4Addr) -> bool {
  let [a, _, _, _] = ip.octets();
  a == 127
}

/// Returns `true` if the IPv6 address is `::1/128`.
#[inline]
pub const fn is_loopback_ipv6_addr(ip: Ipv6Addr) -> bool {
  let segments = ip.segments();
  segments[0] == 0
    && segments[1] == 0
    && segments[2] == 0
    && segments[3] == 0
    && segments[4] == 0
    && segments[5] == 0
    && segments[6] == 0
    && segments[7] == 1
}

/// Returns `true` if the IP address is loopback.
#[inline]
pub const fn is_loopback_ip_addr(ip: IpAddr) -> bool {
  match ip {
    IpAddr::V4(ip) => is_loopback_ipv4_addr(ip),
    IpAddr::V6(ip) => is_loopback_ipv6_addr(ip),
  }
}

/// Returns `true` if the IPv4 address is in RFC 1918 private-use space.
#[inline]
pub const fn is_private_ipv4_addr(ip: Ipv4Addr) -> bool {
  let [a, b, _, _] = ip.octets();
  a == 10 || (a == 172 && b >= 16 && b <= 31) || (a == 192 && b == 168)
}

/// Returns `true` if the IPv6 address is in RFC 4193 unique-local space.
#[inline]
pub const fn is_unique_local_ipv6_addr(ip: Ipv6Addr) -> bool {
  let segments = ip.segments();
  segments[0] & 0xfe00 == 0xfc00
}

/// Returns `true` if the IP address is RFC 1918 private-use or RFC 4193 unique-local.
#[inline]
pub const fn is_private_ip_addr(ip: IpAddr) -> bool {
  match ip {
    IpAddr::V4(ip) => is_private_ipv4_addr(ip),
    IpAddr::V6(ip) => is_unique_local_ipv6_addr(ip),
  }
}

/// Returns `true` if the IPv4 address is in RFC 3927 link-local space.
#[inline]
pub const fn is_link_local_ipv4_addr(ip: Ipv4Addr) -> bool {
  let [a, b, _, _] = ip.octets();
  a == 169 && b == 254
}

/// Returns `true` if the IPv6 address is in `fe80::/10` link-local space.
#[inline]
pub const fn is_link_local_ipv6_addr(ip: Ipv6Addr) -> bool {
  let segments = ip.segments();
  segments[0] & 0xffc0 == 0xfe80
}

/// Returns `true` if the IP address is link-local.
#[inline]
pub const fn is_link_local_ip_addr(ip: IpAddr) -> bool {
  match ip {
    IpAddr::V4(ip) => is_link_local_ipv4_addr(ip),
    IpAddr::V6(ip) => is_link_local_ipv6_addr(ip),
  }
}

/// Returns `true` if the IPv4 address is in an RFC 5737 documentation block.
#[inline]
pub const fn is_documentation_ipv4_addr(ip: Ipv4Addr) -> bool {
  let [a, b, c, _] = ip.octets();
  (a == 192 && b == 0 && c == 2)
    || (a == 198 && b == 51 && c == 100)
    || (a == 203 && b == 0 && c == 113)
}

/// Returns `true` if the IPv6 address is in an RFC 3849 or RFC 9637 documentation block.
#[inline]
pub const fn is_documentation_ipv6_addr(ip: Ipv6Addr) -> bool {
  let segments = ip.segments();
  (segments[0] == 0x2001 && segments[1] == 0x0db8)
    || (segments[0] == 0x3fff && segments[1] <= 0x0fff)
}

/// Returns `true` if the IP address is reserved for documentation or examples.
#[inline]
pub const fn is_documentation_ip_addr(ip: IpAddr) -> bool {
  match ip {
    IpAddr::V4(ip) => is_documentation_ipv4_addr(ip),
    IpAddr::V6(ip) => is_documentation_ipv6_addr(ip),
  }
}

/// Returns `true` if the IPv4 address is in RFC 2544 benchmarking space.
#[inline]
pub const fn is_benchmark_ipv4_addr(ip: Ipv4Addr) -> bool {
  let [a, b, _, _] = ip.octets();
  a == 198 && (b == 18 || b == 19)
}

/// Returns `true` if the IPv6 address is in RFC 5180 benchmarking space.
#[inline]
pub const fn is_benchmark_ipv6_addr(ip: Ipv6Addr) -> bool {
  let segments = ip.segments();
  segments[0] == 0x2001 && segments[1] == 0x0002 && segments[2] == 0
}

/// Returns `true` if the IP address is reserved for benchmarking.
#[inline]
pub const fn is_benchmark_ip_addr(ip: IpAddr) -> bool {
  match ip {
    IpAddr::V4(ip) => is_benchmark_ipv4_addr(ip),
    IpAddr::V6(ip) => is_benchmark_ipv6_addr(ip),
  }
}

/// Returns `true` if the IPv4 address is in RFC 6598 shared address space.
#[inline]
pub const fn is_shared_ipv4_addr(ip: Ipv4Addr) -> bool {
  let [a, b, _, _] = ip.octets();
  a == 100 && b >= 64 && b <= 127
}

/// Returns `true` if the IP address is in shared address space.
#[inline]
pub const fn is_shared_ip_addr(ip: IpAddr) -> bool {
  match ip {
    IpAddr::V4(ip) => is_shared_ipv4_addr(ip),
    IpAddr::V6(_) => false,
  }
}

/// Returns `true` if the IPv4 address is multicast.
#[inline]
pub const fn is_multicast_ipv4_addr(ip: Ipv4Addr) -> bool {
  let [a, _, _, _] = ip.octets();
  a >= 224 && a <= 239
}

/// Returns `true` if the IPv6 address is multicast.
#[inline]
pub const fn is_multicast_ipv6_addr(ip: Ipv6Addr) -> bool {
  let segments = ip.segments();
  segments[0] & 0xff00 == 0xff00
}

/// Returns `true` if the IP address is multicast.
#[inline]
pub const fn is_multicast_ip_addr(ip: IpAddr) -> bool {
  match ip {
    IpAddr::V4(ip) => is_multicast_ipv4_addr(ip),
    IpAddr::V6(ip) => is_multicast_ipv6_addr(ip),
  }
}

/// Returns `true` if the IPv4 address is exactly `0.0.0.0/32`.
#[inline]
pub const fn is_unspecified_ipv4_addr(ip: Ipv4Addr) -> bool {
  let [a, b, c, d] = ip.octets();
  a == 0 && b == 0 && c == 0 && d == 0
}

/// Returns `true` if the IPv6 address is exactly `::/128`.
#[inline]
pub const fn is_unspecified_ipv6_addr(ip: Ipv6Addr) -> bool {
  let segments = ip.segments();
  segments[0] == 0
    && segments[1] == 0
    && segments[2] == 0
    && segments[3] == 0
    && segments[4] == 0
    && segments[5] == 0
    && segments[6] == 0
    && segments[7] == 0
}

/// Returns `true` if the IP address is the exact unspecified address.
#[inline]
pub const fn is_unspecified_ip_addr(ip: IpAddr) -> bool {
  match ip {
    IpAddr::V4(ip) => is_unspecified_ipv4_addr(ip),
    IpAddr::V6(ip) => is_unspecified_ipv6_addr(ip),
  }
}

/// Returns `true` if the IPv4 address is exactly `255.255.255.255/32`.
#[inline]
pub const fn is_broadcast_ipv4_addr(ip: Ipv4Addr) -> bool {
  let [a, b, c, d] = ip.octets();
  a == 255 && b == 255 && c == 255 && d == 255
}

/// Returns `true` if the IP address is the IPv4 limited broadcast address.
#[inline]
pub const fn is_broadcast_ip_addr(ip: IpAddr) -> bool {
  match ip {
    IpAddr::V4(ip) => is_broadcast_ipv4_addr(ip),
    IpAddr::V6(_) => false,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn ip(s: &str) -> IpAddr {
    s.parse().unwrap()
  }

  fn ipv6(s: &str) -> Ipv6Addr {
    s.parse().unwrap()
  }

  macro_rules! assert_class {
    ($predicate:ident, [$($valid:literal),+ $(,)?], [$($invalid:literal),+ $(,)?]) => {{
      $(assert!($predicate(ip($valid)), "{} should match", $valid);)+
      $(assert!(!$predicate(ip($invalid)), "{} should not match", $invalid);)+
    }};
  }

  #[test]
  fn common_semantic_classes_match_boundaries() {
    assert_class!(
      is_loopback_ip_addr,
      ["127.0.0.1", "127.255.255.255", "::1"],
      ["126.255.255.255", "128.0.0.0", "::2"]
    );
    assert_class!(
      is_private_ip_addr,
      [
        "10.0.0.0",
        "10.255.255.255",
        "172.16.0.0",
        "172.31.255.255",
        "192.168.0.0",
        "192.168.255.255",
        "fc00::",
        "fdff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
      ],
      [
        "9.255.255.255",
        "11.0.0.0",
        "172.15.255.255",
        "172.32.0.0",
        "192.167.255.255",
        "192.169.0.0",
        "fbff:ffff::",
        "fe00::"
      ]
    );
    assert_class!(
      is_link_local_ip_addr,
      [
        "169.254.0.0",
        "169.254.255.255",
        "fe80::",
        "febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
      ],
      ["169.253.255.255", "169.255.0.0", "fe7f:ffff::", "fec0::"]
    );
    assert_class!(
      is_documentation_ip_addr,
      [
        "192.0.2.0",
        "192.0.2.255",
        "198.51.100.0",
        "198.51.100.255",
        "203.0.113.0",
        "203.0.113.255",
        "2001:db8::",
        "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff",
        "3fff::",
        "3fff:0fff:ffff:ffff:ffff:ffff:ffff:ffff"
      ],
      [
        "192.0.1.255",
        "192.0.3.0",
        "198.51.99.255",
        "198.51.101.0",
        "203.0.112.255",
        "203.0.114.0",
        "2001:db7:ffff::",
        "2001:db9::",
        "3ffe:ffff::",
        "3fff:1000::"
      ]
    );
    assert_class!(
      is_benchmark_ip_addr,
      [
        "198.18.0.0",
        "198.19.255.255",
        "2001:2::",
        "2001:2:0:ffff:ffff:ffff:ffff:ffff"
      ],
      [
        "198.17.255.255",
        "198.20.0.0",
        "2001:1:ffff::",
        "2001:2:1::",
        "2001:200::"
      ]
    );
    assert_class!(
      is_shared_ip_addr,
      ["100.64.0.0", "100.127.255.255"],
      ["100.63.255.255", "100.128.0.0", "::1"]
    );
    assert_class!(
      is_multicast_ip_addr,
      [
        "224.0.0.0",
        "239.255.255.255",
        "ff00::",
        "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
      ],
      ["223.255.255.255", "240.0.0.0", "feff:ffff::"]
    );
    assert_class!(
      is_unspecified_ip_addr,
      ["0.0.0.0", "::"],
      ["0.0.0.1", "::1"]
    );
    assert_class!(
      is_broadcast_ip_addr,
      ["255.255.255.255"],
      ["255.255.255.254", "::"]
    );
  }

  #[test]
  fn benchmark_ipv6_classifier_matches_rfc5180() {
    for s in ["2001:2::", "2001:2:0:ffff:ffff:ffff:ffff:ffff"] {
      let ip = ipv6(s);
      assert!(is_benchmark_ipv6_addr(ip), "{s} should be benchmarking");
      assert!(crate::RFC5180.contains(&ip), "RFC5180 should contain {s}");
    }

    for s in ["2001:2:1::", "2001:200::"] {
      let ip = ipv6(s);
      assert!(
        !is_benchmark_ipv6_addr(ip),
        "{s} should not be benchmarking"
      );
      assert!(
        !crate::RFC5180.contains(&ip),
        "RFC5180 should not contain {s}"
      );
    }
  }
}
