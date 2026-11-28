#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct Kind(u8);

impl Kind {
	/// Destination Unreachable Message
	pub const DEST_UNREACHABLE: Self = Self(3);
	/// Time Exceeded Message
	pub const TIME_EXCEEDED: Self = Self(11);
	/// Parameter Problem Message
	pub const PARAM_PROBLEM: Self = Self(12);
	/// Source Quench Message
	pub const SRC_QUENCH: Self = Self(4);
	/// Redirect Message
	pub const REDIR: Self = Self(5);
	/// Echo Message
	pub const ECHO: Self = Self(8);
	/// Echo Reply Message
	pub const ECHO_REPLY: Self = Self(0);
}

impl From<u8> for Kind {
	fn from(value: u8) -> Self {
		Self(value)
	}
}

impl From<Kind> for u8 {
	fn from(val: Kind) -> Self {
		val.0
	}
}

/// ```
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |     Type      |     Code      |          Checksum             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             unused                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |      Internet Header + 64 bits of Original Data Datagram      |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
struct Packet {
	octets: [u8],
}

mod field {
	pub(crate) use crate::proto::field::*;

	pub(crate) const TYPE: Field = 0..1;
	pub(crate) const CODE: Field = 1..2;
	pub(crate) const CHECKSUM: Field = 2..4;
	pub(crate) const UNUSED: Field = 4..8;
}

impl Packet {
	#[inline(always)]
	pub(crate) fn type_(&self) -> u8 {
		field::parse_u8(&self.octets, field::TYPE)
	}

	#[inline(always)]
	pub(crate) fn code(&self) -> u8 {
		field::parse_u8(&self.octets, field::CODE)
	}

	#[inline(always)]
	pub(crate) fn checksum(&self) -> u16 {
		field::parse_u16(&self.octets, field::CHECKSUM)
	}
}
