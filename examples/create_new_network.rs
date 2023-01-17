use barreleye_client::{Barreleye, Blockchain, Env, Network};

#[tokio::main]
async fn main() {
	// Define the client
	let url = "http://localhost:22775";
	let api_key = Some("7f9e9182-122d-45e1-b4be-d73fc99e9bc9");
	let client = Barreleye::new(url, api_key);

	// Create a new network
	let rpc_endpoint = "http://127.0.0.1:8545";
	let network = Network::create(
		&client,
		"Ethereum",         // name
		"Ethereum",         // tag
		Env::Mainnet,       // env
		Blockchain::Evm,    // blockchain
		1,                  // chain id
		12_000,             // block time in milliseconds
		vec![rpc_endpoint], // rpc endpoints
		100,                // rate limiter (requests per second)
	)
	.await;

	println!("{:?}", network);
}
