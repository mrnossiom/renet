use std::{fmt, net::Ipv6Addr};

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
pub(crate) struct Header {
	octets: [u8],
}

impl Header {
	const HEADER_LEN: usize = 40;

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
	pub(crate) fn src(&self) -> Ipv6Addr {
		(*self.octets[8..24].as_array().unwrap()).into()
	}

	pub(crate) fn dst(&self) -> Ipv6Addr {
		(*self.octets[24..40].as_array().unwrap()).into()
	}
}

impl fmt::Debug for Header {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Header")
			.field("src", &self.src())
			.field("dst", &self.dst())
			.finish_non_exhaustive()
	}
}
