use std::fs;

fn main() {
    let input = fs::read_to_string("./day02/input/input.txt").unwrap();

    let mut safe = 0;
    let mut safe2 = 0;

    for line in input.trim().split("\n") {
        let levels_iter = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
        
        if is_safe(levels_iter.clone()) {
            safe += 1;
            safe2 += 1;
        } else {
            for i in 0..levels_iter.clone().count() {
                let levels_iter_filtered = levels_iter.clone()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, l)| l);
                
                if is_safe(levels_iter_filtered) {
                    safe2 += 1;
                    break;
                }
            } 
        }
    }

    println!("Part 1: {}", safe);
    println!("Part 2: {}", safe2);
}

fn is_safe(levels_iter: impl Iterator<Item = i32> + Clone) -> bool {
    let levels_iter_skip = levels_iter.clone().skip(1);

    let diff_iter = levels_iter.zip(levels_iter_skip).map(|(a, b)| b - a);

    let mut invalid = false;
    let mut increasing = 0;
    let mut decreasing = 0;
    for delta in diff_iter {
        if !(1..=3).contains(&delta.abs()) {
            invalid = true;
            break;
        }

        if delta > 0 {
            increasing += 1;
        } else {
            decreasing += 1;
        }
    }

    !invalid && (increasing == 0 || decreasing == 0)
}