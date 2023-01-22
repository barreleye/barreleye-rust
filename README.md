# barreleye.rs

The official Rust driver for Barreleye.

[![Status beta](https://img.shields.io/badge/status-beta-ff69b4.svg?style=flat-square)](https://github.com/barreleye/barreleye.rs)
[![Docs](https://img.shields.io/badge/docs-view-44cc11.svg?style=flat-square)](https://docs.rs/crate/barreleye-client/latest)
[![Github Actions](https://img.shields.io/github/actions/workflow/status/barreleye/barreleye.rs/tests.yml?style=flat-square)](https://github.com/barreleye/barreleye.rs/actions)
[![Dependency Status](https://deps.rs/repo/github/barreleye/barreleye.rs/status.svg?style=flat-square)](https://deps.rs/repo/github/barreleye/barreleye.rs)
[![Crates.io](https://img.shields.io/crates/v/barreleye-client?color=brightgreen&style=flat-square)](https://crates.io/crates/barreleye-client)
[![Downloads](https://img.shields.io/crates/d/barreleye-client?color=blue&style=flat-square)](https://crates.io/crates/barreleye-client)
[![License](https://img.shields.io/badge/license-Apache_License_2.0-blue.svg?style=flat-square)](https://github.com/barreleye/barreleye.rs)

## Usage

Add to `Cargo.toml`:

```bash
cargo add barreleye-client
```

Programmatically add a network so Barreleye can start indexing:

```rust
use barreleye_client::{Barreleye, Blockchain, Env, Network};

#[tokio::main]
async fn main() {
	// Define the client
	let url = "http://localhost:22775";
	let api_key = Some("7f9e9182-122d-45e1-b4be-d73fc99e9bc9");
	let client = Barreleye::new(url, api_key);

	// Create a new network resource
	let rpc_endpoint = "http://127.0.0.1:8545";
	let network = Network::create(
		&client,
		"Ethereum",
		Env::Mainnet,
		Blockchain::Evm,
		1,                  // chain id
		12_000,             // block time in milliseconds
		vec![rpc_endpoint], // rpc endpoints
		100,                // indexing rate limiter (requests per second)
	)
	.await;

	println!("{:?}", network);
}
```

Get info about an address:

```rust
use barreleye_client::{Barreleye, Error, Info};

#[tokio::main]
async fn main() {
	// Define the client
	let url = "http://localhost:22775";
	let api_key = None;
	let client = Barreleye::new(url, api_key);

	// Get info about the address
	match Info::get(&client, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa").await {
		Ok(info) => println!("{:?}", info),
		Err(Error::Unavailable) => println!("Is Barreleye server running?"),
		Err(e) => println!("Error: {e}"),
	}
}
```

Check out more [examples](/examples).

## License

[Apache-2.0](/LICENSE)