use std::fmt;

use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

#[derive(KnownLayout, Immutable, FromBytes, Unaligned)]
#[repr(transparent)]
pub(crate) struct MacAddr {
	octets: [u8; 6],
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

#[derive(KnownLayout, Immutable, FromBytes, Unaligned)]
#[repr(transparent)]
pub(crate) struct EtherType {
	octets: [u8; 2],
}

impl fmt::Debug for EtherType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "EtherType({:04X})", self.as_raw())
	}
}

impl EtherType {
	pub fn as_raw(&self) -> u16 {
		u16::from_be_bytes(self.octets)
	}
}

#[derive(KnownLayout, Immutable, FromBytes, Unaligned)]
#[repr(C)]
pub(crate) struct EthernetPacket {
	pub dst: MacAddr,
	pub src: MacAddr,
	pub ether_type: EtherType,
	pub data: [u8],
}

impl fmt::Debug for EthernetPacket {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Ethernet")
			.field("dst", &self.dst)
			.field("src", &self.src)
			.field("ether_type", &self.ether_type)
			.finish_non_exhaustive()
	}
}

mod arp {
	use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

	#[derive(KnownLayout, Immutable, FromBytes, Unaligned)]
	#[repr(transparent)]
	pub(crate) struct AddressSpace {
		octets: [u8; 2],
	}

	#[derive(KnownLayout, Immutable, FromBytes, Unaligned)]
	#[repr(C)]
	pub(crate) struct ArpPacket {
		/// Hardware address space
		pub hrd: AddressSpace,
		/// Protocol address space
		pub pro: AddressSpace,
		/// byte length of each hardware address
		pub hln: u8,
		/// byte length of each protocol address
		pub pln: u8,
		/// opcode
		pub op: [u8; 2],
		// pub sha: [u8],
		// pub spa: [u8],
		// pub tha: [u8],
		// pub tpa: [u8],
	}
}
