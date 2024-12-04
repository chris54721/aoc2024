use aocutils::grid::*;
use grid::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("./day04/input/input.txt").unwrap();
    let grid: Grid<char> = Grid::from(
        input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    );

    let matches_rows: usize = count_xmas_all(grid.iter_rows());
    let matches_cols: usize = count_xmas_all(grid.iter_cols());
    let matches_diag_lr: usize = count_xmas_all(grid_iter_diag_lr(&grid));
    let matches_diag_rl: usize = count_xmas_all(grid_iter_diag_rl(&grid));

    let total1 = matches_rows + matches_cols + matches_diag_lr + matches_diag_rl;

    let mut total2 = 0;
    for r in 1..grid.rows() - 1 {
        for c in 1..grid.cols() - 1 {
            if *grid.get(r, c).unwrap() == 'A' {
                let nw = *grid.get(r - 1, c - 1).unwrap();
                let ne = *grid.get(r - 1, c + 1).unwrap();
                let sw = *grid.get(r + 1, c - 1).unwrap();
                let se = *grid.get(r + 1, c + 1).unwrap();

                if ((nw == 'M' && se == 'S') || (nw == 'S' && se == 'M'))
                    && ((ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M'))
                {
                    total2 += 1;
                }
            }
        }
    }

    println!("Part 1: {}", total1);
    println!("Part 2: {}", total2);
}

fn count_xmas_all<'a>(
    iter: impl Iterator<Item = impl DoubleEndedIterator<Item = &'a char>> + Clone,
) -> usize {
    let count_forwards = iter.clone().map(|l| count_xmas(l)).sum::<usize>();
    let count_backwards = iter.map(|l| count_xmas(l.rev())).sum::<usize>();

    count_forwards + count_backwards
}

fn count_xmas<'a>(char_iter: impl Iterator<Item = &'a char>) -> usize {
    let mut count = 0;
    let mut n = 0;

    for char in char_iter {
        if *char == 'X' {
            n = 1;
            continue;
        }

        let matching = match n {
            1 => *char == 'M',
            2 => *char == 'A',
            3 => *char == 'S',
            _ => false,
        };

        if matching {
            n += 1;
        } else {
            n = 0;
        }

        if n == 4 {
            count += 1;
            n = 0;
        }
    }

    count
}
