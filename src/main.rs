use structopt::StructOpt;
use std::collections::{HashMap, VecDeque};
use std::io;

pub mod intcode;

#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn parse_int_list(list: &mut String) -> Vec<i64> {
	list.retain(|c| c.is_digit(10) || c == ',' || c == '-');

	list
		.split(',')
		.map(|s| if s.is_empty() {0} else {s.parse::<i64>().unwrap()})
		.collect()
}

fn read_program(path: &std::path::PathBuf) -> HashMap<i64, i64> {
	let mut content = std::fs::read_to_string(path).expect("Failed to read file");
	let program = parse_int_list(&mut content);
	let mut memory = HashMap::new();
	for (i, x) in program.iter().enumerate() {
		// I don't know why x needs to be dereferenced here, but whatever
		memory.insert(i as i64, *x);
	}
	memory
}

fn read_input() -> Vec<i64> {
	let mut input = String::new();

	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input");
	
	parse_int_list(&mut input)
}

fn main() {
	let args = Cli::from_args();

	let memory = read_program(&args.path);
	let input: Vec<i64> = read_input();

	// println!("{:?}", input);
	// println!("{:?}", memory);

	let mut vm = intcode::IntVM::new(memory, VecDeque::from(input));

	let output = vm.run();

	println!("{:?}", output);
}
