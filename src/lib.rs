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

pub use forwarding_black_list::{FORWARDING_BLACKLIST, FORWARDING_BLACKLIST_ID};

mod forwarding_black_list;

macro_rules! rfcs {
  ($(($index:literal, $id:literal)), +$(,)?) => {
    paste::paste! {
      $(
        pub use [<rfc $id>]::[< RFC $id >];
      )+

      $(
        mod [<rfc $id>];
      )+

      bitflags::bitflags! {
        /// Address-related RFC flags
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Filter: u128 {
          $(
            #[doc = "[RFC " $id "](https://tools.ietf.org/html/rfc" $id ")"]
            const [<RFC $id>] = 1 << $index;
          )+
          /// Addresses that should not be forwarded
          const FORWARDING_BLACKLIST = 1 << 127;
        }
      }

      const RFCS: &[RFC] = &[
        $([<RFC $id>]),+,
        FORWARDING_BLACKLIST
      ];

      impl ::core::ops::Index<u32> for RFCs {
        type Output = RFC;

        fn index(&self, index: u32) -> &Self::Output {
          match index {
            $(
              $id => &RFCS[$index],
            )+
            val if val == FORWARDING_BLACKLIST_ID => RFCS.last().unwrap(),
            val => panic!("{val} is not a valid RFC identifier"),
          }
        }
      }

      impl ::core::ops::Index<&str> for RFCs {
        type Output = RFC;

        fn index(&self, index: &str) -> &Self::Output {
          let s = if index.starts_with("RFC") {
            index.trim_start_matches("RFC").trim()
          } else if index.starts_with("rfc") {
            index.trim_start_matches("rfc").trim()
          } else {
            index
          };

          match s {
            $(
              stringify!($id) => &RFCS[$index],
            )+
            "blacklist" | "BLACKLIST" | "blocked" | "BLOCKED" => RFCS.last().unwrap(),
            val if val == stringify!(FORWARDING_BLACKLIST_ID) => RFCS.last().unwrap(),
            val => panic!("{val} is not a valid RFC identifier"),
          }
        }
      }

      impl RFCs {
        /// Returns the number of known RFCs
        ///
        /// ## Example
        ///
        /// ```rust
        /// use iprfc::RFCs;
        ///
        /// assert_eq!(29, RFCs::len());
        /// ```
        #[inline]
        pub const fn len() -> usize {
          RFCS.len()
        }

        /// Returns `true` if such id is a valid RFC identifier
        ///
        /// ## Example
        ///
        /// ```rust
        /// use iprfc::{RFCs, FORWARDING_BLACKLIST_ID};
        ///
        /// assert!(RFCs::contains(6890));
        /// assert!(RFCs::contains(FORWARDING_BLACKLIST_ID));
        /// assert!(!RFCs::contains(9999));
        /// ```
        #[inline]
        pub const fn contains(id: u32) -> bool {
          match id {
            $(
              $id => true,
            )+
            FORWARDING_BLACKLIST_ID => true,
            _ => false,
          }
        }

        /// Returns the RFC with the given id, if it exists
        ///
        /// ## Example
        ///
        /// ```rust
        /// use iprfc::RFCs;
        ///
        /// let rfc = RFCs::get(6890);
        /// assert!(rfc.is_some());
        ///
        /// let rfc = RFCs::get(9999);
        /// assert!(rfc.is_none());
        /// ```
        #[inline]
        pub const fn get(id: u32) -> Option<&'static RFC> {
          match id {
            $(
              $id => Some(&[<RFC $id>]),
            )+
            FORWARDING_BLACKLIST_ID => Some(&FORWARDING_BLACKLIST),
            _ => None,
          }
        }

        /// Returns the RFC with the given id, if it exists, otherwise panics
        ///
        /// ## Example
        ///
        /// ```rust
        /// use iprfc::RFCs;
        ///
        /// let rfc = RFCs::get_unchecked(6890);
        /// ```
        #[inline]
        pub const fn get_unchecked(id: u32) -> &'static RFC {
          match id {
            $(
              $id => &[<RFC $id>],
            )+
            FORWARDING_BLACKLIST_ID => &FORWARDING_BLACKLIST,
            _ => panic!("not a valid RFC identifier"),
          }
        }
      }
    }
  };
}

