pub mod board_sizes {
    use super::Size;
    pub const S: Size = Size {width: 5, height: 5};
    pub const M: Size = Size {width: 10, height: 10};
    pub const L: Size = Size {width: 15, height: 15};
    pub const XL: Size = Size {width: 20, height: 20};
    pub const XXL: Size = Size {width: 25, height: 25};
}
#[derive(Copy, Clone, Debug, Default)]
enum CellState {
    #[default]
    Unknown,
    Solid,
    Space,
}
#[derive(Debug)]
pub struct Board {
    size: Size,
    inner: Vec<CellState>,
}
#[derive(Debug)]
pub struct Size {
    width: u8,
    height: u8,
}
impl Board {
    pub fn new(size: Size) -> Self {
        let width = size.width;
        let height = size.height;
        Self {
            size,
            inner: vec![CellState::Unknown; width as usize * height as usize],
        }
    } 
    pub fn print(&self) {
        for i in 0..self.size.height as usize {
            for j in 0..self.size.width as usize {
                let cell = self.inner[i * self.size.width as usize + j];
                print!(" {} ", match cell {
                    CellState::Unknown => '?',
                    CellState::Solid  => '¤',
                    CellState::Space  => '_',
                });
            }
            println!();
        }
    }
    fn update_column(&mut self, col_idx: usize, indices: Indices) {
        let stride = self.size.width as usize;
        for i in indices.0 {
            self.inner[col_idx + i * stride] = CellState::Solid;
        }
    }
    fn update_row(&mut self, row_idx: usize, indices: Indices) {
        let stride = self.size.width as usize;
        for i in indices.0 {
            self.inner[row_idx * stride + i] = CellState::Solid;
        }
    }
}

struct Indices(Vec<usize>);

pub struct Line(pub Vec<u8>);
impl Line {
    fn solve(&self, target_length: usize) -> Indices {
        let total_solids: usize = self.0.iter().map(|c| *c as usize).sum();
        let total_occupancy = total_solids + self.0.len() - 1;
        let size_to_ensure_solid = target_length - total_occupancy;
        let mut indices = vec![];
        let mut idx = 0;
        for hint in self.0.iter() {
            let hint = *hint as usize;          
            idx += hint;
            if hint > size_to_ensure_solid {
                let safe_solids = hint - size_to_ensure_solid;
                for i in 0..safe_solids {
                    indices.push(idx - i - 1)
                }
            }
            idx += 1;
        }
        Indices(indices)
    }
}
pub struct Hints {
    columns: Vec<Line>,
    rows: Vec<Line>,
}

impl Hints {
    pub fn solve(&self, board: &mut Board) {
        assert!(self.columns.len() == board.size.width as usize);
        assert!(self.rows.len() == board.size.height as usize);
        for (i, col) in self.columns.iter().enumerate() {
            let inds = col.solve(board.size.height as usize);
            board.update_column(i, inds);
        }
        for (i, row) in self.rows.iter().enumerate() {
            let inds = row.solve(board.size.width as usize);
                board.update_row(i, inds);
        }
    }
    pub fn new(cols: Vec<Line>, rows: Vec<Line>) -> Self {
        Self {
            columns: cols,
            rows,
        }
    }
}
