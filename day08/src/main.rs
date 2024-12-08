use aocutils::grid::{in_bounds, parse_grid};
use aocutils::vec::Vec2;
use grid::Grid;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("./day08/input/input.txt").unwrap();
    let grid: Grid<char> = parse_grid(input);

    let mut antennas: HashMap<char, Vec<Vec2>> = HashMap::new();
    for (pos, &char) in grid.indexed_iter() {
        if char != '.' {
            antennas.entry(char).or_default().push(Vec2::from(pos));
        }
    }

    let mut antinodes: HashSet<Vec2> = HashSet::new();
    let mut antinodes2: HashSet<Vec2> = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let dist = positions[j] - positions[i];
                let a1 = positions[i] - dist;
                if in_bounds(&grid, &a1) {
                    antinodes.insert(a1);
                }
                let a2 = positions[j] + dist;
                if in_bounds(&grid, &a2) {
                    antinodes.insert(a2);
                }

                let mut c = positions[i] + dist;
                while in_bounds(&grid, &c) {
                    antinodes2.insert(c);
                    c += dist;
                }

                let mut c = positions[j] - dist;
                while in_bounds(&grid, &c) {
                    antinodes2.insert(c);
                    c -= dist;
                }
            }
        }
    }

    println!("Part 1: {}", antinodes.len());
    println!("Part 2: {}", antinodes2.len());
}
