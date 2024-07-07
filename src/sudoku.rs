//
// Print Sudoku board
// Populate Sudoku board with a valid sudoku
// User input
// Check correctness
//

// pub fn print_board(board: [[i32; 9]; 9]) {
//     //Print board and y axis indices
//     for i in 0..9 {

//         if i == 0 {print_horizontal_line()}

//         for j in 0..9 {
//             if j == 0 {print_index((i + 1) as i32, false); print_vertical_line()}

//             print_number(board[i][j]);

//             if j%3 == 2 {print_vertical_line()}
//         }

//         println!();

//         if i%3 == 2 {print_horizontal_line()}
        
//         else {println!()}
//     }

//     //Print x axis indices
//     print!("   ");
//     for i in 0..9 {
//         if i%3 == 0 {print!(" ")}
//         print_index(i as i32, true);
//     }

// }

// fn print_horizontal_line() {
//     println!("  -------------------------------")
// }

// fn print_number(num: i32) {
//     if num != 0 {print!(" {} ", num)}
//     else {print!(" x ")}
// }

// fn print_vertical_line() {
//     print!("|");
// }

// fn print_index(num: i32, x_coord: bool) {
//     if x_coord {
//         match num {
//             0 => print!("A "),
//             1 => print!(" B "),
//             2 => print!(" C "),
//             3 => print!(" D "),
//             4 => print!(" E "),
//             5 => print!(" F "),
//             6 => print!(" G "),
//             7 => print!(" H "),
//             8 => print!(" I"),
//             _ => panic!("print_index was passed a value outside [0, 8]!")
//         };
//     }
//     else {
//         print!("{} ", num);
//     }
// }

pub fn user_input(board: &mut [[i32; 9]; 9], row: i32, column: i32, input: i32) {

    let x = row - 1;
    let y = column - 1;

    board[x as usize][y as usize] = input;

}

pub fn get_solution(board: &mut [[i32; 9]; 9]) {
    solve_sudoku(board);
}

fn solve_sudoku(board: &mut [[i32; 9]; 9]) -> bool {
    let mut row: usize = 0;
    let mut col: usize = 0;

    if !find_unassigned_location(board, &mut row, &mut col) {
        return true
    }

    for num in 1..=9 {
        if is_safe(*board, row, col, num) {
            board[row][col] = num;
            if solve_sudoku(board) {return true}
            board[row][col] = 0;
        }
    }

    return false
}

fn find_unassigned_location(board: &[[i32; 9]; 9], row_ref: &mut usize, col_ref: &mut usize) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                *row_ref = row;
                *col_ref = col;
                return true
            }
        }
    }
    return false
}

fn used_in_row(board: [[i32; 9]; 9], row: usize, num: i32) -> bool {
    for col in 0..9 {
        if board[row][col] == num {
            return true
        }
    }
    return false
}

fn used_in_col(board: [[i32; 9]; 9], col: usize, num: i32) -> bool {
    for row in 0..9 {
        if board[row][col] == num {
            return true
        }
    }
    return false
}

fn used_in_box(board: [[i32; 9]; 9], box_start_row: usize, box_start_col:usize, num: i32) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if board[row + box_start_row][col + box_start_col] == num {
                return true
            }
        }
    }
    return false
}

fn is_safe(board: [[i32; 9]; 9], row: usize, col: usize, num: i32) -> bool {
    return !used_in_row(board, row, num) 
    && !used_in_col(board, col, num) 
    && !used_in_box(board, row - row % 3, col - col % 3, num)
    && board[row][col] == 0;
}
