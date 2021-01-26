use std::collections::HashSet;

#[derive(Debug)]
struct Console<'a> {
    program: &'a Vec<Instruction>,

    acc: i32,
    pc: usize,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: String,
    arg: i32,
}

impl<'a> Console<'a> {
    fn new(program: &'a Vec<Instruction>) -> Console<'a> {
        Console {
            program,
            acc: 0,
            pc: 0,
        }
    }

    fn debug(&mut self) -> (bool, i32) {
        let mut visited: HashSet<usize> = HashSet::new();

        loop {
            let inst = &self.program[self.pc];

            let prev_pc = self.pc;
            match inst.opcode.as_str() {
                "nop" => self.pc += 1,
                "jmp" => self.pc = (self.pc as i32 + inst.arg) as usize,
                "acc" => {
                    self.acc = self.acc + inst.arg;
                    self.pc += 1
                }
                _ => {
                    panic!("Unhandled instruction")
                }
            }
            visited.insert(prev_pc);

            if visited.contains(&self.pc) {
                // println!("Reached already visited instruction {}", self.pc);
                return (false, self.acc);
            }

            if self.pc == self.program.len() {
                // println!("Program terminates {} {}", self.pc, self.acc);
                return (true, self.acc);
            } else if self.pc > self.program.len() {
                println!("Buggy program");
                return (false, self.acc);
            }
        }
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| {
            let tokens = &x.split(" ").collect::<Vec<&str>>()[..];
            if let &[opcode, arg] = tokens {
                Instruction {
                    opcode: opcode.to_owned(),
                    arg: arg.parse().unwrap(),
                }
            } else {
                panic!("Cannot parse")
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(raw: &str) -> i32 {
    let program = parse(raw);
    let mut console = Console::new(&program);
    let (_, acc) = console.debug();
    acc
}

#[aoc(day8, part2)]
pub fn part2(raw: &str) -> i32 {
    let program = parse(raw);
    let mut edited = program.clone();

    for (i, instruction) in program
        .iter()
        .enumerate()
        .filter(|(_i, inst)| inst.opcode != "acc")
    {
        let mut hack = instruction.clone();
        if instruction.opcode == "jmp" {
            hack.opcode = "nop".to_owned()
        } else if instruction.opcode == "nop" {
            hack.opcode = "jmp".to_owned()
        } else {
            panic!("unexpected instruction")
        }

        /* Attempt to run program with edited instruction */
        edited[i] = hack;
        let mut console = Console::new(&edited);
        if let (true, acc) = console.debug() {
            /* Program correctly halted ! */
            return acc;
        }
        /* Infinite loop detected, try again */
        edited[i] = instruction.clone();
    }
    0
}
