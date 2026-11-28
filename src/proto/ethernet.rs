use std::fmt;

use crate::proto::MacAddr;

#[derive(Debug)]
pub struct Error;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct EtherType(u16);

impl EtherType {
	/// Internet Protocol v4
	pub const IPV4: Self = Self(0x0800);
	/// Address Resolution Protocol
	pub const ARP: Self = Self(0x0806);
	/// Internet Protocol v6
	pub const IPV6: Self = Self(0x86DD);
}

impl From<u16> for EtherType {
	fn from(value: u16) -> Self {
		Self(value)
	}
}

impl From<EtherType> for u16 {
	fn from(val: EtherType) -> Self {
		val.0
	}
}

impl fmt::Debug for EtherType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::IPV4 => write!(f, "IPV4"),
			Self::ARP => write!(f, "ARP"),
			Self::IPV6 => write!(f, "IPV6"),
			Self(et) => write!(f, "EtherType({et:04X})"),
		}
	}
}

#[repr(transparent)]
pub(crate) struct Packet {
	octets: [u8],
}

mod field {
	pub(crate) use crate::proto::field::*;

	pub(crate) const DST_MAC: Field = 0..6;
	pub(crate) const SRC_MAC: Field = 6..12;
	pub(crate) const TAG: Field = 12..14;

	pub(crate) const HEADER_END: usize = 14;
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
	pub fn dst(&self) -> MacAddr {
		field::parse_bytes::<6>(&self.octets, field::DST_MAC).into()
	}

	#[inline(always)]
	pub fn src(&self) -> MacAddr {
		field::parse_bytes::<6>(&self.octets, field::SRC_MAC).into()
	}

	#[inline(always)]
	pub fn ether_type(&self) -> Option<EtherType> {
		let tag = field::parse_u16(&self.octets, field::TAG);
		match tag {
			..1500 => None,
			1500..1536 => None,
			1536.. => Some(EtherType(tag)),
		}
	}

	#[inline(always)]
	pub(crate) fn payload(&self) -> &[u8] {
		&self.octets[field::HEADER_END..]
	}
}

impl fmt::Debug for Packet {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut d = f.debug_struct("Packet");
		d.field("dst", &self.dst()).field("src", &self.src());
		if let Some(ether_type) = self.ether_type() {
			d.field("ether_type", &ether_type);
		}
		d.finish_non_exhaustive()
	}
}
