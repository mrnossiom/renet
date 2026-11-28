use std::fmt;

pub(crate) mod arp;
pub(crate) mod ethernet;
pub(crate) mod ipv4;
pub(crate) mod ipv6;

mod field {
	use std::ops::Range;

	pub(crate) type Field = Range<usize>;

	#[inline(always)]
	pub(crate) fn parse_u8(bytes: &[u8], field: Field) -> u8 {
		bytes[field][0]
	}
	#[inline(always)]
	pub(crate) fn parse_u16(bytes: &[u8], field: Field) -> u16 {
		u16::from_be_bytes(*bytes[field].as_array().unwrap())
	}
	#[inline(always)]
	pub(crate) fn parse_u32(bytes: &[u8], field: Field) -> u32 {
		u32::from_be_bytes(*bytes[field].as_array().unwrap())
	}
	#[inline(always)]
	pub(crate) fn parse_bytes<const N: usize>(bytes: &[u8], field: Field) -> [u8; N] {
		*bytes[field].as_array().unwrap()
	}
}

pub(crate) struct MacAddr {
	octets: [u8; 6],
}

impl MacAddr {
	pub const BROADCAST: Self = Self { octets: [0xFF; _] };
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
		if self.octets.iter().all(|c| *c == 0xFF) {
			write!(f, "MacAddr::BROADCAST")
		} else {
			write!(f, "MacAddr({self})")
		}
	}
}

enum State {
	Link,
	Network,
	Transport,
	Session,
}

pub(crate) struct Cursor<'a> {
	next: State,
	frame: &'a [u8],
}
