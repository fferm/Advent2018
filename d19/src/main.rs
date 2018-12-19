extern crate regex;

use std::fs;
use regex::Regex;
use std::cell::Cell;
use std::fmt;

fn main() {
    let mut small_input = false;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut program = read_inputs(filename);
    println!("{:?}", program);

    let mut reg = Registers::new();
    let instr = Instruction{op_code: OpCode::Seti, a: 2, b: 3, c: 4};
    println!("Before: {:?}", reg);

    instr.perform(&mut reg);
    println!("After: {:?}", reg);

/*    for instruction in program.instructions {
        let new_register = instruction.perform(program.registers.get());
        program.registers.set(new_register);
    }

    println!("Registers after program: {:?}", program.registers);*/
}



fn read_inputs(filename: &str) -> Program {
    let mut instructions = Vec::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let ip_line = lines.get(0).unwrap();
    let cap = Regex::new("#ip (\\d+)").unwrap().captures_iter(ip_line).next().expect("Error in regex");
    let ip_reg_num: isize = cap[1].parse().unwrap();

    let mut current_line = 1;
    while current_line < lines.len() {
        let instruction_line = lines.get(current_line).unwrap();
        let instruction_regex = "(\\w+) (\\d+) (\\d+) (\\d+)";
        let cap = Regex::new(&instruction_regex).unwrap().captures_iter(instruction_line).next().expect("Error in capturing instruction regex");
        let instruction = Instruction{op_code: OpCode::from_input(cap[1].to_owned()), a: cap[2].parse().unwrap(), b: cap[3].parse().unwrap(), c: cap[4].parse().unwrap()};

        instructions.push(instruction);

        current_line += 1;
    }

    return Program::new(instructions);
}

struct Program {
    instructions: Vec<Instruction>,
    registers: Cell<Registers>,
    ip_reg_num: isize,
    ip: Cell<isize>
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Program {
        let registers = Registers::new();
        return Program{instructions, registers: Cell::new(registers), ip_reg_num: 0, ip: Cell::new(0)};
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        ret.push_str(&format!("Ip-register: {}\n", self.ip_reg_num)[..]);

        for instr in &self.instructions {
            ret.push_str(&format!("{:?}\n", instr)[..]);
        }

        return write!(f, "{}", ret);
    }
}


#[derive(Copy, Clone)]
struct Registers {
    r0: isize,
    r1: isize,
    r2: isize,
    r3: isize,
    r4: isize,
    r5: isize
}

impl Registers {
    fn new() -> Registers {
        return Registers{r0: 0, r1: 0, r2: 0, r3: 0, r4: 0, r5: 0};
    }

    fn set(&mut self, idx: isize, value: isize) {
        match idx {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            _ => panic!("Unknownn idx: {}", idx)
        }
    }

    fn get(&self, idx: isize) -> isize {
        match idx {
            0 => return self.r0,
            1 => return self.r1,
            2 => return self.r2,
            3 => return self.r3,
            4 => return self.r4,
            5 => return self.r5,
            _ => panic!("Unknownn idx: {}", idx)
        }
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{}, {}, {}, {}, {}, {}]", self.r0, self.r1, self.r2, self.r3, self.r4, self.r5);
    }
}

//#[derive(Debug)]
struct Instruction {
    op_code: OpCode,
    a: isize,
    b: isize,
    c: isize
}

#[derive(Debug)]
enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr
}

