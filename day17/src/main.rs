use day17::map::Map;
use day17::paths::PathSet;
use day17::state::{State, Direction};

fn main() {
    let input = std::fs::read_to_string("./day17/input.txt")
        .expect("Couldn't read the input file");

    let map = Map::parse(&input);
    let mut state = PathSet::new(State::new(0, 0, Direction::Right, 0));
    let mut part_2_state = PathSet::new(State::new(0, 0, Direction::Right, 0));

    let mut i = 0;
    while let Some(_states) = state.do_step(&map) {
        if i % 100_000 == 0 {
            // println!("{states} states left");
        }
        i += 1;
    }
    let part1 = state.get_min(map.width() - 1, map.height() - 1).unwrap();
    println!("Part 1: {part1}");

    let mut i = 0;
    while let Some(_states) = part_2_state.do_step_ultra(&map) {
        if i % 100_000 == 0 {
            // println!("{states} states left");
        }
        i += 1;
    }
    let part2 = part_2_state.get_min_ultra(map.width() - 1, map.height() - 1).unwrap();

    println!("Part 2: {part2}");
}
