mod args;
mod fetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = args::handle_arguments();
	let file = args.get("file");
	let link = args.get("link");
	let output_dir = args.get("output_dir");
	let record = args.get("record").is_some();

	if link.is_some() {
		let _ = fetch::fetch_link(link.unwrap(), output_dir.unwrap()).await;
	}

	if file.is_some() {
		let _ = fetch::fetch_file(file.unwrap(), output_dir.unwrap(), record).await;
	}

	Ok(())
}
