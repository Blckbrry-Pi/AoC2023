use std::collections::HashMap;

use day8::Map;
use day8::direction::Directions;
use day8::ident::Ident;
use day8::state::State;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let input = std::fs::read_to_string("./day8/input.txt")
    // let input = std::fs::read_to_string("./day8/test.txt")
        .expect("Couldn't read the input file");

    let directions = Directions::parse(input.split_once('\n').unwrap().0).unwrap();
    let mut map = Map::default();

    input.lines().skip(2).map(|line| map.add_line(line)).collect::<Option<Vec<_>>>().unwrap();

    let mut final_steps_p1 = None;
    let mut curr = map.start;
    for (steps, direction) in directions.clone().into_iter().enumerate() {
        curr = direction.access(&map.nodes[&curr]);

        if curr == Ident::ZZZ {
            final_steps_p1 = Some(steps + 1);
            break;
        }
    }

    let mut final_steps_p2 = None;
    let mut curr: Vec<_> = map.nodes
        .keys()
        .copied()
        .filter(|ident| ident.last_char() == 'A')
        .collect();

    curr.sort();



    let mut z_seen: Vec<HashMap<State, usize>> = vec![HashMap::new(); curr.len()];
    let mut cycle_len: Vec<Option<usize>> = vec![None; curr.len()];

    for (steps, (dir_idx, direction)) in directions.into_indexed_iter().enumerate() {

        let part_iter = curr.iter().copied().zip(z_seen.iter_mut()).zip(cycle_len.iter_mut());

        for ((ident, z_seen_map), cycle_len) in part_iter {
            let state = State::new(ident, dir_idx);

            if ident.last_char() == 'Z' {
                let prev = z_seen_map.insert(state, steps);
                if let Some(prev) = prev {
                    *cycle_len = Some(steps - prev);
                }
            }
        }

        for ident in curr.iter_mut() {
            *ident = direction.access(&map.nodes[&ident]);
        }

        let count = curr.iter().copied().filter(|ident| ident.last_char() == 'Z').count();

        if count == curr.len() {
            final_steps_p2 = Some(steps + 1);
            break;
        }

        if cycle_len.iter().all(Option::is_some) {
            break;
        }
    }

    if final_steps_p2.is_none() {
        let mut curr_val = *z_seen[0].values().next().unwrap();
        let mut current_lcm = cycle_len[0].unwrap();
        
        for i in 1..curr.len() {
            let val = z_seen[i].values().next().unwrap();
            let len = cycle_len[i].unwrap();

            while curr_val % len != val % len {
                curr_val += current_lcm;
            }

            current_lcm = lcm(current_lcm, len);
        }


        final_steps_p2 = Some(curr_val);
    }

    println!("Part 1: {}", final_steps_p1.unwrap());
    println!("Part 2: {}", final_steps_p2.unwrap());
}
