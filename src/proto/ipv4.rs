use std::{fmt, net::Ipv4Addr};

#[derive(Debug)]
pub struct Error;

/// # Layout
///
/// ```
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |Version|  IHL  |Type of Service|          Total Length         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |         Identification        |Flags|      Fragment Offset    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |  Time to Live |    Protocol   |         Header Checksum       |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                       Source Address                          |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                    Destination Address                        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                    Options                    |    Padding    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[repr(transparent)]
pub(crate) struct Packet {
	octets: [u8],
}

mod field {
	pub(crate) use crate::proto::field::*;

	// version
	// ihl
	pub(crate) const TYPE_OF_SERVICE: Field = 0..1;
	pub(crate) const TOTAL_LENGTH: Field = 2..4;
	pub(crate) const IDENTIFICATION: Field = 4..6;
	// flags, fragment offset
	pub(crate) const TTL: Field = 8..9;
	pub(crate) const PROTOCOL: Field = 9..10;
	pub(crate) const CHECKSUM: Field = 10..12;
	pub(crate) const SOURCE_ADDRESS: Field = 12..16;
	pub(crate) const DESTINATION_ADDRESS: Field = 16..20;
	pub(crate) const OPTIONS: Field = 20..23;
	// padding

	pub(crate) const HEADER_END: usize = 24;
}

impl Packet {
	pub fn parse(octets: &[u8]) -> Result<&Self, Error> {
		if octets.len() < field::HEADER_END {
			return Err(Error);
		}

		// SAFETY: transparent cast
		let packet = unsafe { std::mem::transmute::<&[u8], &Packet>(octets) };

		Ok(packet)
	}
}

impl Packet {
	#[inline(always)]
	pub(crate) fn length(&self) -> u16 {
		field::parse_u16(&self.octets, field::TOTAL_LENGTH)
	}

	#[inline(always)]
	pub(crate) fn ttl(&self) -> u8 {
		field::parse_u8(&self.octets, field::TTL)
	}

	#[inline(always)]
	pub(crate) fn protocol(&self) -> Protocol {
		let proto = field::parse_u8(&self.octets, field::PROTOCOL);
		Protocol::from(proto)
	}

	#[inline(always)]
	pub(crate) fn checksum(&self) -> u16 {
		field::parse_u16(&self.octets, field::CHECKSUM)
	}

	#[inline(always)]
	pub(crate) fn src(&self) -> Ipv4Addr {
		field::parse_bytes::<4>(&self.octets, field::SOURCE_ADDRESS).into()
	}

	#[inline(always)]
	pub(crate) fn dst(&self) -> Ipv4Addr {
		field::parse_bytes::<4>(&self.octets, field::DESTINATION_ADDRESS).into()
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
			.field("ttl", &self.ttl())
			.field("proto", &self.protocol())
			.field("chksum", &self.checksum())
			.finish_non_exhaustive()
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct Protocol(u8);

/// See <https://www.iana.org/assignments/protocol-numbers>
impl Protocol {
	/// Internet Control Message Protocol
	pub const ICMP: Self = Self(1);
	/// Internet Group Management Protocol
	pub const IGMP: Self = Self(2);
	/// Transmission Control Protocol
	pub const TCP: Self = Self(6);
	/// User Datagram Protocol
	pub const UDP: Self = Self(17);
}

impl From<u8> for Protocol {
	fn from(value: u8) -> Self {
		Self(value)
	}
}

impl From<Protocol> for u8 {
	fn from(val: Protocol) -> Self {
		val.0
	}
}

impl fmt::Debug for Protocol {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::ICMP => write!(f, "ICMP"),
			Self::IGMP => write!(f, "IGMP"),
			Self::TCP => write!(f, "TCP"),
			Self::UDP => write!(f, "UDP"),
			Self(proto) => write!(f, "Protocol({proto})"),
		}
	}
}
