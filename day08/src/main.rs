use std::collections::HashMap;

use day08::map::Map;
use day08::cycle::Cycle;
use day08::direction::Directions;
use day08::ident::Ident;
use day08::state::State;

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


fn part_1(map: &Map, directions: &Directions) -> usize {
    let mut curr = map.start;
    for (steps, direction) in directions.clone().into_iter().enumerate() {
        curr = direction.access(&map.nodes[&curr]);

        if curr == Ident::ZZZ {
            return steps + 1;
        }
    }

    unreachable!()
}

fn get_cycles(map: &Map, directions: &Directions, values: Vec<Ident>) -> Vec<Cycle> {
    let mut curr = values.clone();

    let mut z_seen: Vec<HashMap<State, usize>> = vec![HashMap::new(); curr.len()];
    let mut cycle_len: Vec<Option<usize>> = vec![None; curr.len()];

    for (steps, (dir_idx, direction)) in directions.clone().into_indexed_iter().enumerate() {

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

        if cycle_len.iter().all(Option::is_some) {
            break;
        }
    }

    cycle_len.into_iter()
        .zip(z_seen)
        .map(|(cycle, map)| {
            Cycle {
                offset: *map.values().next().unwrap(),
                length: cycle.unwrap()
            }
        })
        .collect()
}

fn get_cycle_sync(cycles: &[Cycle]) -> usize {
    let mut curr_val = cycles[0].offset;
    let mut current_lcm = cycles[0].length;
    
    for cycle in cycles.iter().copied().skip(1) {
        while curr_val % cycle.length != cycle.offset % cycle.length {
            curr_val += current_lcm;
        }

        current_lcm = lcm(current_lcm, cycle.length);
    }

    curr_val
}

fn main() {
    let input = std::fs::read_to_string("./day08/input.txt")
        .expect("Couldn't read the input file");

    let mut map = Map::default();

    // Parse first line for directions.
    let directions = Directions::parse(input.lines().next().unwrap()).unwrap();

    // Parse 3rd to last lines and add them to the map.
    input.lines().skip(2).try_for_each(|line| map.add_line(line).map(|_| ())).unwrap();

    let part1 = part_1(&map, &directions);


    let part2_idents: Vec<_> = map.nodes
        .keys()
        .copied()
        .filter(|ident| ident.last_char() == 'A')
        .collect();
    let part2_cycles = get_cycles(&map, &directions, part2_idents);
    let part2 = get_cycle_sync(&part2_cycles);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
