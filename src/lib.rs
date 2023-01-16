use derive_more::Display;
use eyre::Result;
use reqwest::{Client, Method, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub use address::Address;
pub use heartbeat::Heartbeat;
pub use info::Info;
pub use key::Key;
pub use label::Label;
pub use network::Network;
pub use stats::Stats;
pub use upstream::Upstream;

mod address;
mod heartbeat;
mod info;
mod key;
mod label;
mod network;
mod stats;
mod upstream;

#[derive(Display, Debug, Clone, PartialEq, Eq)]
enum Endpoint {
	#[display(fmt = "addresses")]
	Addresses,
	#[display(fmt = "addresses/{}", "_0")]
	Address(String),
	#[display(fmt = "heartbeat")]
	Heartbeat,
	#[display(fmt = "info")]
	Info,
	#[display(fmt = "keys")]
	Keys,
	#[display(fmt = "keys/{}", "_0")]
	Key(String),
	#[display(fmt = "labels")]
	Labels,
	#[display(fmt = "labels/{}", "_0")]
	Label(String),
	#[display(fmt = "networks")]
	Networks,
	#[display(fmt = "networks/{}", "_0")]
	Network(String),
	#[display(fmt = "stats")]
	Stats,
	#[display(fmt = "upstream")]
	Upstream,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Env {
	#[serde(rename = "localhost")]
	Localhost = 1,
	#[serde(rename = "testnet")]
	Testnet = 2,
	#[serde(rename = "mainnet")]
	#[default]
	Mainnet = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Blockchain {
	#[serde(rename = "bitcoin")]
	Bitcoin = 1,
	#[serde(rename = "evm")]
	Evm = 2,
}

pub struct Barreleye {
	version: String,
	url: String,
	api_key: Option<String>,
}

impl Barreleye {
	pub fn new(url: &str, api_key: Option<&str>) -> Self {
		Self {
			version: "0".to_string(),
			url: url.trim_end_matches('/').to_string(),
			api_key: api_key.map(|v| v.to_owned()),
		}
	}

	fn is_auth_request(&self, endpoint: Endpoint) -> bool {
		endpoint != Endpoint::Info && endpoint != Endpoint::Upstream
	}

	async fn get_request<T: Serialize>(
		&self,
		endpoint: Endpoint,
		query: Option<T>,
	) -> Result<Response> {
		let req = self.request_builder(Method::GET, endpoint);
		Ok(req.query(&query).send().await?)
	}

	async fn body_request(
		&self,
		method: Method,
		endpoint: Endpoint,
		body: JsonValue,
	) -> Result<Response> {
		let req = self.request_builder(method, endpoint);
		Ok(req.json(&body).send().await.unwrap())
	}

	fn request_builder(&self, method: Method, endpoint: Endpoint) -> RequestBuilder {
		let client = Client::new();

		let mut req =
			client.request(method, format!("{}/v{}/{}", self.url, self.version, endpoint));

		if self.is_auth_request(endpoint) {
			req = req.bearer_auth(self.api_key.clone().unwrap());
		}

		req
	}
}
