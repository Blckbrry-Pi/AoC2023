use day09::sequence::Sequence;

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("./day09/test.txt")
        .expect("Couldn't read the input file");

    // Parse 1 sequence per line of the input file
    let sequences: Vec<_> = input.lines().map(Sequence::parse).collect();

    // Get clones of the sequences extrapolated in their proper directions
    let extrapolated_forward: Vec<_> = sequences.iter().cloned().map(|seq| seq.extrapolate()).collect();
    let extrapolated_backward: Vec<_> = sequences.iter().cloned().map(|seq| seq.extrapolate_back()).collect();

    // Sum up the extrapolated numbers
    let part1 = extrapolated_forward.into_iter().map(|seq| seq.last().unwrap()).sum::<isize>();
    let part2 = extrapolated_backward.into_iter().map(|seq| seq.first().unwrap()).sum::<isize>();

    // Print the results
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
