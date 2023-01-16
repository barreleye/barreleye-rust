use chrono::NaiveDateTime as DateTime;
use eyre::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Barreleye, Endpoint};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyData {
	is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
	id: String,
	key: String,
	is_active: bool,
	created_at: DateTime,
}

impl Key {
	pub async fn create(client: &Barreleye) -> Result<Self> {
		Ok(client
			.body_request(Method::POST, Endpoint::Keys, json!(null))
			.await?
			.json::<Self>()
			.await?)
	}

	pub async fn list(client: &Barreleye) -> Result<Vec<Self>> {
		Ok(client
			.body_request(Method::GET, Endpoint::Keys, json!(null))
			.await?
			.json::<Vec<Self>>()
			.await?)
	}

	pub async fn get(client: &Barreleye, id: &str) -> Result<Self> {
		Ok(client
			.body_request(Method::GET, Endpoint::Key(id.to_string()), json!(null))
			.await?
			.json::<Self>()
			.await?)
	}

	pub async fn update(client: &Barreleye, id: &str, data: KeyData) -> Result<()> {
		Ok(client
			.body_request(Method::PUT, Endpoint::Key(id.to_string()), json!(data))
			.await?
			.json::<()>()
			.await?)
	}

	pub async fn delete(client: &Barreleye, id: &str) -> Result<()> {
		Ok(client
			.body_request(Method::DELETE, Endpoint::Key(id.to_string()), json!(null))
			.await?
			.json::<()>()
			.await?)
	}
}
