use day10::map::Map;
use day10::tile::Tile;
use day10::pipe_loop::Loop;

fn main() {
    let input = std::fs::read_to_string("./day10/input.txt")
        .expect("Couldn't read the input file");

    let map = Map::parse(&input.lines().collect::<Vec<_>>());

    let possible_loops = Tile::StartingPosition.get_connections(map.get_starting_location());

    let pipe_loop = possible_loops
        .into_iter()
        .flat_map(|possible_starting_pos| Loop::try_build(possible_starting_pos, &map))
        .next()
        .unwrap();

    let part1 = pipe_loop.size() / 2;
    let part2 = map.location_iter()
        .filter(|&loc| pipe_loop.inside_pipe_loop(loc, map.tiles[0].len()))
        .count();


    println!("{:?}", pipe_loop.debug_with_map(&map));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
