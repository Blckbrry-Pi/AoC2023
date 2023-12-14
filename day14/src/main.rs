use day14::platform::Platform;

fn main() {
    let input = std::fs::read_to_string("./day14/input.txt")
        .expect("Couldn't read the input file");

    let platform = Platform::parse(&input);

    let mut shifted_platform = platform.clone();
    shifted_platform.shift_up();
    let part1 = shifted_platform.calculate_load_north_beam();


    let mut spun_platform = platform.clone();
    let mut seen = std::collections::HashMap::new();
    for i in 0..1_000_000_000 {
        if let Some(prev_idx) = seen.insert(spun_platform.clone(), i) {
            let cycle_len = i - prev_idx;
            let remaining = (1_000_000_000 - i) % cycle_len;
            for _ in 0..remaining {
                spun_platform.spin();
            }
            break;
        }
        spun_platform.spin();
    }

    let part2 = spun_platform.calculate_load_north_beam();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
