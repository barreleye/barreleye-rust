use chrono::NaiveDateTime as DateTime;
use eyre::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::{Barreleye, Endpoint};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
	id: String,
	address: String,
	description: String,
	created_at: DateTime,
}

impl Address {
	pub async fn create(
		client: &Barreleye,
		label: &str,
		network: &str,
		addresses: HashMap<String, String>,
	) -> Result<Self> {
		Ok(client
			.body_request(
				Method::POST,
				Endpoint::Addresses,
				json!({
					"label": label.to_string(),
					"network": network.to_string(),
					"addresses": addresses,
				}),
			)
			.await?
			.json::<Self>()
			.await?)
	}

	pub async fn list(client: &Barreleye) -> Result<Vec<Self>> {
		Ok(client
			.body_request(Method::GET, Endpoint::Addresses, json!(null))
			.await?
			.json::<Vec<Self>>()
			.await?)
	}

	pub async fn get(client: &Barreleye, id: &str) -> Result<Self> {
		Ok(client
			.body_request(Method::GET, Endpoint::Address(id.to_string()), json!(null))
			.await?
			.json::<Self>()
			.await?)
	}

	pub async fn delete(client: &Barreleye, id: &str) -> Result<()> {
		Ok(client
			.body_request(Method::DELETE, Endpoint::Address(id.to_string()), json!(null))
			.await?
			.json::<()>()
			.await?)
	}
}
