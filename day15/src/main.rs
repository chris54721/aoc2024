use aocutils::grid::parse_grid;
use aocutils::vec::Vec2;
use grid::Grid;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("./day15/input/input.txt").unwrap();
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut grid: Grid<char> = parse_grid(grid_str);

    let moves: Vec<Vec2> = moves_str
        .trim()
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Vec2(-1, 0)),
            '>' => Some(Vec2(0, 1)),
            'v' => Some(Vec2(1, 0)),
            '<' => Some(Vec2(0, -1)),
            _ => None,
        })
        .collect();

    let mut pos: Vec2 = grid
        .indexed_iter()
        .find_map(|(pos, c)| if *c == '@' { Some(pos.into()) } else { None })
        .unwrap();

    let mut pos2 = Vec2(pos.0, pos.1 * 2);

    grid[pos.as_tuple_unsigned()] = '.';

    let mut wide_grid = Grid::new(grid.rows(), grid.cols() * 2);
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            let tiles: [char; 2] = match grid[(i, j)] {
                '#' => ['#', '#'],
                'O' => ['[', ']'],
                '.' => ['.', '.'],
                c => panic!("Invalid tile {}", c),
            };

            wide_grid[(i, j * 2)] = tiles[0];
            wide_grid[(i, j * 2 + 1)] = tiles[1];
        }
    }

    for dir in moves {
        make_move(&mut grid, &mut pos, dir);
        make_move_wide(&mut wide_grid, &mut pos2, dir);
    }
    
    let total1 = box_gps_sum(&grid, false);
    let total2 = box_gps_sum(&wide_grid, true);

    println!("Part 1: {}", total1);
    println!("Part 2: {}", total2);
}

fn make_move(grid: &mut Grid<char>, pos: &mut Vec2, dir: Vec2) {
    let mut cur = *pos + dir;
    let mut box_found = false;
    loop {
        let c = grid[cur.as_tuple_unsigned()];

        if c == '#' {
            return;
        }
        if c == 'O' {
            box_found = true;
            cur += dir;
            continue;
        }

        // Found
        *pos += dir;
        if box_found {
            grid[cur.as_tuple_unsigned()] = 'O';
            grid[pos.as_tuple_unsigned()] = '.';
        }
        break;
    }
}

fn make_move_wide(grid: &mut Grid<char>, pos: &mut Vec2, dir: Vec2) {
    let mut cursors: HashSet<Vec2> = HashSet::from([*pos + dir]);
    let mut replacements: HashMap<Vec2, char> = HashMap::new();
    loop {
        let mut new_cursors = HashSet::new();
        let mut free = 0;
        for cur in cursors.clone() {
            let c = grid[cur.as_tuple_unsigned()];

            if c == '#' {
                return;
            }

            if c == '[' || c == ']' {
                let other_box_tile_pos = cur + Vec2(0, if c == '[' { 1 } else { -1 });
                let other_box_tile = if c == '[' { ']' } else { '[' };

                replacements.insert(cur, grid[(cur - dir).as_tuple_unsigned()]);
                replacements.insert(cur + dir, c);
                replacements.insert(other_box_tile_pos + dir, other_box_tile);

                if dir.0 == 0 {
                    // Horizontal
                    new_cursors.insert(cur + dir * 2);
                } else {
                    // Vertical
                    new_cursors.insert(cur + dir);
                    new_cursors.insert(other_box_tile_pos + dir);

                    let behind_other_box_tile_pos = other_box_tile_pos - dir;
                    if replacements.contains_key(&behind_other_box_tile_pos) {
                        // The tile behind is being moved
                        replacements.insert(other_box_tile_pos, grid[behind_other_box_tile_pos.as_tuple_unsigned()]);
                    } else {
                        // The tile behind is NOT being moved, leave empty space
                        replacements.insert(other_box_tile_pos, '.');
                    }
                }

                continue;
            }

            free += 1;
        }
        if free == cursors.len() {
            // Can move
            *pos += dir;
            for (loc, replacement) in replacements {
                grid[loc.as_tuple_unsigned()] = replacement;
            }
            return;
        }
        cursors = new_cursors;
    }
}

fn box_gps_sum(grid: &Grid<char>, wide: bool) -> usize {
    let box_char = if wide { '[' } else { 'O' };
    grid.indexed_iter()
        .filter(|(_, &c)| c == box_char)
        .fold(0, |acc, (pos, _)| acc + 100 * pos.0 + pos.1)
}
