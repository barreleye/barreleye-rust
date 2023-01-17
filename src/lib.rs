use derive_more::Display;
use reqwest::{Client, Method, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub use error::{Error, RequestError};
pub use resources::*;

mod error;
mod resources;

pub type Response<T> = Result<T, Error>;

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
			version: "v0".to_string(),
			url: url.trim_end_matches('/').to_string(),
			api_key: api_key.map(|v| v.to_owned()),
		}
	}

	fn is_auth_request(&self, endpoint: Endpoint) -> bool {
		endpoint != Endpoint::Info && endpoint != Endpoint::Upstream
	}

	async fn get<T: DeserializeOwned>(
		&self,
		endpoint: Endpoint,
		query: &[(&str, &str)],
	) -> Response<T> {
		self.req::<T>(self.make(Method::GET, endpoint).query(&query)).await
	}

	async fn post<T: DeserializeOwned>(
		&self,
		method: Method,
		endpoint: Endpoint,
		body: JsonValue,
	) -> Response<T> {
		self.req::<T>(self.make(method, endpoint).json(&body)).await
	}

	fn make(&self, method: Method, endpoint: Endpoint) -> RequestBuilder {
		let url = format!("{}/{}/{}", self.url, self.version, endpoint);
		let mut req = Client::new().request(method, url);

		if self.is_auth_request(endpoint) {
			req = req.bearer_auth(self.api_key.clone().unwrap());
		}

		req
	}

	async fn req<T: DeserializeOwned>(&self, req: RequestBuilder) -> Response<T> {
		let res = req.send().await.map_err(|_| Error::Unavailable)?;
		if res.status().is_success() {
			Ok(res.json::<T>().await?)
		} else {
			Err(Error::Barreleye(res.json::<RequestError>().await?))
		}
	}
}
