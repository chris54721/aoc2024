use crate::GuardResult::{Exited, Loop};
use grid::Grid;
use std::collections::HashSet;
use std::fs;
use crate::GuardStepResult::Stepped;

fn main() {
    let input = fs::read_to_string("./day06/input/input.txt").unwrap();
    let grid: Grid<char> = Grid::from(
        input
            .trim()
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    );

    let initial_state = State {
        pos: grid.indexed_iter().find(|(_, &x)| x == '^').unwrap().0,
        dir: (-1, 0),
    };

    let Exited(visited) = run_guard(&grid, initial_state) else {
        panic!("Invalid grid")
    };

    let mut total2 = 0;
    for &pos in visited.iter() {
        if pos == initial_state.pos {
            continue;
        }
        let mut updated_grid = grid.clone();
        updated_grid[pos] = '#';

        if let Loop = run_guard(&updated_grid, initial_state) {
            total2 += 1;
        }
    }

    println!("Part 1: {}", visited.len());
    println!("Part 2: {}", total2);
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct State {
    pos: (usize, usize),
    dir: (isize, isize),
}

#[derive(Debug)]
enum GuardResult {
    Exited(HashSet<(usize, usize)>),
    Loop,
}

#[derive(Debug)]
enum GuardStepResult {
    Stepped(State),
    Exited,
}

fn run_guard(grid: &Grid<char>, initial_state: State) -> GuardResult {
    let mut state = initial_state;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(initial_state.pos);

    let mut visited_states: HashSet<State> = HashSet::new();
    visited_states.insert(initial_state);

    loop {
        let step_result = step(grid, &state);
        if let Stepped(new_state) = step_result {
            visited.insert(new_state.pos);
            if !visited_states.insert(new_state) {
                return Loop;
            }
            state = new_state;
        } else {
            return Exited(visited);
        }
    }
}

fn step(grid: &Grid<char>, state: &State) -> GuardStepResult {
    let max_row = (grid.rows() - 1) as isize;
    let max_col = (grid.cols() - 1) as isize;
    
    let next_pos = (state.pos.0 as isize + state.dir.0, state.pos.1 as isize + state.dir.1);
    if next_pos.0 < 0 || next_pos.0 > max_row || next_pos.1 < 0 || next_pos.1 > max_col {
        return GuardStepResult::Exited;
    }
    let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

    if grid[next_pos] != '#' {
        Stepped(State {
            pos: next_pos,
            dir: state.dir,
        })
    } else {
        Stepped(State {
            pos: state.pos,
            dir: match state.dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            }
        })
    }
}