rfcs! {
  (0, 919), (1, 1112), (2, 1122), (3, 1918), (4, 2544), (5, 2765), (6, 2928), (7, 3056), (8, 3068), (9, 3171), (10, 3330), (11, 3849), (12, 3927), (13, 4038), (14, 4193), (15, 4291), (16, 4380), (17, 4773), (18, 4843), (19, 5180), (20, 5735), (21, 5737), (22, 6052), (23, 6333), (24, 6598), (25, 6666), (26, 6890), (27, 7335),
}

/// All known RFCs
///
/// Useful resources:
///
/// * https://www.iana.org/assignments/ipv6-address-space/ipv6-address-space.xhtml
/// * https://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xhtml
/// * https://www.iana.org/assignments/ipv6-address-space/ipv6-address-space.xhtml
#[allow(rustdoc::bare_urls)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RFCs;

impl RFCs {
  /// Iterate over all known RFCs
  ///
  /// ## Example
  ///
  /// ```rust
  /// use iprfc::RFCs;
  ///
  /// for rfc in RFCs::iter() {
  ///   println!("{}", rfc.id());
  /// }
  /// ```
  #[inline]
  pub fn iter<'a>() -> core::slice::Iter<'a, RFC> {
    RFCS.iter()
  }

  /// Returns a subset of [`RFCs`] by filtering with the given [`Filter`]
  ///
  /// ## Example
  ///
  /// ```rust
  /// use iprfc::{RFCs, Filter};
  ///
  /// let subset = RFCs::filter(Filter::RFC919 | Filter::RFC1122);
  /// ```
  #[inline]
  pub const fn filter(filter: Filter) -> Subset {
    Subset(filter)
  }
}

/// A subset of [`RFCs`], useful for filtering
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Subset(Filter);

impl Subset {
  /// Returns `true` if the subset contains the `T`
  ///
  /// ## Example
  ///
  /// ```rust
  /// use iprfc::{RFCs, Filter};
  /// use std::net::{Ipv4Addr, Ipv6Addr};
  ///
  /// let subset = RFCs::filter(Filter::RFC919 | Filter::RFC1122);
  /// assert!(subset.contains(&Ipv4Addr::BROADCAST));
  /// assert!(!subset.contains(&Ipv6Addr::LOCALHOST));
  /// ```
  pub fn contains<T>(&self, ip: &T) -> bool
  where
    RFC: Contains<T>,
  {
    self.0.iter_names().any(|(n, _)| RFCs[n].contains(ip))
  }
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

#[test]
fn test_indexable_by_id() {
  let rfc = RFCs[6890];
  assert_eq!(6890, rfc.id());

  let rfc = RFCs[FORWARDING_BLACKLIST_ID];
  assert_eq!(FORWARDING_BLACKLIST_ID, rfc.id());
}

#[test]
#[should_panic]
fn test_indexable_by_id_panic() {
  let _ = RFCs[9999];
}

#[test]
fn test_get() {
  let rfc = RFCs::get(FORWARDING_BLACKLIST_ID).unwrap();
  assert_eq!(FORWARDING_BLACKLIST_ID, rfc.id());

  let rfc = RFCs::get(7335).unwrap();
  assert_eq!(7335, rfc.id());
}

#[test]
fn test_get_unchecked() {
  let rfc = RFCs::get_unchecked(FORWARDING_BLACKLIST_ID);
  assert_eq!(FORWARDING_BLACKLIST_ID, rfc.id());

  let rfc = RFCs::get_unchecked(7335);
  assert_eq!(7335, rfc.id());
}

#[test]
#[should_panic]
fn test_get_unchecked_panic() {
  let _ = RFCs::get_unchecked(9999);
}

#[test]
fn test_indexable_by_str() {
  let rfc = RFCs["6890"];
  assert_eq!(6890, rfc.id());

  let rfc = RFCs["RFC6890"];
  assert_eq!(6890, rfc.id());

  let rfc = RFCs["rfc 6890"];
  assert_eq!(6890, rfc.id());

  let rfc = RFCs["blacklist"];
  assert_eq!(rfc.id(), FORWARDING_BLACKLIST_ID);
}

#[test]
#[should_panic]
fn test_indexable_by_str_panic() {
  let _ = RFCs["9999"];
}
