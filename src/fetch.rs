use std::{io::Write, path::PathBuf};

use reqwest::Url;

pub async fn fetch_link(
	link: &String,
	output_dir: &String,
) -> Result<(), Box<dyn std::error::Error>> {
	let url = Url::parse(link);
	if url.is_err() {
		println!("Error {:?} for link {:?}", url.err().unwrap(), link);
		return Err(link.clone().into());
	}

	let res = reqwest::get(url.unwrap()).await?.text().await;
	if res.is_err() {
		println!("Error {:?} for link {:?}", res.err().unwrap(), link);
		return Err(link.clone().into());
	}
	let res = res.unwrap();
	let save = save_binary_file(link, &res, output_dir).await;
	if save.is_err() {
		println!("Error {:?}", save.err().unwrap());
		return Err(link.clone().into());
	}
	Ok(())
}

pub async fn fetch_file(
	file: &String,
	output_dir: &String,
) -> Result<(), Box<dyn std::error::Error>> {
	let file = std::fs::read_to_string(file)?;
	let mut lines = file.lines();
	let mut error_links = Vec::new();
	while let Some(line) = lines.next() {
		let fetch = fetch_link(&line.to_string(), output_dir).await;
		if fetch.is_err() {
			println!("Error {:?}", fetch.err().unwrap());
			error_links.push(line.to_string());
		}
	}
	if error_links.len() > 0 {
		println!("Error links:");
		for link in error_links.iter() {
			println!("{}", link);
		}
	}
	Ok(())
}

async fn save_binary_file(
	link: &str,
	res: &str,
	output_dir: &String,
) -> Result<(), Box<dyn std::error::Error>> {
	let title = get_link_title(res);
	let link_bare = link.split_at(link.find("//").unwrap() + 2).1;
	let link_part = link_bare.split_at(link_bare.find('/').unwrap());
	let domain = link_part.0.to_string();
	let mut id = link_part.1;
	let prohibited_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
	for c in prohibited_chars.iter() {
		let index = id.rfind(*c);
		if index.is_none() {
			continue;
		}
		id = id.split_at(index.unwrap() + 1).1;
	}

	let mut url = <&str>::clone(&res);
	let og_image = String::from("property=\"og:image\" content=\"");
	let og_index = url.find(&og_image);
	if og_index.is_none() {
		return Err((String::from("No og:image found for link ") + link).into());
	}
	url = url.split_at(og_index.unwrap() + og_image.len()).1;
	let end_index = url.find("\">");
	if end_index.is_none() {
		return Err((String::from("No end of og:image found for link ") + link).into());
	}
	url = url.split_at(end_index.unwrap()).0;

	url = match url.contains("//") {
		true => url.split_at(url.find("//").unwrap() + 2).1,
		false => url,
	};
	let https_link = String::from("https://") + url;
	url = match url.contains("http") {
		true => url,
		false => https_link.as_str(),
	};
	url = handgle_image_link(link, url);
	println!("Downloading image from {}", url);

	let res = Url::parse(url);
	if res.is_err() {
		return Err((String::from("Error ") + res.err().unwrap().to_string().as_str() + " for link " + link).into());
	}
	let res = reqwest::get(res.unwrap()).await?.bytes().await?;

	let mut path = PathBuf::from(output_dir);
	path.push(title + " " + &domain + " - " + id + find_extension_name(url));

	std::fs::create_dir_all(path.parent().unwrap())?;
	let mut file = std::fs::File::create(path)?;
	file.write_all(&res)?;

	Ok(())
}

fn get_link_title(res: &str) -> String {
	let mut res = <&str>::clone(&res);
	let og_title = String::from("property=\"og:title\" content=\"");
	let og_index = res.find(&og_title);
	if og_index.is_none() {
		return String::from("");
	}
	res = res.split_at(og_index.unwrap() + og_title.len()).1;

	let end_index = res.find("\">");
	if end_index.is_none() {
		return String::from("");
	}
	res = res.split_at(end_index.unwrap()).0;
	res.to_string()
}

fn handgle_image_link<'a>(link: &'a str, url: &'a str) -> &'a str {
	if link.contains("bilibili") {
		return url.split_at(url.find('@').unwrap()).0;
	}
	url
}

fn find_extension_name(url: &str) -> &str {
	let extension_list = [".jpg", ".png", ".jpeg", ".gif", ".webp"];
	for extension in extension_list.iter() {
		if url.contains(extension) {
			return extension;
		}
	}
	".png"
}
