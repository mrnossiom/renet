use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
	pub network: EthConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum EthConfig {
	#[serde(rename = "static")]
	Static {
		addr: String,
		// subnet: Ipv4Addr,
		gateway: String,
	},
	#[serde(rename = "dhcp")]
	Dhcp { hostname: String },
}
