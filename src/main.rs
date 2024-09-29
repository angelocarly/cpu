enum Instruction {
    Print
}

struct Executor {
}

impl Executor {
    fn run(i: Instruction) {
        match i {
            Instruction::Print => {
                println!("Hello, world!");
            },
        }
    }
}

fn main() {
    Executor::run(Instruction::Print);
}


#[test]
fn test_instruction() {
    Executor::run(Instruction::Print);
}