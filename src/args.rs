use std::collections::HashMap;
use std::env;

pub fn handle_arguments() -> HashMap<String, String> {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 {
		show_help();
	}
	let mut map = HashMap::new();
	let mut n = 1;
	while n < args.len() {
		match args[n].as_str() {
			"-f" | "--file" => {
				map.insert(String::from("file"), get_specifier(&mut n));
			}
			"-l" | "--link" => {
				map.insert(String::from("link"), get_specifier(&mut n));
			}
			"-o" | "--output_dir" => {
				map.insert(String::from("output_dir"), get_specifier(&mut n));
			}
			"-h" | "--help" => show_help(),
			_ => unknown_arg(&args[n]),
		}
	}
	if map.get("output_dir").is_none() {
		map.insert(String::from("output_dir"), String::from("./preview"));
	}
	map
}

fn get_specifier(i: &mut usize) -> String {
	let args: Vec<String> = env::args().collect();
	let arg_content = args.get(*i + 1);
	match arg_content {
		Some(arg) => {
			*i += 2;
			String::from(arg)
		}
		None => {
			println!("Missing specifier for {}.", args[*i]);
			std::process::exit(1);
		}
	}
}

fn unknown_arg(arg: &str) {
	println!("Unknown argument: {}. Use -h to show help.", arg);
	std::process::exit(1);
}

fn show_help() {
	println!(
		"
Usage: preview [OPTION]...
Fetches a preview image for a link or a list of links in a file.

Options:
	-f, --file FILE         Fetches a preview image for each link in FILE.
	-l, --link LINK         Fetches a preview image for LINK.
	-o, --output_dir DIR    Sets the output directory to DIR.
	                        (default: ./preview)
	-h, --help              Shows this help.
"
	);
	std::process::exit(0);
}
