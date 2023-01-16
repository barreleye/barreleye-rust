use barreleye_client::{Barreleye, Info};

#[tokio::main]
async fn main() {
	// Define the client
	let url = "http://localhost:22775";
	let api_key = None;
	let client = Barreleye::new(url, api_key);

	// Get info about the address
	let info = Info::get(&client, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa").await.unwrap();
	println!("{:?}", info);
}
