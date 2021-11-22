mod pure_functions;
mod puzzle_actions;

fn main() {
    let _start_puzzle: [[u8; 9]; 9] = [
        [ 4, 1, 0, 0, 0, 0, 0, 0, 0 ],
        [ 0, 0, 3, 0, 0, 0, 0, 2, 9 ],
        [ 0, 0, 0, 0, 0, 4, 0, 6, 0 ],
        [ 0, 0, 0, 7, 0, 0, 0, 9, 0 ],
        [ 0, 0, 7, 4, 0, 0, 0, 0, 2 ],
        [ 0, 0, 0, 0, 0, 8, 0, 0, 5 ],
        [ 6, 7, 0, 0, 0, 1, 0, 0, 0 ],
        [ 0, 0, 9, 0, 2, 0, 0, 0, 3 ],
        [ 0, 3, 0, 0, 0, 9, 0, 5, 0 ]
    ];

    let test_list: Vec<u8> = (6..10).collect();
    let remaining_digits = pure_functions::get_missing_digits(test_list);
    for number in remaining_digits.iter() {
        println!("{}", number)
    }
}