impl OpCode {
    fn from_input(input: String) -> OpCode {
        if input.eq(&"addr".to_owned()) {
            return OpCode::Addr;
        }
        if input.eq(&"addi".to_owned()) {
            return OpCode::Addi;
        }
        if input.eq(&"mulr".to_owned()) {
            return OpCode::Mulr;
        }
        if input.eq(&"muli".to_owned()) {
            return OpCode::Muli;
        }
        if input.eq(&"banr".to_owned()) {
            return OpCode::Banr;
        }
        if input.eq(&"bani".to_owned()) {
            return OpCode::Bani;
        }
        if input.eq(&"borr".to_owned()) {
            return OpCode::Borr;
        }
        if input.eq(&"bori".to_owned()) {
            return OpCode::Bori;
        }
        if input.eq(&"setr".to_owned()) {
            return OpCode::Setr;
        }
        if input.eq(&"seti".to_owned()) {
            return OpCode::Seti;
        }
        if input.eq(&"gtri".to_owned()) {
            return OpCode::Gtri;
        }
        if input.eq(&"gtir".to_owned()) {
            return OpCode::Gtir;
        }
        if input.eq(&"gtrr".to_owned()) {
            return OpCode::Gtrr;
        }
        if input.eq(&"eqri".to_owned()) {
            return OpCode::Eqri;
        }
        if input.eq(&"eqir".to_owned()) {
            return OpCode::Eqir;
        }
        if input.eq(&"eqrr".to_owned()) {
            return OpCode::Eqrr;
        }
        panic!("Unknown op code: {}", input);
    }
}

impl Instruction {
    fn perform(&self, input: &mut Registers) {
        match self.op_code {
            OpCode::Addr => self.perform_addr(input),
            OpCode::Addi => self.perform_addi(input),
            OpCode::Mulr => self.perform_mulr(input),
            OpCode::Muli => self.perform_muli(input),
            OpCode::Banr => self.perform_banr(input),
            OpCode::Bani => self.perform_bani(input),
            OpCode::Borr => self.perform_borr(input),
            OpCode::Bori => self.perform_bori(input),
            OpCode::Setr => self.perform_setr(input),
            OpCode::Seti => self.perform_seti(input),
            OpCode::Gtir => self.perform_gtir(input),
            OpCode::Gtri => self.perform_gtri(input),
            OpCode::Gtrr => self.perform_gtrr(input),
            OpCode::Eqir => self.perform_eqir(input),
            OpCode::Eqri => self.perform_eqri(input),
            OpCode::Eqrr => self.perform_eqrr(input),
        }
    }

    fn perform_addr(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        reg.set(self.c, v1 + v2);
    }

    fn perform_addi(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = self.b;

        reg.set(self.c, v1 + v2);
    }

    fn perform_mulr(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        reg.set(self.c, v1 * v2);
    }

    fn perform_muli(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = self.b;

        reg.set(self.c, v1 * v2);
    }

    fn perform_banr(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        reg.set(self.c, v1 & v2);
    }

    fn perform_bani(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = self.b;

        reg.set(self.c, v1 & v2);
    }

    fn perform_borr(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        reg.set(self.c, v1 | v2);
    }

    fn perform_bori(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = self.b;

        reg.set(self.c, v1 | v2);
    }

    fn perform_setr(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);

        reg.set(self.c, v1);
    }

    fn perform_seti(&self, reg: &mut Registers) {
        let v1 = self.a;

        reg.set(self.c, v1);
    }

    fn perform_gtir(&self, reg: &mut Registers) {
        let v1 = self.a;
        let v2 = reg.get(self.b);

        let mut result = 0;
        if v1 > v2 {
            result = 1;
        }
        reg.set(self.c, result);
    }

    fn perform_gtri(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = self.b;

        let mut result = 0;
        if v1 > v2 {
            result = 1;
        }

        reg.set(self.c, result);
    }

    fn perform_gtrr(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        let mut result = 0;
        if v1 > v2 {
            result = 1;
        }
        reg.set(self.c, result);
    }

    fn perform_eqir(&self, reg: &mut Registers) {
        let v1 = self.a;
        let v2 = reg.get(self.b);

        let mut result = 0;
        if v1 == v2 {
            result = 1;
        }
        reg.set(self.c, result);
    }

    fn perform_eqri(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = self.b;

        let mut result = 0;
        if v1 == v2 {
            result = 1;
        }

        reg.set(self.c, result);
    }

    fn perform_eqrr(&self, reg: &mut Registers) {
        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        let mut result = 0;
        if v1 == v2 {
            result = 1;
        }
        reg.set(self.c, result);
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?} {} {} {}", self.op_code, self.a, self.b, self.c);
    }
}
