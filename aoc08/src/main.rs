use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Inst {
    Acc,
    Jmp,
    Nop,
}

fn parse(code: &str) -> Option<Vec<(Inst, i32)>> {
    code
        .lines()
        .map(|line| -> Option<(Inst, i32)>{
            let mut parts = line.split(' ');
            let inst = match parts.next()? {
                "acc" => Some(Inst::Acc),
                "jmp" => Some(Inst::Jmp),
                "nop" => Some(Inst::Nop),
                _ => None,
            };
            let offset = parts
                .next()?
                .parse()
                .ok()?;
            Some((inst?, offset))
        })
        .collect()
}

#[derive(Default)]
struct Cpu {
    pc: i32,
    acc: i32,
}

impl Cpu {
    fn execute(&mut self, operator: Inst, operand: i32) {
        match operator {
            Inst::Acc => {
                self.acc += operand;
                self.pc += 1;
            }
            Inst::Jmp => {
                self.pc += operand;
            }
            Inst::Nop => {
                self.pc += 1;
            }
        }
    }
}

fn run(insts: &[(Inst, i32)]) -> (bool, i32) {
    let mut visited_locations = HashSet::new();
    let mut cpu = Cpu::default();
    while let Some(&(operator, operand)) = insts.get(cpu.pc as usize) {
        if !visited_locations.insert(cpu.pc as usize) {
            return (false, cpu.acc);
        }
        cpu.execute(operator, operand);
    }
    (true, cpu.acc)
}


fn main() {
    let insts: Vec<_> = parse(include_str!("../input.txt")).unwrap();
    let answer1 = run(&insts).1;
    println!("#1 = {}", answer1);

    for i in 0..insts.len() {
        let copy = match insts[i].0 {
            Inst::Acc => continue,
            Inst::Jmp => {
                let mut copy = insts.clone();
                copy[i].0 = Inst::Nop;
                copy
            }
            Inst::Nop => {
                let mut copy = insts.clone();
                copy[i].0 = Inst::Jmp;
                copy
            }
        };

        let (terminated, acc) = run(&copy);
        if terminated {
            println!("#2 = {}", acc);
        }
    }
}
