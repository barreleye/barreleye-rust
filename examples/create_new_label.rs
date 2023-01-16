use barreleye_client::{Barreleye, Label};

#[tokio::main]
async fn main() {
	// Define the client
	let url = "http://localhost:22775";
	let api_key = Some("7f9e9182-122d-45e1-b4be-d73fc99e9bc9");
	let client = Barreleye::new(url, api_key);

	// Create a new label
	let label = Label::create(&client, "Label #1", "Sample label").await.unwrap();
	println!("{:?}", label);
}
