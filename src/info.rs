use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{Barreleye, Endpoint};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
	network: String,
	address: String,
	balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
	id: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
	id: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
	address: String,
	assets: Vec<Asset>,
	networks: Vec<Network>,
	labels: Vec<Label>,
}

impl Info {
	pub async fn get(client: &Barreleye, address: &str) -> Result<Self> {
		Ok(client
			.get_request(Endpoint::Info, Some([("address", address)]))
			.await?
			.json::<Self>()
			.await?)
	}
}
