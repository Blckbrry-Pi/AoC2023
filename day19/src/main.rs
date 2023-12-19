use day19::{instructions::Instructions, attribs::{Attribs, AttribRange}};

fn main() {
    let input = std::fs::read_to_string("./day19/input.txt")
        .expect("Couldn't read the input file");

    let (instructions, items) = input.split_once("\n\n").unwrap();

    let workflows = Instructions::parse(instructions);
    let items: Vec<_> = items.lines().map(Attribs::parse).collect();

    let mut part1 = 0;

    for item in items {
        if workflows.run(item) {
            part1 += item.value();
        }
    }

    let part2 = workflows.run_range(AttribRange::new((1, 4001), (1, 4001), (1, 4001), (1, 4001)));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
