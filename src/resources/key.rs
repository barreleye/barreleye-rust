use chrono::NaiveDateTime as DateTime;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Barreleye, Endpoint, Response};

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
	pub async fn create(client: &Barreleye) -> Response<Self> {
		client.post::<Self>(Method::POST, Endpoint::Keys, json!(null)).await
	}

	pub async fn list(client: &Barreleye) -> Response<Vec<Self>> {
		client.get::<Vec<Self>>(Endpoint::Keys, &[]).await
	}

	pub async fn get(client: &Barreleye, id: &str) -> Response<Self> {
		client.get::<Self>(Endpoint::Key(id.to_string()), &[]).await
	}

	pub async fn update(client: &Barreleye, id: &str, data: KeyData) -> Response<()> {
		client.post::<()>(Method::PUT, Endpoint::Key(id.to_string()), json!(data)).await
	}

	pub async fn delete(client: &Barreleye, id: &str) -> Response<()> {
		client.post::<()>(Method::DELETE, Endpoint::Key(id.to_string()), json!(null)).await
	}
}
