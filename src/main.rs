use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::Write;

use structopt::StructOpt;

mod intcode;

#[derive(StructOpt)]
struct Cli {
	/// Input program
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,

	#[structopt(
		name = "initial-input",
		short = "I",
		long = "input",
		use_delimiter = true
	)]
	first_in: Vec<i64>,

	#[structopt(
		short = "i",
		long = "input-file",
		parse(from_os_str)
	)]
	input_file: Option<std::path::PathBuf>,

	#[structopt(
		short = "o",
		long = "output-file",
		parse(from_os_str)
	)]
	output_file: Option<std::path::PathBuf>,

	// #[structopt(
	// 	short = "N",
	// 	long = "disable-stdin"
	// )]
	// disable_stdin: bool,
}

fn parse_int_list(list: &mut String) -> Vec<i64> {
	list.retain(|c| c.is_digit(10) || c == ',' || c == '-');

	if list.is_empty() {return Vec::new()}

	list
		.split(',')
		.map(|s| if s.is_empty() {0} else {s.parse::<i64>().unwrap()})
		.collect()
}

fn read_program(path: &std::path::PathBuf) -> Vec<i64> {
	let mut content = std::fs::read_to_string(path).expect("Failed to read file");
	/*let program = */parse_int_list(&mut content)//;
	// let mut memory = HashMap::new();
	// for (i, x) in program.iter().enumerate() {
	// 	// I don't know why x needs to be dereferenced here, but whatever
	// 	memory.insert(i as i64, *x);
	// }
	// memory
}

fn read_stdin() -> Vec<i64> {
	let mut input = String::new();

	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input");
	
	parse_int_list(&mut input)
}

fn main() {
	let args = Cli::from_args();

	// println!("{:?}", args.first_in);
	// println!("{:?}", args.input_file);
	// println!("{:?}", args.output_file);
	// println!("{:?}", args.disable_stdin);

	let memory = read_program(&args.path);
	let mut input: Vec<i64> = Vec::new();

	input.append(&mut args.first_in.clone());
	match args.input_file {
		None => {}
		Some(path) => {
			let mut content = std::fs::read_to_string(path).expect("Failed to read input file");
			input.append(&mut parse_int_list(&mut content));
		}
	}
	// TODO reenable stdin, but only read from stdin if other inputs are depleted
	// if !args.disable_stdin {
	//	input.append(&mut read_stdin());
	// }

	// println!("{:?}", input);
	// println!("{:?}", memory);

	let mut vm = intcode::IntVM::new(memory, VecDeque::from(input));

	let output = vm.run();

	match args.output_file {
		None => println!("{:?}", output),
		Some(path) => {
			let output = format!("{:?}", output);
			let mut outfile = match File::create(&path) {
				Err(err) => panic!("Failed to create output file: {}", err),
				Ok(file) => file
			};
			match outfile.write_all(output.as_bytes()) {
				Err(err) => panic!("Failed to write to output file: {}", err),
				Ok(_) => {}
			}
		}
	}
}
