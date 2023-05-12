mod sudoku;
mod blank_cell;
mod pure_functions;

use sudoku::Sudoku;

// TODO: Add solve for an array of u8s.
pub fn solve_file(input_sudoku_file: String, verbose: bool) -> Result<String, &'static str> {
    if verbose {
        println!("\n");
    }
    let start_puzzle = Sudoku::from_file(input_sudoku_file)?;
    let output = start_puzzle.get_valid_solutions(verbose);
    let output_sudoku_file = output[0].to_string();
    if verbose {
        println!("\nFinished Puzzle:\n\n{}\n", output_sudoku_file);
    }
    Ok(output_sudoku_file)
}

pub fn solve(input_values: [u8; 81], verbose: bool) -> Result<[u8; 81], &'static str> {
    if verbose {
        println!("\n");
    }
    let output_values = Sudoku::from(input_values)?
        .get_valid_solutions(verbose)[0]
        .numbers;
    if verbose {
        println!("\nFinished Puzzle:\n\n{:?}\n", output_values);
    }
    Ok(output_values)
}