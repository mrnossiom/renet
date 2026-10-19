use std::fmt;

use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

pub(crate) mod arp;
pub(crate) mod ethernet;
pub(crate) mod ipv4;
pub(crate) mod ipv6;

#[derive(KnownLayout, Immutable, FromBytes, Unaligned)]
#[repr(transparent)]
pub(crate) struct MacAddr {
	octets: [u8; 6],
}

impl From<[u8; 6]> for MacAddr {
	fn from(value: [u8; 6]) -> Self {
		MacAddr { octets: value }
	}
}

impl fmt::Display for MacAddr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let [c1, c2, c3, c4, c5, c6] = self.octets;
		write!(f, "{c1:02X}:{c2:02X}:{c3:02X}:{c4:02X}:{c5:02X}:{c6:02X}")
	}
}

impl fmt::Debug for MacAddr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "MacAddr({self})")
	}
}
