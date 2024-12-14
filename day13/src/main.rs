use aocutils::vec::Vec2;
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("./day13/input/input.txt").unwrap();

    let re_button = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|m| {
            let mut lines = m.lines();
            let captures_a = re_button.captures(lines.next().unwrap()).unwrap();
            let captures_b = re_button.captures(lines.next().unwrap()).unwrap();
            let captures_prize = re_prize.captures(lines.next().unwrap()).unwrap();

            Machine {
                button_a: Vec2(
                    captures_a[1].parse().unwrap(),
                    captures_a[2].parse().unwrap(),
                ),
                button_b: Vec2(
                    captures_b[1].parse().unwrap(),
                    captures_b[2].parse().unwrap(),
                ),
                prize: Vec2(
                    captures_prize[1].parse().unwrap(),
                    captures_prize[2].parse().unwrap(),
                ),
            }
        })
        .collect();

    let total1: usize = machines.iter().filter_map(|m| m.min_tokens(false)).sum();
    let total2: usize = machines.iter().filter_map(|m| m.min_tokens(true)).sum();

    println!("Part 1: {}", total1);
    println!("Part 2: {}", total2);
}

#[derive(Debug)]
struct Machine {
    button_a: Vec2,
    button_b: Vec2,
    prize: Vec2,
}

impl Machine {
    fn min_tokens(&self, offset: bool) -> Option<usize> {
        let mut target = self.prize;
        if offset {
            target = self.prize + Vec2(10000000000000, 10000000000000)
        }
        
        let det = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;
        let b = (target.1 * self.button_a.0 - target.0 * self.button_a.1) / det;
        let a = (target.0 - b * self.button_b.0) / self.button_a.0;
        
        if a * self.button_a + b * self.button_b != target {
            return None;
        }
        Some((a * 3 + b) as usize)
    }
}
