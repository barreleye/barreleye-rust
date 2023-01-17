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
pub struct Heartbeat {}

impl Heartbeat {
	pub async fn get(client: &Barreleye) -> Response<()> {
		client.get::<()>(Endpoint::Heartbeat, &[]).await
	}
}
