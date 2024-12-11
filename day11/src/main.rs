use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./day11/input/input.txt").unwrap();
    let stones: Vec<usize> = input
        .trim()
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    let part1: usize = stones.iter().fold(0, |acc, s| acc + count(*s, 25, &mut cache));
    let part2: usize = stones.iter().fold(0, |acc, s| acc + count(*s, 75, &mut cache));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn count(stone: usize, steps: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if steps == 0 {
        return 1;
    }
    
    if let Some(result) = cache.get(&(stone, steps)) {
        return *result;
    }
    
    let result: usize;

    if stone == 0 {
        result = count(1, steps - 1, cache);
    } else {
        let digits: Vec<char> = stone.to_string().as_str().chars().collect();

        if digits.len() % 2 == 0 {
            result = digits
                .chunks(digits.len() / 2)
                .map(|n| count(chars_to_num(n), steps - 1, cache))
                .sum();
        } else {
            result = count(2024 * stone, steps - 1, cache);
        }   
    }

    cache.insert((stone, steps), result);
    
    result
}

fn chars_to_num<'a>(chars: impl IntoIterator<Item = &'a char>) -> usize {
    let mut num: usize = 0;
    for c in chars {
        num *= 10;
        num += c.to_digit(10).unwrap() as usize;
    }
    num
}
