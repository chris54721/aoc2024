use aocutils::grid::{in_bounds, parse_grid};
use aocutils::vec::Vec2;
use grid::Grid;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./day12/input/input.txt").unwrap();
    let grid: Grid<char> = parse_grid(input);

    let polygons = get_polygons(&grid);

    let part1: usize = polygons.iter().map(|p| perimeter(p) * p.len()).sum();
    let part2: usize = polygons.iter().map(|p| sides(p) * p.len()).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn get_polygons(grid: &Grid<char>) -> Vec<Vec<Vec2>> {
    let mut polygons: Vec<Vec<Vec2>> = Vec::new();
    let mut assigned_pos: HashSet<Vec2> = HashSet::new();
    while assigned_pos.len() < grid.rows() * grid.cols() {
        let first_unassigned: Vec2 = grid
            .indexed_iter()
            .map(|(pos, _)| Vec2::from(pos))
            .find(|pos| !assigned_pos.contains(pos))
            .unwrap();

        assigned_pos.insert(first_unassigned);

        polygons.push(expand_polygon(
            vec![first_unassigned],
            grid,
            &mut assigned_pos,
        ))
    }

    polygons
}

fn expand_polygon(
    polygon: Vec<Vec2>,
    grid: &Grid<char>,
    assigned_pos: &mut HashSet<Vec2>,
) -> Vec<Vec2> {
    let char = grid[polygon.first().unwrap().as_tuple_unsigned()];

    let mut new_items = Vec::new();

    for pos in &polygon {
        for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_pos = *pos + Vec2::from(delta);
            if in_bounds(grid, &new_pos)
                && grid[new_pos.as_tuple_unsigned()] == char
                && !assigned_pos.contains(&new_pos)
            {
                new_items.push(new_pos);
                assigned_pos.insert(new_pos);
            }
        }
    }

    if !new_items.is_empty() {
        let mut new_polygon: Vec<Vec2> = polygon.clone();
        new_polygon.append(&mut new_items);
        expand_polygon(new_polygon, grid, assigned_pos)
    } else {
        polygon
    }
}

fn perimeter(polygon: &[Vec2]) -> usize {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter().map(|dir| perimeter_dir(polygon, Vec2::from(*dir)))
        .sum()
}

fn perimeter_dir(polygon: &[Vec2], dir: Vec2) -> usize {
    outer_positions_dir(polygon, dir).len()
}

fn outer_positions_dir(polygon: &[Vec2], dir: Vec2) -> Vec<Vec2> {
    polygon.iter().filter(|&&pos| !polygon.contains(&(pos + dir))).cloned().collect()
}

fn sides(polygon: &[Vec2]) -> usize {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter().map(|dir| sides_dir(polygon, Vec2::from(*dir)))
        .sum()
}

fn sides_dir(polygon: &[Vec2], dir: Vec2) -> usize {
    let mut outer_pos = outer_positions_dir(polygon, dir);

    // (current, orthogonal) direction
    let (c, o) = if dir.0 == 0 { (1, 0) } else { (0, 1) };

    outer_pos.sort_by(|a, b| a[c].cmp(&b[c]).then(a[o].cmp(&b[o])));

    let lines = outer_pos.chunk_by(|a, b| a[c] == b[c]);

    let mut total = 0;
    for line in lines {
        total += 1 + line.iter()
            .zip(line.iter().skip(1))
            .filter(|&(&a, &b)| b[o] - a[o] > 1)
            .count()
    }

    total
}