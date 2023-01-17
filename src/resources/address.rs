use chrono::NaiveDateTime as DateTime;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::{Barreleye, Endpoint, Response};

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
	) -> Response<Self> {
		client
			.post::<Self>(
				Method::POST,
				Endpoint::Addresses,
				json!({
					"label": label.to_string(),
					"network": network.to_string(),
					"addresses": addresses,
				}),
			)
			.await
	}

	pub async fn list(client: &Barreleye) -> Response<Vec<Self>> {
		client.get::<Vec<Self>>(Endpoint::Addresses, &[]).await
	}

	pub async fn get(client: &Barreleye, id: &str) -> Response<Self> {
		client.get::<Self>(Endpoint::Address(id.to_string()), &[]).await
	}

	pub async fn delete(client: &Barreleye, id: &str) -> Response<()> {
		client.post::<()>(Method::DELETE, Endpoint::Address(id.to_string()), json!(null)).await
	}
}
