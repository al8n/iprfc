# RELEASED

## 0.2.2

- Fix the `RFC6890` IPv6 table: `2001::/16` over-covered the whole block and
  swallowed real global-unicast space (e.g. RIR allocations, `2001:4860::`).
  Model the actual special-purpose ranges instead — `2001::/23` (IETF Protocol
  Assignments) and `2001:db8::/32` (Documentation) — matching the documented
  table and the IANA IPv6 Special-Purpose registry.

## 0.1.0 (January 14th, 2025)

- Add `Filter` and `RFCs` struct
- Add indexing APIs

## 0.1.0 (January 13rd, 2025)

- RFC919, RFC1112, RFC1122, RFC1918, RFC2544, RFC2765, RFC2928, RFC3056, RFC3068, RFC3171, RFC3330, RFC3849, RFC3927, RFC4038, RFC4193, RFC4291, RFC4380, RFC4773, RFC4843, RFC5180, RFC5735, RFC5737, RFC6052, RFC6333, RFC6598, RFC6666, RFC6890, RFC7335
