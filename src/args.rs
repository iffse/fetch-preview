use std::env;
use std::collections::HashMap;

pub fn handle_arguments() -> HashMap<String, String> {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 {
		show_help();
	}

	let mut filename = String::new();
	let mut output_dir = String::new();

	let mut n = 1;
	while n < args.len() {
		match args[n].as_str() {
			"-f" => {
				filename = get_specifier(&mut n);
			},
			"-o" => {
				output_dir = get_specifier(&mut n);
			},
			"-h" => show_help(),
			_ => unknown_arg(&args[n]),
		}
	};

	let mut map = HashMap::new();
	map.insert(String::from("filename"), filename);
	map.insert(String::from("output_dir"), output_dir);
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
			missing_specifier(&args[*i]);
			String::new()
		},
	}
}

fn missing_specifier(arg: &str) {
	println!("Missing specifier for {}.", arg);
	std::process::exit(1);
}

fn unknown_arg(arg: &str) {
	println!("Unknown argument: {}. Use -h to show help.", arg);
	std::process::exit(1);
}

fn show_help() {
	println!("Usage: ");
	std::process::exit(0);
}
