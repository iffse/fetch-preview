use std::env;
use std::collections::HashMap;

pub fn handle_arguments() -> HashMap<String, String> {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 {
		show_help();
	}

	let mut filename = String::new();
	let mut output_dir = String::new();

	let check_argument = |x: &usize| {
		let i = x + 1;
		if i >= args.len() {
			missing_arg(args[i - 1].as_str());
		}
		i
	};

	let mut n = 1;
	while n < args.len() {
		match args[n].as_str() {
			"-f" => {
				n = check_argument(&n);
				filename = args[n].clone();
			},
			"-o" => {
				n = check_argument(&n);
				output_dir = args[n].clone();
			},
			"-h" => show_help(),
			_ => unknown_arg(&args[n]),
		}
	};

	let mut map = HashMap::new();
	map.insert("filename".to_string(), filename);
	map.insert("output_dir".to_string(), output_dir);
	map
}



fn missing_arg(arg: &str) {
	println!("Missing argument for {}.", arg);
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
