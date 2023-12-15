use day12::spring_row::Row;

fn main() {
    println!("{:?}", Row::all_possible_groups(&[3, 1], "###..#".len()).collect::<Vec<_>>());


    let input = std::fs::read_to_string("./day12/test.txt")
        .expect("Couldn't read the input file");

    let rows = input
        .lines()
        .map(Row::parse)
        .collect::<Vec<_>>();

    let possibilities = rows
        .clone()
        .into_iter()
        .flat_map(|mut r| r.valid_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // let possibilities_p2: usize = rows
    //     .into_iter()
    //     .map(|r| r.expand())
    //     .map(|mut r| r.valid_iter().count())
    //     .sum();

    let possibilities_p2: usize = rows
        .into_iter()
        .map(|r| r.expand())
        .map(|r| Row::all_possible_groups(&r.groups, r.tiles.len()).count())
        .inspect(|a| println!("{a}"))
        .sum();



    let part1 = possibilities.len();
    let part2 = possibilities_p2; // TODO: Part 2

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");


}
