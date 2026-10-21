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
			Self::IPV4 => write!(f, "EtherType::IPV4"),
			Self::ARP => write!(f, "EtherType::ARP"),
			Self::IPV6 => write!(f, "EtherType::IPV6"),
			Self(et) => write!(f, "EtherType({et:04X})"),
		}
	}
}

#[repr(transparent)]
pub(crate) struct Header {
	octets: [u8],
}

impl Header {
	const HEADER_LEN: usize = 14;
	// const CRC_LEN: usize = 4;

	pub fn parse(octets: &[u8]) -> Result<(&Self, &[u8]), Error> {
		if octets.len() < Self::HEADER_LEN {
			return Err(Error);
		}

		let (header, next_frame) = octets.split_at(Self::HEADER_LEN);
		// SAFETY: transparent cast
		let header = unsafe { std::mem::transmute::<&[u8], &Header>(header) };

		Ok((header, next_frame))
	}
}

impl Header {
	#[inline]
	pub fn dst(&self) -> MacAddr {
		(*self.octets[0..6].as_array::<6>().unwrap()).into()
	}

	#[inline]
	pub fn src(&self) -> MacAddr {
		(*self.octets[6..12].as_array::<6>().unwrap()).into()
	}

	#[inline]
	pub fn ether_type(&self) -> Option<EtherType> {
		let tag = u16::from_be_bytes(*self.octets[12..14].as_array().unwrap());
		match tag {
			..1500 => None,
			1500..1536 => None,
			1536.. => Some(EtherType(tag)),
		}
	}
}

impl fmt::Debug for Header {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut d = f.debug_struct("Frame");
		d.field("dst", &self.dst()).field("src", &self.src());
		if let Some(ether_type) = self.ether_type() {
			d.field("ether_type", &ether_type);
		}
		d.finish_non_exhaustive()
	}
}
