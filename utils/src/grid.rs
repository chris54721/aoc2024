use crate::vec::Vec2;
use grid::Grid;
use std::cmp::*;

pub fn parse_grid<T: From<char>>(str: &str) -> Grid<T> {
    Grid::from(
        str.trim()
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect::<Vec<Vec<T>>>(),
    )
}

pub fn in_bounds(grid: &Grid<char>, pos: &Vec2) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < grid.rows() as isize && pos.1 < grid.cols() as isize
}

/*** L/R iterator (from SW to NE) ***/

#[derive(Clone)]
pub struct GridDiagLRIter<'a, T> {
    grid: &'a Grid<T>,
    current_start: (usize, usize),
    current_start_rev: (usize, usize),
    done: bool,
}

impl<'a, T> Iterator for GridDiagLRIter<'a, T> {
    type Item = GridDiagLRLineIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let iter = GridDiagLRLineIter {
            grid: self.grid,
            start: self.current_start,
            end: find_diag_end_lr(self.grid, self.current_start),
            done: false,
        };

        if self.current_start == self.current_start_rev {
            self.done = true;
        } else if self.current_start.0 == 0 {
            // We are on the top edge, move right
            self.current_start.1 += 1;
        } else {
            // We are on the left edge, move up
            self.current_start.0 -= 1;
        }

        Some(iter)
    }
}

impl<'a, T> DoubleEndedIterator for GridDiagLRIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let iter = GridDiagLRLineIter {
            grid: self.grid,
            start: self.current_start_rev,
            end: find_diag_end_lr(self.grid, self.current_start_rev),
            done: false,
        };

        if self.current_start == self.current_start_rev {
            self.done = true;
        } else if self.current_start_rev.1 == 0 {
            // We are on the left edge, move down
            self.current_start_rev.0 += 1;
        } else {
            // We are on the top edge, move left
            self.current_start_rev.1 -= 1;
        }

        Some(iter)
    }
}

pub struct GridDiagLRLineIter<'a, T> {
    grid: &'a Grid<T>,
    start: (usize, usize),
    end: (usize, usize),
    done: bool,
}

impl<'a, T> Iterator for GridDiagLRLineIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let item = self.grid.get(self.start.0, self.start.1);

        if self.start == self.end {
            self.done = true;
        } else {
            self.start.0 += 1;
            self.start.1 += 1;
        }

        item
    }
}

impl<'a, T> DoubleEndedIterator for GridDiagLRLineIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let item = self.grid.get(self.end.0, self.end.1);

        if self.start == self.end {
            self.done = true;
        } else {
            self.end.0 -= 1;
            self.end.1 -= 1;
        }

        item
    }
}

pub fn grid_iter_diag_lr<T>(grid: &Grid<T>) -> GridDiagLRIter<T> {
    GridDiagLRIter {
        grid,
        current_start: (grid.rows() - 1, 0),
        current_start_rev: (0, grid.cols() - 1),
        done: false,
    }
}

fn find_diag_end_lr<T>(grid: &Grid<T>, start: (usize, usize)) -> (usize, usize) {
    let diag_len = min(grid.rows() - 1 - start.0, grid.cols() - 1 - start.1);

    (start.0 + diag_len, start.1 + diag_len)
}

#[derive(Clone)]
pub struct GridDiagRLIter<'a, T> {
    grid: &'a Grid<T>,
    current_start: (usize, usize),
    current_start_rev: (usize, usize),
    done: bool,
}

impl<'a, T> Iterator for GridDiagRLIter<'a, T> {
    type Item = GridDiagRLLineIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let iter = GridDiagRLLineIter {
            grid: self.grid,
            start: self.current_start,
            end: find_diag_end_rl(self.grid, self.current_start),
            done: false,
        };

        // Move the starting point for the next diagonal
        if self.current_start == self.current_start_rev {
            self.done = true;
        } else if self.current_start.1 < self.grid.cols() - 1 {
            // If on the top edge, move to the right
            self.current_start.1 += 1;
        } else {
            // If on the right edge, move downward
            self.current_start.0 += 1;
        }

        Some(iter)
    }
}

impl<'a, T> DoubleEndedIterator for GridDiagRLIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let iter = GridDiagRLLineIter {
            grid: self.grid,
            start: self.current_start_rev,
            end: find_diag_end_rl(self.grid, self.current_start_rev),
            done: false,
        };

        // Move the starting point for the previous diagonal
        if self.current_start == self.current_start_rev {
            self.done = true;
        } else if self.current_start_rev.0 > 0 {
            // If on the right edge, move upward
            self.current_start_rev.0 -= 1;
        } else {
            // If on the top edge, move leftward
            self.current_start_rev.1 -= 1;
        }

        Some(iter)
    }
}

pub struct GridDiagRLLineIter<'a, T> {
    grid: &'a Grid<T>,
    start: (usize, usize),
    end: (usize, usize),
    done: bool,
}

impl<'a, T> Iterator for GridDiagRLLineIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let item = self.grid.get(self.start.0, self.start.1);

        if self.start == self.end {
            self.done = true;
        } else {
            self.start.0 += 1;
            self.start.1 -= 1;
        }

        item
    }
}

impl<'a, T> DoubleEndedIterator for GridDiagRLLineIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let item = self.grid.get(self.end.0, self.end.1);

        if self.start == self.end {
            self.done = true;
        } else {
            self.end.0 -= 1;
            self.end.1 += 1;
        }

        item
    }
}

pub fn grid_iter_diag_rl<T>(grid: &Grid<T>) -> GridDiagRLIter<T> {
    GridDiagRLIter {
        grid,
        current_start: (0, 0),
        current_start_rev: (grid.rows() - 1, grid.cols() - 1),
        done: false,
    }
}

fn find_diag_end_rl<T>(grid: &Grid<T>, start: (usize, usize)) -> (usize, usize) {
    let diag_len = min(grid.rows() - 1 - start.0, start.1);

    (start.0 + diag_len, start.1 - diag_len)
}

pub fn print_grid(grid: &Grid<char>) {
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            print!("{}", grid.get(i, j).unwrap());
        }
        println!();
    }
}