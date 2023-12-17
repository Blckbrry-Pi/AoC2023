use day16::state::State;
use day16::map::Map;

fn main() {
    let input = std::fs::read_to_string("./day16/input.txt")
        .expect("Couldn't read the input file");

    let map = Map::parse(&input);

    let curr_state = State::new(&map).with_on_left(0).step_until_done(&map);

    let left_iter = (0..map.height()).map(|y| State::new(&map).with_on_left(y).step_until_done(&map).count());
    let top_iter = (0..map.width()).map(|x| State::new(&map).with_on_top(x).step_until_done(&map).count());
    let right_iter = (0..map.height()).map(|y| State::new(&map).with_on_right(y).step_until_done(&map).count());
    let bottom_iter = (0..map.width()).map(|x| State::new(&map).with_on_bottom(x).step_until_done(&map).count());

    let part1 = curr_state.count();
    let part2 = left_iter.chain(top_iter).chain(right_iter).chain(bottom_iter).max().unwrap(); // TODO: Part 2

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
