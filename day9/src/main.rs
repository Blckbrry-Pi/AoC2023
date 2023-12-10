use day9::sequence::Sequence;

fn main() {
    let input = std::fs::read_to_string("./day9/input.txt")
        .expect("Couldn't read the input file");

    let sequences: Vec<_> = input.lines().map(Sequence::parse).collect();

    let extrapolated_forward: Vec<_> = sequences.iter().cloned().map(|seq| seq.extrapolate()).collect();
    let extrapolated_backward: Vec<_> = sequences.iter().cloned().map(|seq| seq.extrapolate_back()).collect();

    let part1 = extrapolated_forward.into_iter().map(|seq| seq.last().unwrap()).sum::<isize>();
    let part2 = extrapolated_backward.into_iter().map(|seq| seq.first().unwrap()).sum::<isize>();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
