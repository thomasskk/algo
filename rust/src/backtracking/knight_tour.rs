const MOVES: [(i32, i32); 8] = [
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
    (2, -1),
];

struct Board {
    size: usize,
    board: Vec<Vec<i32>>,
}

impl Board {
    fn new(size: usize) -> Self {
        Board {
            size,
            board: vec![vec![0; size]; size],
        }
    }

    fn is_complete(&self) -> bool {
        self.board.iter().all(|x| !x.contains(&0))
    }
}
