pub struct Barreleye {
	#[allow(dead_code)]
	url: String,
}

impl Barreleye {
	pub fn new(url: &str) -> Self {
		Self { url: url.to_string() }
	}
}
