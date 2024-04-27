use picrs::{Board, Hints, Line};
use picrs::board_sizes::S;

fn main() {
    let mut board = Board::new(S);
    // let r1 = Line(vec![1, 1]);
    // let r2 = Line(vec![5]);
    // let r3 = Line(vec![1, 1]);
    // let r4 = Line(vec![5]);
    // let r5 = Line(vec![1, 1]);
    // let c1 = Line(vec![1, 1]);
    // let c2 = Line(vec![5]);
    // let c3 = Line(vec![1, 1]);
    // let c4 = Line(vec![5]);
    // let c5 = Line(vec![1, 1]);
    let r1 = Line(vec![4]);
    let r2 = Line(vec![1]);
    let r3 = Line(vec![5]);
    let r4 = Line(vec![1]);
    let r5 = Line(vec![4]);
    let c1 = Line(vec![2, 1]);
    let c2 = Line(vec![1, 1, 1]);
    let c3 = Line(vec![1, 1, 1]);
    let c4 = Line(vec![1, 1, 1]);
    let c5 = Line(vec![1, 2]);
    let hints = Hints::new(
        vec![c1, c2, c3, c4, c5],
        vec![r1, r2, r3, r4, r5],
    );
    hints.solve(&mut board);
    board.print();
}
