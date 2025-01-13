#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

#[cfg(test)]
extern crate std;

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub use ipnet::{IpNet, Ipv4Net, Ipv6Net};

pub use forwarding_black_list::FORWARDING_BLACK_LIST;

mod forwarding_black_list;

macro_rules! rfcs {
  ($($id:literal), +$(,)?) => {
    paste::paste! {
      $(
        pub use [<rfc $id>]::[< RFC $id >];
      )+

      $(
        mod [<rfc $id>];
      )+

      /// All known RFCs
      ///
      /// Useful resources:
      ///
      /// * https://www.iana.org/assignments/ipv6-address-space/ipv6-address-space.xhtml
      /// * https://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xhtml
      /// * https://www.iana.org/assignments/ipv6-address-space/ipv6-address-space.xhtml
      #[allow(rustdoc::bare_urls)]
      pub const RFCS: &[RFC] = &[
        $([<RFC $id>]),+,
        FORWARDING_BLACK_LIST
      ];
    }
  };
}

rfcs! {
  919, 1112, 1122, 1918, 2544, 2765, 2928, 3056, 3068, 3171, 3330, 3849, 3927, 4038, 4193, 4291, 4380, 4773, 4843, 5180, 5735, 5737, 6052, 6333, 6598, 6666, 6890, 7335,
}

/// RFC
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RFC {
  id: u32,
  ip_nets: &'static [IpNet],
  ipv4_nets: &'static [Ipv4Net],
  ipv6_nets: &'static [Ipv6Net],
}

impl RFC {
  /// Get the identifier of the RFC
  ///
  /// ## Example
  ///
  /// ```rust
  /// use iprfc::RFC6890;
  ///
  /// assert_eq!(6890, RFC6890.id());
  /// ```
  #[inline]
  pub const fn id(&self) -> u32 {
    self.id
  }

  /// Get all of the IP addresses of the RFC
  ///
  /// ## Example
  ///
  /// ```rust
  /// use iprfc::RFC6890;
  ///
  /// for ip in RFC6890.ip_nets() {
  ///   println!("{}", ip);
  /// }
  /// ```
  #[inline]
  pub const fn ip_nets(&self) -> &'static [IpNet] {
    self.ip_nets
  }

  /// Get all of the IPv4 addresses of the RFC
  ///
  /// ## Example
  ///
  /// ```rust
  /// use iprfc::RFC6890;
  ///
  /// for ip in RFC6890.ipv4_nets() {
  ///   println!("{}", ip);
  /// }
  /// ```
  #[inline]
  pub const fn ipv4_nets(&self) -> &'static [Ipv4Net] {
    self.ipv4_nets
  }

  /// Get all of the IPv6 addresses of the RFC
  ///
  /// ## Example
  ///
  /// ```rust
  /// use iprfc::RFC6890;
  ///
  /// for ip in RFC6890.ipv6_nets() {
  ///   println!("{}", ip);
  /// }
  #[inline]
  pub const fn ipv6_nets(&self) -> &'static [Ipv6Net] {
    self.ipv6_nets
  }

  /// Returns `true` if the ip is contained by the [`RFC`].
  ///
  /// ## Example
  ///
  /// ```rust
  /// use iprfc::{RFC6890, IpNet, Ipv4Net, Ipv6Net};
  /// use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
  /// use std::str::FromStr;
  ///
  /// // IPv4 address examples
  /// let private_ip = Ipv4Addr::new(192, 168, 1, 1);
  /// assert!(RFC6890.contains(&private_ip));
  ///
  /// let loopback_ip = Ipv4Addr::new(127, 0, 0, 1);
  /// assert!(RFC6890.contains(&loopback_ip));
  ///
  /// let public_ip = Ipv4Addr::new(8, 8, 8, 8);
  /// assert!(!RFC6890.contains(&public_ip));
  ///
  /// let testnet_ip = Ipv4Addr::new(192, 0, 2, 1);
  /// assert!(RFC6890.contains(&testnet_ip));
  ///
  /// // IPv4 network examples
  /// let private_net = Ipv4Net::from_str("192.168.0.0/24").unwrap();
  /// assert!(RFC6890.contains(&private_net));
  ///
  /// let public_net = Ipv4Net::from_str("8.8.8.0/24").unwrap();
  /// assert!(!RFC6890.contains(&public_net));
  ///
  /// let testnet_net = Ipv4Net::from_str("192.0.2.0/28").unwrap();
  /// assert!(RFC6890.contains(&testnet_net));
  ///
  /// // IPv6 address examples
  /// let ipv6_loopback = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
  /// assert!(RFC6890.contains(&ipv6_loopback));
  ///
  /// let ipv6_doc = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
  /// assert!(RFC6890.contains(&ipv6_doc));
  ///
  /// let ipv6_linklocal = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
  /// assert!(RFC6890.contains(&ipv6_linklocal));
  ///
  /// // IPv6 network examples
  /// let ipv6_doc_net = Ipv6Net::from_str("2001:db8::/48").unwrap();
  /// assert!(RFC6890.contains(&ipv6_doc_net));
  ///
  /// let ipv6_ula_net = Ipv6Net::from_str("fc00::/7").unwrap();
  /// assert!(RFC6890.contains(&ipv6_ula_net));
  ///
  /// let ipv6_global_net = Ipv6Net::from_str("2a00::/32").unwrap();
  /// assert!(!RFC6890.contains(&ipv6_global_net));
  ///
  /// // Works with IpNet enum too
  /// let ip_net: IpNet = private_net.into();
  /// assert!(RFC6890.contains(&ip_net));
  ///
  /// // Works with IpAddr enum too
  /// let ip: IpAddr = private_ip.into();
  /// assert!(RFC6890.contains(&ip));
  /// ```
  #[inline]
  pub fn contains<T>(&self, ip: &T) -> bool
  where
    Self: Contains<T>,
  {
    Contains::contains(self, ip)
  }
}

/// Returns `true` if the [`RFC`] contains `T`.
pub trait Contains<T>: sealed::Sealed {
  /// Returns `true` if the [`RFC`] contains `T`.
  fn contains(&self, t: &T) -> bool;
}

impl Contains<IpNet> for RFC {
  #[inline]
  fn contains(&self, ip: &IpNet) -> bool {
    self.ip_nets.iter().any(|&i| i.contains(ip))
  }
}

impl Contains<IpAddr> for RFC {
  #[inline]
  fn contains(&self, ip: &IpAddr) -> bool {
    self.ip_nets.iter().any(|&i| i.contains(ip))
  }
}

impl Contains<Ipv4Net> for RFC {
  #[inline]
  fn contains(&self, ip: &Ipv4Net) -> bool {
    self.ipv4_nets.iter().any(|&i| i.contains(ip))
  }
}

impl Contains<Ipv4Addr> for RFC {
  #[inline]
  fn contains(&self, ip: &Ipv4Addr) -> bool {
    self.ipv4_nets.iter().any(|&i| i.contains(ip))
  }
}

impl Contains<Ipv6Net> for RFC {
  #[inline]
  fn contains(&self, ip: &Ipv6Net) -> bool {
    self.ipv6_nets.iter().any(|&i| i.contains(ip))
  }
}

impl Contains<Ipv6Addr> for RFC {
  #[inline]
  fn contains(&self, ip: &Ipv6Addr) -> bool {
    self.ipv6_nets.iter().any(|&i| i.contains(ip))
  }
}

mod sealed {
  pub trait Sealed {}

  impl Sealed for super::RFC {}
}
