type Intcode = usize;

struct Program(Vec<Intcode>);

struct ProgramIter;

impl Iterator for ProgramIter {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl IntoIterator for Program {
    type Item = Instruction;
    type IntoIter = ProgramIter;

    fn into_iter(self) -> Self::IntoIter {
        ProgramIter
    }
}

#[derive(Debug, PartialEq)]
enum Error {
    InvalidOpCode,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add,
    Mul,
    Load(usize),
    Store(usize),
    Halt,
}

struct Interpreter {
    counter: usize,
    program: Program,
}

impl Interpreter {
    fn new(program: Program) -> Self {
        Interpreter {
            counter: 0,
            program,
        }
    }

    fn intcode_at_offset(&self, offset: usize) -> Intcode {
        self.program.0[self.counter + offset]
    }

    fn next_instructions(&self) -> Result<Vec<Instruction>, Error> {
        match self.intcode_at_offset(0) {
            1 => Ok(vec![
                Instruction::Load(self.intcode_at_offset(1)),
                Instruction::Load(self.intcode_at_offset(2)),
                Instruction::Add,
                Instruction::Store(self.intcode_at_offset(3)),
            ]),
            2 => Ok(vec![
                Instruction::Load(self.intcode_at_offset(1)),
                Instruction::Load(self.intcode_at_offset(2)),
                Instruction::Mul,
                Instruction::Store(self.intcode_at_offset(3)),
            ]),
            99 => Ok(vec![Instruction::Halt]),
            _ => Err(Error::InvalidOpCode),
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    mod interpreter {
        use super::*;

        #[test]
        fn can_interpret_halt() {
            let expected = vec![Instruction::Halt];

            let program = Program(vec![99]);
            let interpreter = Interpreter::new(program);
            let instructions = interpreter.next_instructions();

            assert_eq!(Ok(expected), instructions);
        }

        #[test]
        fn halting_is_idempotent() {
            let expected = vec![Instruction::Halt];

            let program = Program(vec![99]);
            let interpreter = Interpreter::new(program);
            interpreter.next_instructions().expect("Unexpected error");
            let instructions = interpreter.next_instructions();

            assert_eq!(Ok(expected), instructions);
        }

        #[test]
        fn can_interpret_add() {
            let expected = vec![
                Instruction::Load(0),
                Instruction::Load(0),
                Instruction::Add,
                Instruction::Store(0),
            ];

            let program = Program(vec![1, 0, 0, 0]);
            let interpreter = Interpreter::new(program);
            let instructions = interpreter.next_instructions();

            assert_eq!(Ok(expected), instructions);
        }

        #[test]
        fn can_interpret_mul() {
            let expected = vec![
                Instruction::Load(0),
                Instruction::Load(0),
                Instruction::Mul,
                Instruction::Store(0),
            ];

            let program = Program(vec![2, 0, 0, 0]);
            let interpreter = Interpreter::new(program);
            let instructions = interpreter.next_instructions();

            assert_eq!(Ok(expected), instructions);
        }
    }
}
