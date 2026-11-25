use clap::Parser;
use std::{ffi::CString, fs, io::Read, path::PathBuf, str::FromStr};

use crate::proto::{
	arp,
	ethernet::{self, EtherType},
	ipv4, ipv6,
};

mod config;
mod proto;
mod raw;
mod tap;

#[derive(Parser)]
struct Args {
	#[clap(long, default_value = "tap0")]
	device: CString,

	#[clap(long)]
	config: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let Args { device, config } = Args::parse();

	let config_content = fs::read_to_string(config).unwrap();
	let config = toml::from_str::<config::Config>(&config_content).unwrap();
	println!("{config:?}");

	let mut tap = tap::open(&device)?;

	loop {
		let mut buf = [0u8; u16::MAX as usize];
		let nread = tap.read(&mut buf)?;
		let link_frame = &buf[..nread];

		println!();

		let (eth_head, net_frame) = ethernet::Header::parse(link_frame).unwrap();
		println!("{eth_head:X?}");

		match eth_head.ether_type().unwrap() {
			EtherType::ARP => {
				let arp_frame = arp::Header::parse(net_frame).unwrap();
				println!("{arp_frame:X?}");
			}
			EtherType::IPV4 => {
				let (ipv4_header, transport_frame) = ipv4::Header::parse(net_frame).unwrap();
				println!("{ipv4_header:X?}");
			}
			EtherType::IPV6 => {
				let (ipv6_header, transport_frame) = ipv6::Header::parse(net_frame).unwrap();
				println!("{ipv6_header:X?}");
			}
			_ => todo!(),
		}
	}

	// Ok(())
}
