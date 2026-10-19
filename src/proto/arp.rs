use std::fmt;

#[derive(Debug)]
pub struct Error;

#[repr(transparent)]
pub(crate) struct AddressSpace {
	octets: [u8; 2],
}

#[repr(C)]
pub(crate) struct Frame<'a> {
	octets: &'a [u8],
}

impl<'a> Frame<'a> {
	pub fn parse(octets: &'a [u8]) -> Result<Self, Error> {
		Ok(Self { octets })
	}
}

impl Frame<'_> {
	/// Hardware address space
	#[inline]
	pub fn hrd(&self) -> AddressSpace {
		todo!()
	}

	/// Protocol address space
	#[inline]
	pub fn pro(&self) -> AddressSpace {
		todo!()
	}

	/// byte length of each hardware address
	#[inline]
	pub fn hln(&self) -> u8 {
		todo!()
	}

	/// byte length of each protocol address
	#[inline]
	pub fn pln(&self) -> u8 {
		todo!()
	}

	/// opcode
	pub fn op(&self) -> u16 {
		todo!()
	}

	pub fn sha(&self) -> &[u8] {
		todo!()
	}
	pub fn spa(&self) -> &[u8] {
		todo!()
	}
	pub fn tha(&self) -> &[u8] {
		todo!()
	}
	pub fn tpa(&self) -> &[u8] {
		todo!()
	}
}

impl fmt::Debug for Frame<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Frame").finish_non_exhaustive()
	}
}
