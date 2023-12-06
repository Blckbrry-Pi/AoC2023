use day5::{Mapper, range::IdRange};



fn main() {
    let input = std::fs::read_to_string("./day5/input.txt")
        .expect("Couldn't read the input file");

    let mapper = Mapper::parse(&input.lines().collect::<Vec<_>>()[2..]);

    let seeds: Vec<usize> = input.lines().next().unwrap()[7..].split(' ').map(|s| s.parse().unwrap()).collect::<Vec<_>>();

    // Part 1    
    let locations: Vec<usize> = seeds.iter().copied().map(|seed| mapper.get_val(seed)).collect();

    // Part 2
    let seed_ranges: Vec<IdRange> = seeds.iter().copied()
        .zip(seeds.iter().copied().skip(1))
        .step_by(2)
        .map(|(range_start, range_len)| IdRange::new_start_len(range_start, range_len))
        .collect();

    let mut possible_locations = mapper.possibilities(&seed_ranges);
    possible_locations.sort();
    
    println!("Part 1: {}", locations.into_iter().min().unwrap());
    println!("Part 2: {}", possible_locations[0].min);
}

// Done with part 1 at 2:32pm
// Done with part 2 at 3:40pm