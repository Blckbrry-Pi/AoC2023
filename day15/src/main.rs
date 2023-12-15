use day15::{*, boxlist::BoxList};

fn main() {
    let input = std::fs::read_to_string("./day15/input.txt")
        .expect("Couldn't read the input file");

    let part1 = input.split(',').map(hasher::Hasher::hash_val_of_str_256).sum::<usize>();

    let mut boxes = BoxList::empty();
    input.split(',').map(From::from).for_each(|op| boxes.run_op(op));

    let part2 = boxes.total();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
