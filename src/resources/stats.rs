use serde::{Deserialize, Serialize};

use crate::{Barreleye, Endpoint, Response};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
	name: String,
	tail_index: u64,
	block_height: u64,
	sync: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
	sync: f64,
	networks: Vec<Network>,
}

impl Stats {
	pub async fn get(client: &Barreleye) -> Response<Self> {
		client.get::<Self>(Endpoint::Stats, &[]).await
	}
}
