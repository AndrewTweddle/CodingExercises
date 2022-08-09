use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let stdin_lock = stdin.lock();
    let mut line_reader = stdin_lock.lines();
    let n = line_reader
        .next()
        .unwrap()
        .unwrap()
        .parse::<i64>()
        .expect("Unable to parse n");
    for k in 1..=n {
        let total_ways_of_placing_two_knights = (k * k) * (k * k - 1) / 2; // k^2 choose 2

        // Count the number of ways of placing two knights to attack each other.
        // Do it from the perspective of the knight with lower rank.

        // There are 4 types of upward moves:
        // - 2 left, 1 up
        // - 1 left, 2 up
        // - 1 right, 2 up
        // - 2 right, 1 up
        // However, by symmetry, there are the same number of positions for each type of move
        // in which the lower knight can attack the upper knight.
        // So just count one of them, say: 2 right, 1 up.
        // The lower knight can be:
        //   i. in any of the k-2 leftmost columns, and
        //  ii. in any of the k-1 bottom-most rows
        let ways_of_attacking_with_one_move_type = (k - 2) * (k - 1);

        let ways_of_placing_two_knights_not_attacking_each_other =
            total_ways_of_placing_two_knights - 4 * ways_of_attacking_with_one_move_type;
        println!("{0}", ways_of_placing_two_knights_not_attacking_each_other);
    }
}
