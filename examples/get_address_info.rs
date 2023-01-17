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
