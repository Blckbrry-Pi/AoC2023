use day7::{HandBidPair, IN_PART_2};


fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let mut pairs: Vec<_> = lines.map(HandBidPair::parse_line).collect();

    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let mut total = 0;

    for (idx, HandBidPair(_, bid)) in pairs.into_iter().enumerate() {
        total += bid * (idx + 1);
    }

    total
}

fn main() {
    let input = std::fs::read_to_string("./day7/input.txt")
        .expect("Couldn't read the input file");

    let part_1 = from_lines(input.lines());
    IN_PART_2.store(true, std::sync::atomic::Ordering::Relaxed);
    let part_2 = from_lines(input.lines());

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
