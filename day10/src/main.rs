use day10::map::Map;
use day10::tile::Tile;
use day10::pipe_loop::Loop;

fn main() {
    // Read file
    let input = std::fs::read_to_string("./day10/input.txt")
        .expect("Couldn't read the input file");

    // Parse the map into a 2D vector.
    let map = Map::parse(&input.lines().collect::<Vec<_>>());

    // Get all the possible starting locations for pipe loops. (2 will be correct)
    let possible_loops = Tile::StartingPosition.get_connections(map.get_starting_location());

    // Get the first loop that passes through the starting position (the pipe
    // loop we want).
    let pipe_loop = possible_loops
        .into_iter()
        .flat_map(|possible_starting_pos| Loop::try_build(possible_starting_pos, &map))
        .next()
        .unwrap();

    // The size of the pipe loop is the number of locations inside the loop.
    // Dividing by 2 is just a quick "hack" to get the farthest apart any 2
    // points can be.
    let part1 = pipe_loop.size() / 2;

    // Count the number of locations on the map that are inside the pipe loop.
    let part2 = map.location_iter()
        .filter(|&loc| pipe_loop.inside_pipe_loop(loc, map.width()))
        .count();

    // Print results
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
