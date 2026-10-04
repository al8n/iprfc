# Changelog

## Unreleased

- Fix panics in subsets containing `Filter::FORWARDING_BLACKLIST` and accept
  the decimal blacklist identifier in string indexing, preserving existing
  filter bit positions and RFC iteration order.
- Complete RFC 5735's 15-row table, include both mapped and translated IPv6
  prefixes from RFC 2765, and narrow RFC 3068's IPv6 anycast address to `/128`.
- Replace the RFC6890 compatibility aggregate and forwarding blacklist with
  version-pinned, checked-in IANA Special-Purpose Address Space snapshots.
  The blacklist uses each most-specific `Forwardable` value, including
  explicit unknown holes, and is generated as canonical disjoint CIDRs.
- Complete the historical RFC 3330 September 2002 summary table and clarify
  that named historical RFC table constants remain document snapshots; RFC6890
  is the explicit IANA-registry compatibility exception.
- Clarify compatibility behavior for `Filter`, `RFCs`, and factual RFC-table
  corrections.
- Support `serde` without `std`, test stable feature combinations, Rust 1.81,
  and bare-metal builds in CI, and replace obsolete coverage actions.
- Replace the unmaintained `paste` dependency with `pastey` 0.2.

## Released

## 0.2.3 (July 1, 2026)

- Add const semantic IP classifiers.
- Add RFC 9637's `3fff::/20` IPv6 documentation prefix.

## 0.2.2 (June 8, 2026)

- Fix the `RFC6890` IPv6 table: `2001::/16` over-covered the whole block and
  swallowed real global-unicast space (e.g. RIR allocations, `2001:4860::`).
  Model the RFC 6890 table's actual special-purpose ranges instead —
  `2001::/23` (IETF Protocol Assignments) and `2001:db8::/32`
  (Documentation).

## 0.2.0 (January 14, 2025)

- Add `Filter` and `RFCs`.
- Add indexing APIs.

## 0.1.1 (January 13, 2025)

- Published version; this repository does not record a separately attributable
  source change.

## 0.1.0 (January 13, 2025)

- Add RFC919, RFC1112, RFC1122, RFC1918, RFC2544, RFC2765, RFC2928, RFC3056,
  RFC3068, RFC3171, RFC3330, RFC3849, RFC3927, RFC4038, RFC4193, RFC4291,
  RFC4380, RFC4773, RFC4843, RFC5180, RFC5735, RFC5737, RFC6052, RFC6333,
  RFC6598, RFC6666, RFC6890, and RFC7335.
