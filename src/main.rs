mod args;
mod fetch;

use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = args::handle_arguments();
	let file = args.get("file");
	let link = args.get("link");
	let output_dir = args.get("output_dir");

	if link.is_some() {
		fetch::fetch_link(link.unwrap(), output_dir.unwrap()).await?;
	}

	Ok(())
}
