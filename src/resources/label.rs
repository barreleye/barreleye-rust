use chrono::NaiveDateTime as DateTime;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Barreleye, Endpoint, Response};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelData {
	name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
	id: String,
	name: String,
	description: String,
	created_at: DateTime,
}

impl Label {
	pub async fn create(client: &Barreleye, name: &str, description: &str) -> Response<Self> {
		client
			.post::<Self>(
				Method::POST,
				Endpoint::Labels,
				json!({
					"name": name.to_string(),
					"description": description.to_string(),
				}),
			)
			.await
	}

	pub async fn list(client: &Barreleye) -> Response<Vec<Self>> {
		client.get::<Vec<Self>>(Endpoint::Labels, &[]).await
	}

	pub async fn get(client: &Barreleye, id: &str) -> Response<Self> {
		client.get::<Self>(Endpoint::Label(id.to_string()), &[]).await
	}

	pub async fn update(client: &Barreleye, id: &str, data: LabelData) -> Response<()> {
		client.post::<()>(Method::PUT, Endpoint::Label(id.to_string()), json!(data)).await
	}

	pub async fn delete(client: &Barreleye, id: &str) -> Response<()> {
		client.post::<()>(Method::DELETE, Endpoint::Label(id.to_string()), json!(null)).await
	}
}
