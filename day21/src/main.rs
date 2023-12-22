use std::collections::HashSet;

use day21::elf_state::ElfState;
use day21::map::Map;
use day21::step::Step;


fn main() {
    let input = std::fs::read_to_string("./day21/input.txt")
        .expect("Couldn't read the input file");

    let map = Map::parse(&input);

    let mut curr_positions: HashSet<_> = [map.start_pos().unwrap()].into_iter().collect();
    for _ in 0..64 {
        curr_positions = map.take_steps(curr_positions, Step::orthogonal());
    }
    let part1 = curr_positions.len();

    let mut elf_state = ElfState::new(map.start_pos().unwrap());

    let mut step_time = std::time::Duration::ZERO;
    let mut reduce_time = std::time::Duration::ZERO;

    for i in 0..26501365 {
        if i % 1000 == 0 {
            println!("{i}");
        }
        let start = std::time::Instant::now();
        elf_state.step(&map, i % 2 == 0);
        let stepped = std::time::Instant::now();
        if i % 5 == 0 {
            elf_state.reduce(&map);
        }
        let end = std::time::Instant::now();

        step_time += stepped.duration_since(start);
        reduce_time += end.duration_since(stepped);
    }
    let part2 = elf_state.count(true);

    println!("chunks: {}, individuals: {}", elf_state.count_chunks(true), elf_state.count_individuals(true));
    println!("step: {:.4}s, reduce: {:.4}s", step_time.as_secs_f64(), reduce_time.as_secs_f64() * 10.0);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
