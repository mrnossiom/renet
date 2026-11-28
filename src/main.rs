use clap::Parser;
use std::{ffi::CString, fs, io::Read, path::PathBuf};

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

		let eth_packet = ethernet::Packet::parse(link_frame).unwrap();
		println!("{eth_packet:?}");

		match eth_packet.ether_type().unwrap() {
			EtherType::ARP => {
				let arp_packet = arp::Packet::parse(eth_packet.payload()).unwrap();
				println!("{arp_packet:?}");
			}
			EtherType::IPV4 => {
				let ipv4_packet = ipv4::Packet::parse(eth_packet.payload()).unwrap();
				println!("{ipv4_packet:?}");
			}
			EtherType::IPV6 => {
				let ipv6_packet = ipv6::Packet::parse(eth_packet.payload()).unwrap();
				println!("{ipv6_packet:?}");
			}
			_ => todo!(),
		}
	}

	// Ok(())
}
