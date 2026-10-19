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

impl Into<u16> for EtherType {
	fn into(self) -> u16 {
		self.0
	}
}

impl fmt::Debug for EtherType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "EtherType({:04X})", self.0)
	}
}

#[repr(C)]
pub(crate) struct Frame<'a> {
	octets: &'a [u8],
}

impl<'a> Frame<'a> {
	// layout [6 (dst)|6 (src)|]
	const HEADER_LEN: usize = 14;
	const CRC_LEN: usize = 4;

	pub fn parse(octets: &'a [u8]) -> Result<Self, Error> {
		if octets.len() < 64 {
			return Err(Error);
		}

		Ok(Self { octets })
	}
}

impl Frame<'_> {
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

	#[inline]
	pub fn payload(&self) -> &[u8] {
		let start = Self::HEADER_LEN;
		let end = self.octets.len() - Self::CRC_LEN;
		&self.octets[start..end]
	}
}

impl fmt::Debug for Frame<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Frame")
			.field("dst", &self.dst())
			.field("src", &self.src())
			.field("ether_type", &self.ether_type())
			.finish_non_exhaustive()
	}
}
