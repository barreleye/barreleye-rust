use chrono::NaiveDateTime as DateTime;
use eyre::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Barreleye, Blockchain, Endpoint, Env};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkData {
	name: Option<String>,
	tag: Option<String>,
	env: Option<Env>,
	blockchain: Option<Blockchain>,
	chain_id: Option<u64>,
	block_time_ms: Option<u64>,
	rpc_endpoints: Option<Vec<String>>,
	rps: Option<u32>,
	is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
	name: String,
	tag: String,
	env: Env,
	blockchain: Blockchain,
	chain_id: u64,
	block_time_ms: u64,
	rpc_endpoints: Vec<String>,
	rps: u32,
	is_active: bool,
	created_at: DateTime,
}

impl Network {
	pub async fn create(
		client: &Barreleye,
		name: &str,
		tag: &str,
		env: Env,
		blockchain: Blockchain,
		chain_id: u64,
		block_time_ms: u64,
		rpc_endpoints: Vec<String>,
		rps: u32,
	) -> Result<Self> {
		Ok(client
			.body_request(
				Method::POST,
				Endpoint::Networks,
				json!({
					"name": name.to_string(),
					"tag": tag.to_string(),
					"env": env,
					"blockchain": blockchain,
					"chainId": chain_id,
					"blockTimeMs": block_time_ms,
					"rpcEndpoints": rpc_endpoints,
					"rps": rps,
				}),
			)
			.await?
			.json::<Self>()
			.await?)
	}

	pub async fn list(client: &Barreleye) -> Result<Vec<Self>> {
		Ok(client
			.body_request(Method::GET, Endpoint::Networks, json!(null))
			.await?
			.json::<Vec<Self>>()
			.await?)
	}

	pub async fn get(client: &Barreleye, id: &str) -> Result<Self> {
		Ok(client
			.body_request(Method::GET, Endpoint::Network(id.to_string()), json!(null))
			.await?
			.json::<Self>()
			.await?)
	}

	pub async fn update(client: &Barreleye, id: &str, data: NetworkData) -> Result<()> {
		Ok(client
			.body_request(Method::PUT, Endpoint::Network(id.to_string()), json!(data))
			.await?
			.json::<()>()
			.await?)
	}

	pub async fn delete(client: &Barreleye, id: &str) -> Result<()> {
		Ok(client
			.body_request(Method::DELETE, Endpoint::Network(id.to_string()), json!(null))
			.await?
			.json::<()>()
			.await?)
	}
}
