use std::collections::{HashMap, VecDeque};

pub struct IntVM {
	// TODO maybe store a second copy of memory in a format optimized for instructions?
	//      and only add to it when an instr is read for the first time
	//      and remove instrs when one of their values is changed in main memory
	memory: HashMap<i64, i64>,
	instr_ptr: i64,
	offset: i64,
	input: VecDeque<i64>,
}

#[derive(PartialEq, Eq)]
enum StepResult {
	CONTINUE,
	HALT
}

const ARG_COUNTS: [i64; 10] = [0, 3, 3, 1, 1, 2, 2, 3, 3, 1];

impl IntVM {
	pub fn new(memory: HashMap<i64, i64>, input: VecDeque<i64>) -> IntVM {
		IntVM {
			memory,
			instr_ptr: 0,
			offset: 0,
			input,
		}
	}

	pub fn run(&mut self) -> Vec<i64> {
		let mut output = Vec::new();
		while self.step(&mut output) == StepResult::CONTINUE {}
		output
	}

	fn get_memory(&self, idx: i64) -> i64 {
		if idx < 0 {
			panic!("Attempted to access negative memory address {}", idx);
		}
		match self.memory.get(&idx) {
			Some(x) => *x,
			None => 0
		}
	}

	fn set_memory(&mut self, idx: i64, val: i64) {
		self.memory.insert(idx, val);
	}

	fn get_arg_ptr(&self, offset: i64, mode: i64) -> i64 {
		let ptr = self.instr_ptr + offset;
		match mode {
			0 => self.get_memory(ptr),
			1 => ptr,
			2 => self.get_memory(ptr)+self.offset,
			_ => panic!("Received invalid argument mode {} at position {}, instruction index {}", mode, offset, self.instr_ptr)
		}
	}

	fn get_input(&mut self) -> std::option::Option<i64> {
		self.input.pop_front()
	}
	
	fn step(&mut self, output: &mut Vec<i64>) -> StepResult {
		let instr = self.get_memory(self.instr_ptr);

		// TODO negative instruction ints are undefined behavior?
		let opcode = (instr % 100) as usize;

		if opcode == 99 {return StepResult::HALT;}

		let mode1 = (instr / 100) % 10;
		let mode2 = (instr / 1000) % 10;
		let mode3 = (instr / 10000) % 10;

		let arg1_ptr = self.get_arg_ptr(1, mode1);
		let arg2_ptr = self.get_arg_ptr(2, mode2);
		let arg3_ptr = self.get_arg_ptr(3, mode3);
		
		let arg1 = self.get_memory(arg1_ptr);
		let arg2 = if ARG_COUNTS[opcode] > 1 {self.get_memory(arg2_ptr)} else {0};
		// let arg3 = if ARG_COUNTS[opcode] > 2 {self.get_memory(arg3_ptr)} else {0};

		match opcode {
			1 => self.set_memory(arg3_ptr, arg1+arg2),
			2 => self.set_memory(arg3_ptr, arg1*arg2),
			3 => {
				let val = self.get_input();
				match val {
					Some(x) => self.set_memory(arg1_ptr, x),
					None => return StepResult::HALT
				}
			}
			4 => output.push(arg1),
			5 => if arg1 != 0 {self.instr_ptr = arg2; return StepResult::CONTINUE},
			6 => if arg1 == 0 {self.instr_ptr = arg2; return StepResult::CONTINUE},
			7 => self.set_memory(arg3_ptr, if arg1 < arg2 {1} else {0}),
			8 => self.set_memory(arg3_ptr, if arg1 == arg2 {1} else {0}),
			9 => self.offset += arg1,
			_ => panic!("Received invalid opcode {} in instruction {} at memory index {}", opcode, instr, self.instr_ptr)
		}

		self.instr_ptr += 1 + ARG_COUNTS[opcode];
		StepResult::CONTINUE
	}
}
