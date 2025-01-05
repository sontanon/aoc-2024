fn main() {
    let result_1 = exercise_1(
        vec![2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0],
        28066687,
        0,
        0,
    );
    println!("{}", result_1);
}

struct Computer {
    tape: Vec<u8>,
    ax: usize,
    bx: usize,
    cx: usize,
    instruction_pointer: usize,
    output: Vec<usize>,
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

    fn execute_instruction(&mut self, instruction: &Instruction) {
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
                    return;
                }
            }
            Instruction::Bxc => {
                self.bx ^= self.cx;
            }
            Instruction::Out(_) => {
                self.output.push(operand % 8);
            }
        }
        self.instruction_pointer += 2;
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

#[cfg(test)]
mod tests {
    use super::exercise_1;

    #[test]
    fn test_exercise_1() {
        assert_eq!(
            exercise_1(vec![0, 1, 5, 4, 3, 0], 729, 0, 0),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }
}
