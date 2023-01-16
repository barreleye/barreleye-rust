use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{Barreleye, Endpoint};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
	hash: String,
	from_address: String,
	to_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpstreamData {
	network: String,
	address: String,
	label: String,
	transactions: Vec<Transaction>,
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
pub struct Upstream {
	address: String,
	upstream: Vec<UpstreamData>,
	networks: Vec<Network>,
	labels: Vec<Label>,
}

impl Upstream {
	pub async fn get(client: &Barreleye, address: &str, detailed: Option<bool>) -> Result<Self> {
		Ok(client
			.get_request(
				Endpoint::Upstream,
				Some([
					("address", address),
					("detailed", if detailed.unwrap_or(false) { "true" } else { "false" }),
				]),
			)
			.await?
			.json::<Self>()
			.await?)
	}
}
