mod board;
mod funcs;

fn main() {
    let mut board = board::Board::new();
    board.add_rand_block();
    loop {
        funcs::print_screen(&board);
        funcs::get_key_input(&mut board);
        board.add_rand_block();
    }
}

// Loop{
//     PrintBoard()
//     Get key input
//     Changeboard/update
//     check win condition
// }
