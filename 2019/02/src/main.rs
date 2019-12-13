use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct InvalidOpCode;

#[derive(Debug, PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Halt,
}

impl TryFrom<isize> for OpCode {
    type Error = InvalidOpCode;

    fn try_from(value: isize) -> Result<OpCode, Self::Error> {
        match value {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Multiply),
            99 => Ok(OpCode::Halt),
            _ => Err(InvalidOpCode),
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    mod opcodes {
        use super::*;

        #[test]
        fn can_be_created_from_valid_integers() {
            let inputs = vec![(1, OpCode::Add), (2, OpCode::Multiply), (99, OpCode::Halt)];

            for (given, expected) in inputs {
                assert_eq!(Ok(expected), OpCode::try_from(given));
            }
        }

        #[test]
        fn cannot_be_created_from_invalid_integers() {
            let inputs = vec![
                (-1, InvalidOpCode),
                (0, InvalidOpCode),
                (3, InvalidOpCode),
                (100, InvalidOpCode),
            ];

            for (given, expected) in inputs {
                assert_eq!(Err(expected), OpCode::try_from(given));
            }
        }
    }
}
