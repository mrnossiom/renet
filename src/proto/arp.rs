use std::fmt;

#[derive(Debug)]
pub struct Error;

#[derive(PartialEq, Eq)]
pub(crate) struct Opcode(u16);

impl Opcode {
	pub const REQUEST: Self = Self(1);
	pub const REPLY: Self = Self(2);
}

impl fmt::Debug for Opcode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::REQUEST => write!(f, "Opcode::REQUEST"),
			Self::REPLY => write!(f, "Opcode::Reply"),
			Self(code) => write!(f, "Opcode({code:02X})"),
		}
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct AddressSpace(u16);

impl From<u16> for AddressSpace {
	fn from(value: u16) -> Self {
		Self(value)
	}
}

impl From<AddressSpace> for u16 {
	fn from(val: AddressSpace) -> Self {
		val.0
	}
}

#[repr(transparent)]
pub(crate) struct Packet {
	octets: [u8],
}

impl Packet {
	const HEADER_MIN_LEN: usize = 2 + 2 + 1 + 1;

	pub fn parse(octets: &[u8]) -> Result<&Self, Error> {
		if octets.len() < Self::HEADER_MIN_LEN {
			return Err(Error);
		}

		let octets = unsafe { std::mem::transmute::<&[u8], &Packet>(octets) };
		Ok(octets)
	}
}

impl Packet {
	/// Hardware address space
	#[inline]
	pub fn hrd(&self) -> AddressSpace {
		u16::from_be_bytes(*self.octets[0..2].as_array().unwrap()).into()
	}

	/// Protocol address space
	#[inline]
	pub fn pro(&self) -> AddressSpace {
		u16::from_be_bytes(*self.octets[2..4].as_array().unwrap()).into()
	}

	/// byte length of each hardware address
	#[inline]
	pub fn hln(&self) -> u8 {
		self.octets[4]
	}

	/// byte length of each protocol address
	#[inline]
	pub fn pln(&self) -> u8 {
		self.octets[5]
	}

	/// opcode
	pub fn op(&self) -> Opcode {
		Opcode(u16::from_be_bytes(*self.octets[6..8].as_array().unwrap()))
	}

	/// Hardware address of sender of this packet
	pub fn sha(&self) -> &[u8] {
		let start = 8;
		let end = start + self.hln() as usize;
		&self.octets[start..end]
	}

	/// Protocol address of sender of this packet
	pub fn spa(&self) -> &[u8] {
		let start = 8 + self.hln() as usize;
		let end = start + self.pln() as usize;
		&self.octets[start..end]
	}

	/// Hardware address of target of this packet
	pub fn tha(&self) -> &[u8] {
		let start = 8 + self.hln() as usize + self.pln() as usize;
		let end = start + self.hln() as usize;
		&self.octets[start..end]
	}

	/// Protocol address of target
	pub fn tpa(&self) -> &[u8] {
		let start = 8 + self.hln() as usize + self.pln() as usize + self.hln() as usize;
		let end = start + self.pln() as usize;
		&self.octets[start..end]
	}
}

impl fmt::Debug for Packet {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Frame")
			.field("hrd", &self.hrd())
			.field("pro", &self.pro())
			.field("hln", &self.hln())
			.field("prn", &self.pln())
			.field("op", &self.op())
			.field("sha", &self.sha())
			.field("spa", &self.spa())
			.field("tha", &self.tha())
			.field("tpa", &self.tpa())
			.finish()
	}
}
