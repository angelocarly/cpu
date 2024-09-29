use crate::Instruction::Print;

enum Instruction {
    Print(i8)
}

struct Machine {
    memory: [bool; 256],
}

impl Machine {
    fn new() -> Machine {
        Machine {
            memory: [false; 256],
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Print(i) => {
                    println!("Memory at {}: {}", i, self.memory[*i as usize]);
                },
            }
        }
    }
}

fn main() {
}


#[test]
fn test_instruction() {
    let mut m = Machine::new();
    let instructions = [
        Print(0),
        Print(1),
    ];
    m.run(&instructions);
}