use day22::{brick::Brick, brick_layout::BrickLayout};


fn main() {
    let input = std::fs::read_to_string("./day22/input.txt")
        .expect("Couldn't read the input file");

    let layout = BrickLayout::parse(&input);
    let reliance_table = layout.build_reliance_table();
    let disintegratable: Vec<_> = BrickLayout::disintegratable(&reliance_table).into_iter().collect();
    let sole_reliances = layout.sole_reliances();

    let sole_reliance_count: usize = sole_reliances.into_values().map(|set| set.len()).sum();

    println!("{reliance_table:#?}");
    // println!("{disintegratable:?}");
    // println!("{sole_reliances:?}");

    // println!("{layout:?}");
    // println!("{layout:#?}");

    let part1 = disintegratable.len(); // TODO: Part 1
    let part2 = sole_reliance_count; // TODO: Part 2

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
