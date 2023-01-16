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
pub struct Heartbeat {}

impl Heartbeat {
	pub async fn get(client: &Barreleye) -> Result<bool> {
		Ok(client.get_request(Endpoint::Heartbeat, Option::<()>::None).await?.status().is_success())
	}
}
