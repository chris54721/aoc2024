use std::fs;

fn main() {
    let input = fs::read_to_string("./day01/input/input.txt").unwrap();

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
    }

    list1.sort();
    list2.sort();

    let difference: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    let part2: i32 = list1
        .iter()
        .map(|&a| a * list2.iter().filter(|&&b| a == b).count() as i32)
        .sum();

    println!("Part 1: {}", difference);
    println!("Part 2: {}", part2);
}
