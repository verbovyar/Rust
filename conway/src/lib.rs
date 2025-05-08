#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

fn wrapping_super_add(lhs: usize, rhs: isize) -> Option<usize> {
    if rhs < 0 {
        let rhs_abs = rhs.checked_abs();
        rhs_abs?;
        lhs.checked_sub(rhs_abs.unwrap() as usize)
    } else {
        lhs.checked_add(rhs as usize)
    }
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![T::default(); cols * rows],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        assert_eq!(grid.len(), rows * cols);
        Self {
            rows,
            cols,
            grid: Vec::from(grid),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[self.cols * row + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[self.cols * row + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        assert!(row < self.rows);
        assert!(col < self.cols);

        let mut result: Vec<(usize, usize)> = vec![];

        for drow in -1..=1 {
            for dcol in -1..2 {
                if drow == 0 && dcol == 0 {
                    continue;
                }

                let new_row = wrapping_super_add(row, drow);
                let new_col = wrapping_super_add(col, dcol);
                if new_row.is_none() || new_col.is_none() {
                    continue;
                }

                let new_row_unwrap = new_row.unwrap();
                let new_col_unwrap = new_col.unwrap();

                if new_row_unwrap >= self.rows || new_col_unwrap >= self.cols {
                    continue;
                }

                result.push((new_row_unwrap, new_col_unwrap))
            }
        }

        result
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let mut new_grid: Grid<Cell> = self.get_grid().clone();

        for row in 0..new_grid.rows {
            for col in 0..new_grid.cols {
                let mut num_alive: u8 = 0;
                for neighbour in new_grid.neighbours(row, col) {
                    if *self.grid.get(neighbour.0, neighbour.1) == Cell::Alive {
                        num_alive += 1;
                    }
                }

                let cur_state = self.grid.get(row, col);
                let new_state;

                match *cur_state {
                    Cell::Alive => {
                        if !(2..=3).contains(&num_alive) {
                            new_state = Cell::Dead;
                        } else {
                            assert!(num_alive == 2 || num_alive == 3);
                            new_state = Cell::Alive;
                        }
                    }
                    Cell::Dead => {
                        if num_alive == 3 {
                            new_state = Cell::Alive;
                        } else {
                            new_state = Cell::Dead;
                        }
                    }
                }

                new_grid.set(new_state, row, col);
            }
        }

        self.grid = new_grid;
    }
}
