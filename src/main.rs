use std::io::{stdin, stdout};
use std::io::Write;

fn init() {
	// set up signal handlers
	ctrlc::set_handler(move || {
		std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

fn main() {
	init();

	loop {
        print!("> ");
        stdout().flush();
    	let mut input = String::new();
    	stdin().read_line(&mut input).unwrap();
    	println!("{}", input);

    	match input.as_str() {
    		"quit" => std::process::exit(0),
    		_ => println!("something else!"),
    	}
	}
}
