use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;

use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Error, Debug)]
enum Error {
    #[error("encountered an I/O error")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, PartialEq)]
struct Tile {
    marked: bool,
    number: u8,
}

impl Tile {
    fn new(number: u8) -> Self {
        Self {
            marked: false,
            number,
        }
    }
}

#[derive(Debug, PartialEq)]
struct BingoBoard(Vec<Vec<Tile>>);

impl BingoBoard {
    fn new(board: Vec<Vec<u8>>) -> Self {
        Self(
            board
                .iter()
                .map(|row| row.iter().map(|n| Tile::new(*n)).collect::<Vec<Tile>>())
                .collect::<Vec<Vec<Tile>>>(),
        )
    }

    fn mark(&mut self, number: u8) {
        for row in self.0.iter_mut() {
            for tile in row.iter_mut() {
                if tile.number == number {
                    tile.marked = true;
                }
            }
        }
    }

    fn bingo(&self) -> bool {
        false
    }
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input).map_err(Error::Io)?);
    let input = reader.lines().collect::<Result<Vec<String>, _>>()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{BingoBoard, Tile};

    #[test]
    fn test_tile_marking() {
        let expected = Tile {
            marked: true,
            number: 42,
        };
        let mut given = Tile::new(42);
        given.marked = true;

        assert_eq!(given, expected);
    }

    #[test]
    fn test_board_instantiation() {
        let expected = BingoBoard(vec![
            vec![
                Tile {
                    marked: false,
                    number: 14,
                },
                Tile {
                    marked: false,
                    number: 21,
                },
                Tile {
                    marked: false,
                    number: 17,
                },
                Tile {
                    marked: false,
                    number: 24,
                },
                Tile {
                    marked: false,
                    number: 4,
                },
            ],
            vec![
                Tile {
                    marked: false,
                    number: 10,
                },
                Tile {
                    marked: false,
                    number: 16,
                },
                Tile {
                    marked: false,
                    number: 15,
                },
                Tile {
                    marked: false,
                    number: 9,
                },
                Tile {
                    marked: false,
                    number: 19,
                },
            ],
            vec![
                Tile {
                    marked: false,
                    number: 18,
                },
                Tile {
                    marked: false,
                    number: 8,
                },
                Tile {
                    marked: false,
                    number: 23,
                },
                Tile {
                    marked: false,
                    number: 26,
                },
                Tile {
                    marked: false,
                    number: 20,
                },
            ],
            vec![
                Tile {
                    marked: false,
                    number: 22,
                },
                Tile {
                    marked: false,
                    number: 11,
                },
                Tile {
                    marked: false,
                    number: 13,
                },
                Tile {
                    marked: false,
                    number: 6,
                },
                Tile {
                    marked: false,
                    number: 5,
                },
            ],
            vec![
                Tile {
                    marked: false,
                    number: 2,
                },
                Tile {
                    marked: false,
                    number: 0,
                },
                Tile {
                    marked: false,
                    number: 12,
                },
                Tile {
                    marked: false,
                    number: 3,
                },
                Tile {
                    marked: false,
                    number: 7,
                },
            ],
        ]);

        let given = BingoBoard::new(vec![
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7],
        ]);

        assert_eq!(given, expected);
    }

    fn test_board_marking() {
        let expected = BingoBoard(vec![
            vec![
                Tile {
                    marked: false,
                    number: 14,
                },
                Tile {
                    marked: false,
                    number: 21,
                },
                Tile {
                    marked: false,
                    number: 17,
                },
                Tile {
                    marked: false,
                    number: 24,
                },
                Tile {
                    marked: true,
                    number: 4,
                },
            ],
            vec![
                Tile {
                    marked: false,
                    number: 10,
                },
                Tile {
                    marked: false,
                    number: 16,
                },
                Tile {
                    marked: false,
                    number: 15,
                },
                Tile {
                    marked: false,
                    number: 9,
                },
                Tile {
                    marked: false,
                    number: 19,
                },
            ],
            vec![
                Tile {
                    marked: false,
                    number: 18,
                },
                Tile {
                    marked: false,
                    number: 8,
                },
                Tile {
                    marked: false,
                    number: 23,
                },
                Tile {
                    marked: false,
                    number: 26,
                },
                Tile {
                    marked: false,
                    number: 20,
                },
            ],
            vec![
                Tile {
                    marked: false,
                    number: 22,
                },
                Tile {
                    marked: false,
                    number: 11,
                },
                Tile {
                    marked: false,
                    number: 13,
                },
                Tile {
                    marked: false,
                    number: 6,
                },
                Tile {
                    marked: false,
                    number: 5,
                },
            ],
            vec![
                Tile {
                    marked: false,
                    number: 2,
                },
                Tile {
                    marked: false,
                    number: 0,
                },
                Tile {
                    marked: false,
                    number: 12,
                },
                Tile {
                    marked: false,
                    number: 3,
                },
                Tile {
                    marked: false,
                    number: 7,
                },
            ],
        ]);

        let mut given = BingoBoard::new(vec![
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7],
        ]);
        given.mark(4);

        assert_eq!(given, expected);
    }
}
