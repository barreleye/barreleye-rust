use serde::{Deserialize, Serialize};

use crate::{Barreleye, Endpoint, Response};

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
	pub async fn get(client: &Barreleye, address: &str) -> Response<Self> {
		client.get::<Self>(Endpoint::Info, &[("address", address)]).await
	}
}
