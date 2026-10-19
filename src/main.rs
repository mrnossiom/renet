use clap::Parser;
use std::{fs, io::Read, path::PathBuf};

use crate::proto::{
	arp,
	ethernet::{self, EtherType},
};

mod config;
mod proto;
mod raw;
mod tap;

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

	let mut tap = tap::open_tap(c"tap0")?;

	loop {
		let mut buf = [0u8; u16::MAX as usize];
		let nread = tap.read(&mut buf)?;
		let raw_frame = &buf[..nread];

		let eth_frame = ethernet::Frame::parse(raw_frame).unwrap();
		println!("{eth_frame:?}");
		let ether_type = eth_frame.ether_type().unwrap();
		println!("{ether_type:?}");

		match ether_type {
			EtherType::ARP => {
				let arp_frame = arp::Frame::parse(eth_frame.payload()).unwrap();
				println!("{arp_frame:?}");
			}
			EtherType::IPV4 => {}
			EtherType::IPV6 => {}
			_ => todo!(),
		}
	}

	// Ok(())
}
