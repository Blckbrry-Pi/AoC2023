use day13::pattern::{Pattern, Axis};

fn main() {
    let input = std::fs::read_to_string("./day13/input.txt")
        .expect("Couldn't read the input file");

    let patterns: Vec<_> = input.split("\n\n").map(Pattern::parse).collect();

    let part1 = patterns.iter().map(Pattern::get_reflection_clear).map(Axis::value).sum::<usize>();
    let part2 = patterns.iter().map(Pattern::get_reflection_1_smudge).map(Axis::value).sum::<usize>();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
