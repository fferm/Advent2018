extern crate regex;

use std::fs;
use regex::Regex;
use std::cell::Cell;
use std::fmt;

fn main() {
    let small_input = false;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut program = read_inputs(filename);
    println!("{:?}", program);

    while program.should_continue() {
        print!("ip={:2}\t{:?}\t", program.ip.get(), program.get_current_instruction());

        let new_registers = program.run_current_instruction();

        println!("{:?}", new_registers);
    }
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

    return Program::new(instructions, ip_reg_num);
}

struct Program {
    instructions: Vec<Instruction>,
    registers: Cell<Registers>,
    ip_reg_num: isize,
    ip: Cell<isize>
}

impl Program {
    fn new(instructions: Vec<Instruction>, ip_reg_num: isize) -> Program {
        let registers = Registers::new();
        return Program{instructions, registers: Cell::new(registers), ip_reg_num, ip: Cell::new(0)};
    }

    fn run_current_instruction(&mut self) -> Registers {    // Returns registers after instruction is run
        self.write_ip_to_ip_register();
        self.run_actual_instruction();
        self.write_ip_register_to_ip();

        self.step_instruction_pointer();

        return self.registers.get().clone()
    }

    fn write_ip_to_ip_register(&mut self) {
        let mut reg = self.registers.get().clone();
        reg.set(self.ip_reg_num, self.ip.get());
        self.registers.set(reg);
    }

    fn run_actual_instruction(&mut self) {
        let current_instruction = self.get_current_instruction();
        let new_reg = current_instruction.perform(&self.registers.get());
        self.registers.set(new_reg);
    }

    fn write_ip_register_to_ip(&self) {
        let value = self.registers.get().get(self.ip_reg_num);
        self.ip.set(value);
    }

    fn get_current_instruction(&self) -> &Instruction {
        return self.instructions.get(self.ip.get() as usize).unwrap();
    }

    fn should_continue(&self) -> bool {
        return (self.ip.get() as usize) < self.instructions.len();
    }

    fn step_instruction_pointer(&mut self) {
        let prev = self.ip.get();

        self.ip.set(prev + 1);
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
        return write!(f, "[{:10}, {:10}, {:10}, {:10}, {:10}, {:10}]", self.r0, self.r1, self.r2, self.r3, self.r4, self.r5);
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
    fn perform(&self, input: &Registers) -> Registers {
        match self.op_code {
            OpCode::Addr => return self._perform_addr(input),
            OpCode::Addi => return self._perform_addi(input),
            OpCode::Mulr => return self._perform_mulr(input),
            OpCode::Muli => return self._perform_muli(input),
            OpCode::Banr => return self._perform_banr(input),
            OpCode::Bani => return self._perform_bani(input),
            OpCode::Borr => return self._perform_borr(input),
            OpCode::Bori => return self._perform_bori(input),
            OpCode::Setr => return self._perform_setr(input),
            OpCode::Seti => return self._perform_seti(input),
            OpCode::Gtir => return self._perform_gtir(input),
            OpCode::Gtri => return self._perform_gtri(input),
            OpCode::Gtrr => return self._perform_gtrr(input),
            OpCode::Eqir => return self._perform_eqir(input),
            OpCode::Eqri => return self._perform_eqri(input),
            OpCode::Eqrr => return self._perform_eqrr(input),
        }
    }

    fn _perform_addr(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        ret.set(self.c, v1 + v2);

        return ret;
    }

    fn _perform_addi(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = self.b;

        ret.set(self.c, v1 + v2);

        return ret;
    }

    fn _perform_mulr(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        ret.set(self.c, v1 * v2);

        return ret;
    }

    fn _perform_muli(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = self.b;

        ret.set(self.c, v1 * v2);

        return ret;
    }

    fn _perform_banr(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        ret.set(self.c, v1 & v2);

        return ret;
    }

    fn _perform_bani(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = self.b;

        ret.set(self.c, v1 & v2);

        return ret;
    }

    fn _perform_borr(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        ret.set(self.c, v1 | v2);

        return ret;
    }

    fn _perform_bori(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = self.b;

        ret.set(self.c, v1 | v2);

        return ret;
    }

    fn _perform_setr(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);

        ret.set(self.c, v1);

        return ret;
    }

    fn _perform_seti(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = self.a;

        ret.set(self.c, v1);

        return ret;
    }

    fn _perform_gtir(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = self.a;
        let v2 = reg.get(self.b);

        let mut result = 0;
        if v1 > v2 {
            result = 1;
        }
        ret.set(self.c, result);

        return ret;
    }

    fn _perform_gtri(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = self.b;

        let mut result = 0;
        if v1 > v2 {
            result = 1;
        }

        ret.set(self.c, result);

        return ret;
    }

    fn _perform_gtrr(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        let mut result = 0;
        if v1 > v2 {
            result = 1;
        }
        ret.set(self.c, result);

        return ret;
    }

    fn _perform_eqir(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = self.a;
        let v2 = reg.get(self.b);

        let mut result = 0;
        if v1 == v2 {
            result = 1;
        }
        ret.set(self.c, result);

        return ret;
    }

    fn _perform_eqri(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = self.b;

        let mut result = 0;
        if v1 == v2 {
            result = 1;
        }

        ret.set(self.c, result);

        return ret;
    }

    fn _perform_eqrr(&self, reg: &Registers) -> Registers {
        let mut ret = reg.clone();

        let v1 = reg.get(self.a);
        let v2 = reg.get(self.b);

        let mut result = 0;
        if v1 == v2 {
            result = 1;
        }
        ret.set(self.c, result);

        return ret;
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?} {:2} {:2} {:2}", self.op_code, self.a, self.b, self.c);
    }
}
