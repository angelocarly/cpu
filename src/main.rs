use crate::Instruction::{Load, Read, Return, Write};

enum Instruction {
    Load(bool),
    Read(i8),
    Write(i8),
    Return,
}

struct Machine {
    memory: [bool; 256],
    register: bool,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            memory: [false; 256],
            register: false
        }
    }

    fn run(&mut self, instructions: &[Instruction]) -> Option<bool> {
        for instruction in instructions {
            match instruction {
                Load(v) => {
                    self.register = *v;
                },
                Read(i) => {
                    self.register = self.memory[*i as usize]
                },
                Write(i) => {
                    self.memory[*i as usize] = self.register;
                },
                Return=> {
                    return Some(self.register);
                }
            }
        }

        None
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_return_gives_none() {
        let instructions = [];
        let result = Machine::new().run(&instructions);
        assert_eq!(result, None);
    }

    #[test]
    fn test_return_gives_register() {
        let instructions = [
            Return
        ];
        let result = Machine::new().run(&instructions);
        assert_eq!(result, Some(false));
    }

    #[test]
    fn test_return_gives_loaded_register() {
        let instructions = [
            Load(true),
            Return
        ];
        let result = Machine::new().run(&instructions);
        assert_eq!(result, Some(true));
    }

    #[test]
    fn test_write_readback() {
        let instructions = [
            Load(true),
            Write(0),
            Read(0),
            Return
        ];
        let result = Machine::new().run(&instructions);
        assert_eq!(result, Some(true));
    }
}