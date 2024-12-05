use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./day05/input/input.txt").unwrap();
    let (rules_str, pages_str) = input.split_once("\n\n").unwrap();

    let rules: HashSet<(i32, i32)> = HashSet::from_iter(rules_str.trim().split("\n").map(|l| {
        let (n1, n2) = l.split_once("|").unwrap();
        (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap())
    }));

    let pages = pages_str
        .trim()
        .split("\n")
        .map(|l| {
            l.split(",")
                .map(|p| p.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut total1 = 0;
    let mut total2 = 0;
    for page in &pages {
        let initial_mid = page[(page.len() - 1) / 2];
        let mut valid = true;
        'p: for j in 0..page.len() {
            for k in j + 1..page.len() {
                if rules.contains(&(page[k], page[j])) {
                    valid = false;
                    break 'p;
                }
            }
        }
        if valid {
            total1 += initial_mid;
        } else {
            let mut reordered = vec![page[0]];
            for p in page.iter().skip(1) {
                let mut pos = reordered.len();
                for (i, r) in reordered.iter().enumerate() {
                    if rules.contains(&(*r, *p)) {
                        pos = i;
                        break;
                    }
                }
                reordered.insert(pos, *p);
            }

            total2 += reordered[(reordered.len() - 1) / 2];
        }
    }

    println!("Part 1: {}", total1);
    println!("Part 2: {}", total2);
}
