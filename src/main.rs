use std::fmt;
use std::io;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Blank,
    Player1,
    Player2,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Cell::Blank => ' ',
            Cell::Player1 => '○',
            Cell::Player2 => '●',
        };
        write!(f, "{}", printable)
    }
}


fn main() {
    let mut grid: [Cell; 42] = [Cell::Blank; 42];

    let mut current_player: Cell = Cell::Player1;

    display(grid);

    loop {
        let pos = 'input:  loop {
            let column: u8 = input_column(current_player);

            for line in (0..6).rev() {
                let pos: usize = usize::from(line * 7 + column - 1);
                if grid[pos] == Cell::Blank {
                    grid[pos] = current_player;
                    break 'input pos;
                } else if line == 0 {
                    print!("Column full!\n");
                }
            }
        };

        display(grid);

        if winning(grid, pos) {
            print!("Player {} win!\n", current_player);
            break;
        }

        if grid_full(grid) {
            print!("The match ended in a draw!\n");
            break;
        }

        if current_player == Cell::Player1 {
            current_player = Cell::Player2;
        } else {
            current_player = Cell::Player1;
        }
    }
}

fn display(grid: [Cell; 42]) {
    // Clear terminal
    print!("{}[2J", 27 as char);

    print!("-- CONNECT 4 --\n\n");

    print!(" 1 2 3 4 5 6 7\n");
    print!("┌─┬─┬─┬─┬─┬─┬─┐\n");

    for (idx, cell) in grid.iter().enumerate() {
        print!("│{}", cell);

        if idx % 7 == 6 {
            print!("│\n");
        }
    }
    print!("└─┴─┴─┴─┴─┴─┴─┘\n\n");
}

fn input_column(player: Cell) -> u8 {
    loop {
        let mut input = String::new();
        println!("Player {}, please input column number (1-7):", player);
        io::stdin()
            .read_line(&mut input)
            .expect("Error");

        match input.trim().parse::<u8>() {
            Ok(n) => {
                if n >= 1 && n <= 7 {
                    return n;
                } else {
                    println!("invalid value (column must be 1-7)");
                }
            },
            Err(..) => println!("this was not an integer: {}", input),
        };
    }
}

fn grid_full(grid: [Cell; 42]) -> bool {
    None == grid.iter().find(|&&cell| cell == Cell::Blank)
}

fn winning(grid: [Cell; 42], pos: usize) -> bool {
    // Step 1: horizontal
    // Step 6: diagonal from bottom-left to top-right
    // Step 7: vertical (only to bottom)
    // Step 8: diagonal from top-left to bottom-right
    // for step in [1,6,7,8] {
    for step in [1,6,7,8] {
        let mut in_a_row: usize = 1;

        // before
        if step != 7 {
            let mut pos2 = pos;
            loop {
                if in_a_row == 4 {
                    return true;
                }

                if pos2 < step {
                    break;
                }

                pos2 -= step;

                if pos%7 < (pos2%7 + 4) && grid[pos] == grid[pos2] {
                    in_a_row += 1;
                } else {
                    break;
                }
            }
        }

        // after
        let mut pos2 = pos + step;
        while in_a_row < 4 && pos2 < 42 && (isize::try_from(pos).unwrap()%7 - isize::try_from(pos2).unwrap()%7).abs() < 4 && grid[pos] == grid[pos2] {
            in_a_row += 1;
            pos2 += step;
        }

        if in_a_row == 4 {
            return true;
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    const ___: Cell = Cell::Blank;
    const _O_: Cell = Cell::Player1;
    const _X_: Cell = Cell::Player2;

    #[test]
    fn test_winning_empty() {
        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,_O_,
        ], 41), false);
    }

    #[test]
    fn test_winning_vertical() {
        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,_O_,_X_,___,___,___,
            ___,___,_O_,_X_,___,___,___,
            ___,___,_O_,_X_,___,___,___,
        ], 24), false);

        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,_O_,___,___,___,___,
            ___,___,_O_,_X_,___,___,___,
            ___,___,_O_,_X_,___,___,___,
            ___,___,_O_,_X_,___,___,___,
        ], 16), true);

        assert_eq!(winning([
            _O_,___,___,___,___,___,___,
            _O_,___,___,___,___,___,___,
            _O_,___,___,___,___,___,___,
            _O_,_X_,___,___,___,___,___,
            _X_,_X_,___,___,___,___,___,
            _O_,_X_,___,___,___,___,___,
        ], 0), true);

        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,_O_,
            ___,___,___,___,___,___,_O_,
            ___,___,___,___,___,___,_O_,
            ___,___,___,___,___,_X_,_O_,
            ___,___,___,___,___,_X_,_X_,
        ], 13), true);
    }

    #[test]
    fn test_winning_horizontal() {
        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            _O_,_O_,_O_,_X_,_X_,_X_,_O_,
        ], 35), false);

        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            _O_,_O_,_O_,_O_,_X_,_X_,_X_,
        ], 35), true);

        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            _O_,_O_,_O_,_X_,_X_,_X_,_X_,
        ], 40), true);

        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,_O_,___,___,___,
            _O_,_O_,_X_,_X_,_X_,_X_,_O_,
        ], 39), true);

        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            _O_,_O_,_X_,_X_,_X_,_X_,_O_,
            _O_,_O_,_X_,_O_,_X_,_X_,_O_,
        ], 33), true);

        assert_eq!(winning([
            _O_,_X_,_X_,_O_,_X_,_O_,_O_,
            _X_,_O_,_X_,_O_,_X_,_O_,_X_,
            _X_,_O_,_X_,_O_,_X_,_O_,_X_,
            _O_,_X_,_O_,_X_,_O_,_X_,_O_,
            _O_,_X_,_O_,_X_,_O_,_X_,_O_,
            _O_,_X_,_O_,_X_,_O_,_X_,_O_,
        ], 0), false);

        assert_eq!(winning([
            _X_,_X_,_X_,_O_,_X_,_O_,_O_,
            _X_,_O_,_X_,_O_,_X_,_O_,_X_,
            _X_,_O_,_X_,_O_,_X_,_O_,_X_,
            _O_,_X_,_O_,_X_,_O_,_X_,_O_,
            _O_,_X_,_O_,_X_,_O_,_X_,_O_,
            _O_,_X_,_O_,_X_,_O_,_X_,_O_,
        ], 1), false);
    }

    #[test]
    fn test_winning_diagonal() {
        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,_O_,___,___,___,___,
            _X_,_O_,_X_,_O_,_X_,_O_,_X_,
            _O_,_X_,_O_,_X_,_O_,_X_,_O_,
        ], 23), false);

        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,___,_O_,___,___,___,
            ___,___,_O_,_X_,___,___,___,
            _X_,_O_,_X_,_O_,_X_,_O_,_X_,
            _O_,_X_,_O_,_X_,_O_,_X_,_O_,
        ], 17), true);

        assert_eq!(winning([
            ___,___,___,___,___,___,___,
            ___,___,___,___,___,___,___,
            ___,___,_X_,___,___,___,___,
            ___,___,_O_,_X_,_O_,___,___,
            _X_,_O_,_X_,_O_,_X_,_O_,_X_,
            _O_,_X_,_O_,_X_,_O_,_X_,_O_,
        ], 16), true);
    }
}
