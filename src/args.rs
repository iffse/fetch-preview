use std::env;
use std::collections::HashMap;

pub fn handle_arguments() -> HashMap<String, String> {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 {
		show_help();
	}
	let mut map = HashMap::new();
	let mut n = 1;
	while n < args.len() {
		match args[n].as_str() {
			"-f" => {
				map.insert(String::from("file"), get_specifier(&mut n));
			},
			"-o" => {
				map.insert(String::from("output_dir"), get_specifier(&mut n));
			},
			"-h" => show_help(),
			_ => unknown_arg(&args[n]),
		}
	};
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
		},
	}
}

fn unknown_arg(arg: &str) {
	println!("Unknown argument: {}. Use -h to show help.", arg);
	std::process::exit(1);
}

fn show_help() {
	println!("Usage: ");
	std::process::exit(0);
}
