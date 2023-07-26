mod args;
mod fetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = args::handle_arguments();
	let file = args.get("file");
	let link = args.get("link");
	let output_dir = args.get("output_dir");

	if link.is_some() {
		fetch::fetch_link(link.unwrap(), output_dir.unwrap()).await?;
	}

	if file.is_some() {
		fetch::fetch_file(file.unwrap(), output_dir.unwrap()).await?;
	}

	Ok(())
}
