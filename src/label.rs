use chrono::NaiveDateTime as DateTime;
use eyre::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Barreleye, Endpoint};

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
	pub async fn create(client: &Barreleye, name: &str, description: &str) -> Result<Self> {
		Ok(client
			.body_request(
				Method::POST,
				Endpoint::Labels,
				json!({
					"name": name.to_string(),
					"description": description.to_string(),
				}),
			)
			.await?
			.json::<Self>()
			.await?)
	}

	pub async fn list(client: &Barreleye) -> Result<Vec<Self>> {
		Ok(client
			.body_request(Method::GET, Endpoint::Labels, json!(null))
			.await?
			.json::<Vec<Self>>()
			.await?)
	}

	pub async fn get(client: &Barreleye, id: &str) -> Result<Self> {
		Ok(client
			.body_request(Method::GET, Endpoint::Label(id.to_string()), json!(null))
			.await?
			.json::<Self>()
			.await?)
	}

	pub async fn update(client: &Barreleye, id: &str, data: LabelData) -> Result<()> {
		Ok(client
			.body_request(Method::PUT, Endpoint::Label(id.to_string()), json!(data))
			.await?
			.json::<()>()
			.await?)
	}

	pub async fn delete(client: &Barreleye, id: &str) -> Result<()> {
		Ok(client
			.body_request(Method::DELETE, Endpoint::Label(id.to_string()), json!(null))
			.await?
			.json::<()>()
			.await?)
	}
}
