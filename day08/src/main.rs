use day08::cycles::{get_a_cycles, get_cycle_sync};
use day08::map::Map;
use day08::direction::Directions;
use day08::ident::Ident;


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

fn main() {
    let input = std::fs::read_to_string("./day08/input.txt")
        .expect("Couldn't read the input file");

    let mut map = Map::default();

    // Parse first line for directions.
    let directions = Directions::parse(input.lines().next().unwrap()).unwrap();

    // Parse 3rd to last lines and add them to the map.
    input.lines().skip(2).try_for_each(|line| map.add_line(line).map(|_| ())).unwrap();


    let part1 = part_1(&map, &directions);
    let part2 = get_cycle_sync(&get_a_cycles(&map, &directions));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
