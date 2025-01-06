/// Ran until 621_696_516_096
fn main() {
    let result_1 = exercise_1(
        vec![2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0],
        28_066_687,
        0,
        0,
    );
    println!("{}", result_1);

    let result_2 = exercise_2(
        vec![2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0],
        0,
        0,
       500_000_000_000,
    );
    println!("{}", result_2);
}

struct Computer {
    tape: Vec<u8>,
    ax: usize,
    bx: usize,
    cx: usize,
    instruction_pointer: usize,
    output: Vec<u8>,
}

enum Operand {
    Literal(u8),
    Combo(u8),
}

enum Instruction {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Self::Adv(operand),
            1 => Self::Bxl(operand),
            2 => Self::Bst(operand),
            3 => Self::Jnz(operand),
            4 => Self::Bxc,
            5 => Self::Out(operand),
            6 => Self::Bdv(operand),
            7 => Self::Cdv(operand),
            _ => panic!("Unrecognized operand code {}", opcode),
        }
    }
    fn extract_operand(&self) -> Operand {
        match self {
            Self::Adv(i) => Operand::Combo(*i),
            Self::Bxl(i) => Operand::Literal(*i),
            Self::Bst(i) => Operand::Combo(*i),
            Self::Jnz(i) => Operand::Literal(*i),
            Self::Bxc => Operand::Literal(0),
            Self::Out(i) => Operand::Combo(*i),
            Self::Bdv(i) => Operand::Combo(*i),
            Self::Cdv(i) => Operand::Combo(*i),
        }
    }
}

impl Computer {
    fn get_operand(&self, instruction: &Instruction) -> usize {
        let operand = instruction.extract_operand();
        match operand {
            Operand::Literal(i) => i as usize,
            Operand::Combo(i) => match i {
                0..=3 => i as usize,
                4 => self.ax,
                5 => self.bx,
                6 => self.cx,
                7 => panic!("Reserved operand code"),
                _ => panic!("Unsupported operand code"),
            },
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Option<u8> {
        let operand = self.get_operand(instruction);
        match instruction {
            Instruction::Adv(_) | Instruction::Bdv(_) | Instruction::Cdv(_) => {
                let numerator = self.ax;
                let denominator: usize = usize::pow(2, operand as u32);
                match instruction {
                    Instruction::Adv(_) => self.ax = numerator / denominator,
                    Instruction::Bdv(_) => self.bx = numerator / denominator,
                    Instruction::Cdv(_) => self.cx = numerator / denominator,
                    _ => unreachable!(),
                };
            }
            Instruction::Bxl(_) => {
                self.bx ^= operand;
            }
            Instruction::Bst(_) => {
                self.bx = operand % 8;
            }
            Instruction::Jnz(_) => {
                if self.ax != 0 {
                    self.instruction_pointer = operand;
                    return None;
                }
            }
            Instruction::Bxc => {
                self.bx ^= self.cx;
            }
            Instruction::Out(_) => {
                let out = (operand % 8) as u8;
                self.output.push(out);
                self.instruction_pointer += 2;
                return Some(out);
            }
        }
        self.instruction_pointer += 2;
        None
    }

    fn valid_instruction(&self) -> bool {
        self.tape.len() > 1 && self.instruction_pointer < self.tape.len() - 1
    }

    fn execution_loop(&mut self) {
        while self.valid_instruction() {
            let instruction = Instruction::new(
                self.tape[self.instruction_pointer],
                self.tape[self.instruction_pointer + 1],
            );
            self.execute_instruction(&instruction);
        }
    }

    fn print_output(&self) -> String {
        self.output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn brute_force_is_identical_program(&mut self) -> bool {
        let mut out_cmp_idx = 0;
        while self.valid_instruction() {
            let instruction = Instruction::new(
                self.tape[self.instruction_pointer],
                self.tape[self.instruction_pointer + 1],
            );
            if let Some(out) = self.execute_instruction(&instruction) {
                if out_cmp_idx >= self.tape.len() || self.tape[out_cmp_idx] != out {
                    return false;
                }
                out_cmp_idx += 1;
            }
        }
        (self.output.len() == self.tape.len()) && self.output == self.tape
    }
}

fn exercise_1(tape: Vec<u8>, ax: usize, bx: usize, cx: usize) -> String {
    let mut computer = Computer {
        tape,
        ax,
        bx,
        cx,
        instruction_pointer: 0,
        output: vec![],
    };

    computer.execution_loop();
    computer.print_output()
}

use rayon::prelude::*;

fn exercise_2(tape: Vec<u8>, bx: usize, cx: usize, search_start: usize) -> usize {
    const CHUNK_SIZE: usize = 128;

    let chunks = (0..).map(|i| {
        let start = search_start + (i * CHUNK_SIZE);
        (start, start + CHUNK_SIZE)
    });

    chunks
        .take_while(|(start, _)| *start <= usize::MAX - CHUNK_SIZE)
        .find_map(|(chunk_start, chunk_end)| {
            (chunk_start..chunk_end).into_par_iter().find_first(|&ax| {
                let mut computer = Computer {
                    tape: tape.clone(),
                    ax,
                    bx,
                    cx,
                    instruction_pointer: 0,
                    output: Vec::with_capacity(tape.len()),
                };
                if ax % 1_073_741_824 == 0 {
                    println!("{}", ax);
                }
                computer.brute_force_is_identical_program()
            })
        })
        .expect("Solution should exist")
}

#[cfg(test)]
mod tests {
    use super::{exercise_1, exercise_2};

    #[test]
    fn test_exercise_1() {
        assert_eq!(
            exercise_1(vec![0, 1, 5, 4, 3, 0], 729, 0, 0),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn test_exercise_2() {
        assert_eq!(exercise_2(vec![0, 3, 5, 4, 3, 0], 0, 0, 1), 117440);
    }
}
