use std::{fmt, net::Ipv6Addr};

use crate::proto::ipv4::Protocol;

#[derive(Debug)]
pub struct Error;

/// # Layout
///
/// ```
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |Version| Traffic Class |           Flow Label                  |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |         Payload Length        |  Next Header  |   Hop Limit   |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                         Source Address                        +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                      Destination Address                      +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[repr(transparent)]
pub(crate) struct Packet {
	octets: [u8],
}

mod field {
	pub(crate) use crate::proto::field::*;

	pub(crate) const PROTOCOL: Field = 6..7;
	pub(crate) const SOURCE_ADDRESS: Field = 8..24;
	pub(crate) const DESTINATION_ADDRESS: Field = 24..40;

	pub(crate) const HEADER_END: usize = 40;
}

impl Packet {
	pub fn parse(octets: &[u8]) -> Result<&Self, Error> {
		if octets.len() < field::HEADER_END {
			return Err(Error);
		}

		// SAFETY: transparent cast
		let header = unsafe { std::mem::transmute::<&[u8], &Packet>(octets) };

		Ok(header)
	}
}

impl Packet {
	#[inline(always)]
	pub(crate) fn src(&self) -> Ipv6Addr {
		field::parse_bytes::<16>(&self.octets, field::SOURCE_ADDRESS).into()
	}

	#[inline(always)]
	pub(crate) fn dst(&self) -> Ipv6Addr {
		field::parse_bytes::<16>(&self.octets, field::DESTINATION_ADDRESS).into()
	}

	#[inline(always)]
	pub(crate) fn protocol(&self) -> Protocol {
		let proto = field::parse_u8(&self.octets, field::PROTOCOL);
		Protocol::from(proto)
	}

	#[inline(always)]
	pub(crate) fn payload(&self) -> &[u8] {
		&self.octets[field::HEADER_END..]
	}
}

impl fmt::Debug for Packet {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Header")
			.field("src", &self.src())
			.field("dst", &self.dst())
			.field("protocol", &self.protocol())
			.finish_non_exhaustive()
	}
}
