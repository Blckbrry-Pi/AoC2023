use crate::map::Map;
use crate::direction::Directions;
use crate::ident::Ident;
use crate::cycle::Cycle;

pub fn get_cycle(map: &Map, directions: &Directions, start: Ident) -> Cycle {
    let mut curr = start;

    let mut end_z_seen: Option<usize> = None;
    let mut cycle_len: Option<usize> = None;

    for (steps, direction) in directions.clone().into_iter().enumerate() {
        if curr.end_z() {
            if let Some(prev_steps) = end_z_seen {
                cycle_len = Some(steps - prev_steps);
            } else {
                end_z_seen = Some(steps);
            }
        }

        curr = direction.access(&map.nodes[&curr]);

        if cycle_len.is_some() {
            break;
        }
    }

    let length = cycle_len.unwrap();
    let offset = end_z_seen.unwrap() % length;

    Cycle { offset, length }
}

pub fn get_cycles(map: &Map, directions: &Directions, values: Vec<Ident>) -> Vec<Cycle> {
    values.into_iter()
        .map(|ident| get_cycle(map, directions, ident))
        .collect()
}

pub fn get_a_cycles(map: &Map, directions: &Directions) -> Vec<Cycle> {
    let end_a = map.nodes.keys()
        .copied()
        .filter(Ident::end_a)
        .collect::<Vec<_>>();

    get_cycles(map, directions, end_a)
}




fn lcm(a: usize, b: usize) -> usize { a * b / gcd(a, b) }
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { return a; }
    gcd(b, a % b)
}

pub fn get_cycle_sync(cycles: &[Cycle]) -> usize {
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