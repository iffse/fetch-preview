use std::{collections::HashMap, io::Write, path::PathBuf};

use reqwest::Url;

pub async fn fetch_link(
	link: &String,
	output_dir: &String,
) -> Result<(), Box<dyn std::error::Error>> {
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

	let dir = std::fs::read_dir(output_dir);
	if dir.is_ok() {
		for entry in dir.unwrap() {
			let entry = entry.unwrap();
			let file_name = entry.file_name();
			let file_name = file_name.to_str().unwrap();
			let match_name = domain.to_owned() + " - " + id;
			if file_name.contains(&match_name) {
				println!("Image for {} already exists, skipping...", &link);
				return Ok(());
			}
		}
	}

	let mut link = link.to_owned();
	if !link.starts_with("http") {
		link.insert_str(0, "https://");
	}
	let url = Url::parse(&link);
	if url.is_err() {
		println!("Error {:?} for link {:?}", url.err().unwrap(), link);
		return Err(link.into());
	}

	let res = reqwest::get(url.unwrap()).await?.text().await;
	if res.is_err() {
		println!("Error {:?} for link {:?}", res.err().unwrap(), link);
		return Err(link.into());
	}

	let res = res.unwrap();
	let title = get_link_title(&res);
	let file_name = title + " " + &domain + " - " + id;
	let save = save_binary_file(&link, &res, output_dir, &file_name).await;
	if save.is_err() {
		println!("Error {:?}", save.err().unwrap());
		return Err(link.into());
	}
	Ok(())
}

pub async fn fetch_file(
	file: &String,
	output_dir: &String,
) -> Result<(), Box<dyn std::error::Error>> {
	let file = std::fs::read_to_string(file)?;
	let lines = file.lines();
	let mut error_links = Vec::new();

	for line in lines {
		let link = line.to_string();
		let fetch = fetch_link(&link, output_dir).await;
		if fetch.is_err() {
			error_links.push(line.to_string());
		}
	}
	if !error_links.is_empty() {
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
	file_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
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
	url = trim_special_part(url);
	println!("Downloading image from {}", url);

	let res = Url::parse(url);
	if res.is_err() {
		return Err((String::from("Error ")
			+ res.err().unwrap().to_string().as_str()
			+ " for link "
			+ link)
			.into());
	}
	let res = reqwest::get(res.unwrap()).await?.bytes().await?;

	let mut path = PathBuf::from(output_dir);
	path.push(file_name.to_owned() + find_extension_name(url));

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
	let prohibited_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
	let mut res = res.to_string();
	for c in prohibited_chars.iter() {
		res = res.replace(*c, " ");
	}

	let char_map = HashMap::from([
		("&#39;", "'"),
		("&amp;", "&"),
		("&quot;", "\""),
		("&lt;", "<"),
		("&gt;", ">"),
	]);
	for (key, value) in char_map.iter() {
		res = res.replace(key, value);
	}
	res
}

fn trim_special_part(url: &str) -> &str {
	let special_chars = ['@', '?'];
	for c in special_chars.iter() {
		if url.contains(*c) {
			return url.split_at(url.find(*c).unwrap()).0;
		}
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
