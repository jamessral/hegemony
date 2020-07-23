use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn tester() {
	let contents = reader("../data/hello_world.heg");

	match contents {
		Ok(c) => println!("{}", c),
		_ => panic!("unable to read file")
	}
}

fn reader(file_path: &str) -> std::io::Result<String> {
	let file = File::open(file_path)?;
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents)?;

	Ok(contents)
}
