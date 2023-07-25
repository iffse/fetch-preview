mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	args::handle_arguments();

	let res = reqwest::get("http://url.com")
		.await?
		.text()
		.await?;

	println!("{:#?}", res);
	Ok(())
}
