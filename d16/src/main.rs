extern crate regex;

use std::fs;
use regex::Regex;
use std::cell::Cell;

fn main() {
    let mut small_input = false;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut program = read_inputs(filename);

    for instruction in program.instructions {
        let new_register = instruction.perform(program.registers.get());
        program.registers.set(new_register);
    }

    println!("Registers after program: {:?}", program.registers);
}



fn read_inputs(filename: &str) -> Program {
    let mut instructions = Vec::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();
    let mut current_line = 0;

    while current_line < lines.len() {
        let instruction_line = lines.get(current_line).unwrap();
        let instruction_regex = "(\\d+) (\\d+) (\\d+) (\\d+)";
        let cap = Regex::new(&instruction_regex).unwrap().captures_iter(instruction_line).next().expect("Error in capturing instruction regex");
        let instruction = Instruction{op_code: cap[1].parse().unwrap(), a: cap[2].parse().unwrap(), b: cap[3].parse().unwrap(), c: cap[4].parse().unwrap()};

        instructions.push(instruction);

        current_line += 1;
    }

    let program = Program {instructions, registers: Cell::new(Registers{r0: 0, r1: 0, r2: 0, r3: 0})};
    return program;
}

fn read_register_line(line: &str, starting_text: &str) -> Registers {
    let regex = starting_text.to_owned() + "\\[(\\d+), (\\d+), (\\d+), (\\d+)\\]";
    let re = Regex::new(&regex[..]).unwrap();
    let cap = re.captures_iter(line).next().expect("Error in capturing regex");
    let register = Registers{r0: cap[1].parse().unwrap(), r1: cap[2].parse().unwrap(), r2: cap[3].parse().unwrap(), r3: cap[4].parse().unwrap()};

    return register;

}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    registers: Cell<Registers>
}

#[derive(Debug)]
struct Step {
    before: Registers,
    after: Registers,
    instruction: Instruction
}

impl Step {
    fn run_all_instructions(&self) -> Vec<String> {
        let mut result = Vec::new();

        if self.instruction.perform_addr(self.before) == self.after {
            result.push("addr".to_owned());
        }
        if self.instruction.perform_addi(self.before) == self.after {
            result.push("addi".to_owned());
        }
        if self.instruction.perform_mulr(self.before) == self.after {
            result.push("mulr".to_owned());
        }
        if self.instruction.perform_muli(self.before) == self.after {
            result.push("muli".to_owned());
        }
        if self.instruction.perform_banr(self.before) == self.after {
            result.push("banr".to_owned());
        }
        if self.instruction.perform_bani(self.before) == self.after {
            result.push("bani".to_owned());
        }
        if self.instruction.perform_borr(self.before) == self.after {
            result.push("borr".to_owned());
        }
        if self.instruction.perform_bori(self.before) == self.after {
            result.push("bori".to_owned());
        }
        if self.instruction.perform_setr(self.before) == self.after {
            result.push("setr".to_owned());
        }
        if self.instruction.perform_seti(self.before) == self.after {
            result.push("seti".to_owned());
        }
        if self.instruction.perform_gtir(self.before) == self.after {
            result.push("gtir".to_owned());
        }
        if self.instruction.perform_gtri(self.before) == self.after {
            result.push("gtri".to_owned());
        }
        if self.instruction.perform_gtrr(self.before) == self.after {
            result.push("gtrr".to_owned());
        }
        if self.instruction.perform_eqir(self.before) == self.after {
            result.push("eqir".to_owned());
        }
        if self.instruction.perform_eqri(self.before) == self.after {
            result.push("eqri".to_owned());
        }
        if self.instruction.perform_eqrr(self.before) == self.after {
            result.push("eqrr".to_owned());
        }

        return result;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Registers {
    r0: isize,
    r1: isize,
    r2: isize,
    r3: isize
}

impl Registers {
    fn set(&mut self, idx: isize, value: isize) {
        match idx {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            _ => panic!("Unknownn idx: {}", idx)
        }
    }

    fn get(&self, idx: isize) -> isize {
        match idx {
            0 => return self.r0,
            1 => return self.r1,
            2 => return self.r2,
            3 => return self.r3,
            _ => panic!("Unknownn idx: {}", idx)
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op_code: isize,
    a: isize,
    b: isize,
    c: isize
}

impl Instruction {
    fn perform(&self, input: Registers) -> Registers {
        match self.op_code {
            0 => return self.perform_gtir(input),
            1 => return self.perform_mulr(input),
            2 => return self.perform_seti(input),
            3 => return self.perform_gtrr(input),
            4 => return self.perform_bori(input),
            5 => return self.perform_borr(input),
            6 => return self.perform_banr(input),
            7 => return self.perform_eqri(input),
            8 => return self.perform_bani(input),
            9 => return self.perform_addr(input),
            10 => return self.perform_addi(input),
            11 => return self.perform_eqrr(input),
            12 => return self.perform_gtri(input),
            13 => return self.perform_eqir(input),
            14 => return self.perform_setr(input),
            15 => return self.perform_muli(input),
            _ => panic!("Unknown opcode: {}", self.op_code)

        }
    }

    fn perform_addr(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = input.get(self.b);

        reg.set(self.c, v1 + v2);

        return reg;
    }

    fn perform_addi(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = self.b;

        reg.set(self.c, v1 + v2);

        return reg;
    }

    fn perform_mulr(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = input.get(self.b);

        reg.set(self.c, v1 * v2);

        return reg;
    }

    fn perform_muli(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = self.b;

        reg.set(self.c, v1 * v2);

        return reg;
    }

    fn perform_banr(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = input.get(self.b);

        reg.set(self.c, v1 & v2);

        return reg;
    }

    fn perform_bani(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = self.b;

        reg.set(self.c, v1 & v2);

        return reg;
    }

    fn perform_borr(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = input.get(self.b);

        reg.set(self.c, v1 | v2);

        return reg;
    }

    fn perform_bori(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = self.b;

        reg.set(self.c, v1 | v2);

        return reg;
    }

    fn perform_setr(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);

        reg.set(self.c, v1);

        return reg;
    }

    fn perform_seti(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = self.a;

        reg.set(self.c, v1);

        return reg;
    }

    fn perform_gtir(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = self.a;
        let v2 = input.get(self.b);

        let mut result = 0;
        if v1 > v2 {
            result = 1;
        }
        reg.set(self.c, result);

        return reg;
    }

    fn perform_gtri(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = self.b;

        let mut result = 0;
        if v1 > v2 {
            result = 1;
        }

        reg.set(self.c, result);

        return reg;
    }

    fn perform_gtrr(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = input.get(self.b);

        let mut result = 0;
        if v1 > v2 {
            result = 1;
        }
        reg.set(self.c, result);

        return reg;
    }

    fn perform_eqir(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = self.a;
        let v2 = input.get(self.b);

        let mut result = 0;
        if v1 == v2 {
            result = 1;
        }
        reg.set(self.c, result);

        return reg;
    }

    fn perform_eqri(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = self.b;

        let mut result = 0;
        if v1 == v2 {
            result = 1;
        }

        reg.set(self.c, result);

        return reg;
    }

    fn perform_eqrr(&self, input: Registers) -> Registers {
        let mut reg =input.clone();

        let v1 = input.get(self.a);
        let v2 = input.get(self.b);

        let mut result = 0;
        if v1 == v2 {
            result = 1;
        }
        reg.set(self.c, result);

        return reg;
    }
}