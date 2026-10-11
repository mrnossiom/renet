use clap::Parser;
use nix::{
	ioctl_write_ptr_bad,
	libc::{self, ifreq},
};
use std::{
	fs,
	io::Read,
	mem::{MaybeUninit, transmute},
	os::fd::AsRawFd,
	path::PathBuf,
};
use zerocopy::FromBytes;

mod config;
mod raw;

ioctl_write_ptr_bad!(tun_set_interface, libc::TUNSETIFF, ifreq);

#[derive(Debug)]
pub(crate) enum EtherType {
	/// Internet Protocol v4
	IPv4,
	/// Address Resolution Protocol
	Arp,
	/// Internet Protocol v6
	IPv6,
}

impl TryFrom<u16> for EtherType {
	type Error = ();
	fn try_from(value: u16) -> Result<Self, Self::Error> {
		match value {
			0x0800 => Ok(Self::IPv4),
			0x0806 => Ok(Self::Arp),
			0x86DD => Ok(Self::IPv6),
			_ => Err(()),
		}
	}
}

impl From<EtherType> for u16 {
	fn from(val: EtherType) -> Self {
		match val {
			EtherType::IPv4 => 0x0800,
			EtherType::Arp => 0x0806,
			EtherType::IPv6 => 0x86DD,
		}
	}
}

#[derive(Parser)]
struct Args {
	#[clap(long)]
	config: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let Args { config } = Args::parse();

	let config_content = fs::read_to_string(config).unwrap();
	let config = toml::from_str::<config::Config>(&config_content).unwrap();
	println!("{config:?}");

	let mut file = fs::OpenOptions::new()
		.read(true)
		.write(true)
		// .custom_flags(libc::O_NONBLOCK)
		.open("/dev/net/tun")?;

	let iff_name = c"tap0";

	unsafe {
		let mut ifreq = MaybeUninit::<ifreq>::zeroed().assume_init();
		let raw_name = transmute::<&[u8], &[i8]>(iff_name.to_bytes_with_nul());
		ifreq.ifr_name[..raw_name.len()].clone_from_slice(raw_name);

		ifreq.ifr_ifru.ifru_flags |= (libc::IFF_TAP | libc::IFF_NO_PI) as i16;

		tun_set_interface(file.as_raw_fd(), &raw const ifreq).unwrap();
	}

	loop {
		let mut buf = [0u8; u16::MAX as usize];
		let nread = file.read(&mut buf).unwrap();
		let raw_frame = &buf[..nread];

		let eth_frame = raw::EthernetPacket::ref_from_bytes(raw_frame).unwrap();
		println!("{eth_frame:?}");

		let ether_type: EtherType = eth_frame.ether_type.as_raw().try_into().unwrap();
		println!("{ether_type:?}");
	}

	// Ok(())
}
