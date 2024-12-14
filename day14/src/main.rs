use aocutils::vec::Vec2;
use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string("./day14/input/input.txt").unwrap();

    let mut robot_states: Vec<RobotState> = input
        .lines()
        .map(|l| {
            let mut values = l
                .split(|c: char| !c.is_numeric() && c != '-')
                .filter_map(|s| s.parse::<isize>().ok());
            RobotState {
                pos: Vec2(values.next().unwrap(), values.next().unwrap()),
                vel: Vec2(values.next().unwrap(), values.next().unwrap()),
            }
        })
        .collect();

    let mut bounds = Vec2(0, 0);
    for robot in robot_states.iter() {
        if robot.pos.0 > bounds.0 {
            bounds.0 = robot.pos.0;
        }
        if robot.pos.1 > bounds.1 {
            bounds.1 = robot.pos.1;
        }
    }

    for _ in 0..100 {
        for robot in robot_states.iter_mut() {
            simulate(robot, bounds);
        }
    }

    let total1: usize = quadrant_counts(&robot_states, bounds).iter().product();

    println!("Part 1: {total1}");

    let mut step = 100;
    'tree: loop {
        step += 1;
        for robot in robot_states.iter_mut() {
            simulate(robot, bounds);
        }

        let mut states: Vec<_> = robot_states.iter().map(|s| s.pos).collect();
        states.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        let mut consecutive = 0;

        for (a, b) in states.iter().zip(states.iter().skip(1)) {
            if a.0 == b.0 && b.1 - a.1 == 1 {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive == 20 {
                break 'tree;
            }
        }
    }

    println!("Part 2: {step}");
}

#[derive(Debug)]
struct RobotState {
    pos: Vec2,
    vel: Vec2,
}

fn simulate(state: &mut RobotState, bounds: Vec2) {
    let mut new_pos = state.pos + state.vel;
    if new_pos.0 < 0 {
        new_pos.0 = bounds.0 + new_pos.0 + 1;
    }
    if new_pos.0 > bounds.0 {
        new_pos.0 = new_pos.0 - bounds.0 - 1;
    }
    if new_pos.1 < 0 {
        new_pos.1 = bounds.1 + new_pos.1 + 1;
    }
    if new_pos.1 > bounds.1 {
        new_pos.1 = new_pos.1 - bounds.1 - 1;
    }

    state.pos = new_pos;
}

fn quadrant_counts(states: &[RobotState], bounds: Vec2) -> [usize; 4] {
    let midpoint = bounds / 2isize;

    let quadrants_iter =
        states.iter().filter_map(
            |s| match (s.pos.0.cmp(&midpoint.0), s.pos.1.cmp(&midpoint.1)) {
                (Ordering::Less, Ordering::Less) => Some(0),
                (Ordering::Less, Ordering::Greater) => Some(1),
                (Ordering::Greater, Ordering::Less) => Some(2),
                (Ordering::Greater, Ordering::Greater) => Some(3),
                _ => None,
            },
        );

    let mut quadrant_counts: [usize; 4] = [0; 4];
    for q in quadrants_iter {
        quadrant_counts[q] += 1;
    }

    quadrant_counts
}
