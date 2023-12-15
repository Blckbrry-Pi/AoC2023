use day12::{spring_row::Row, group_manager::GroupManager};

fn main() {
    let input = std::fs::read_to_string("./day12/input.txt")
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

    let managers = rows.iter().map(Row::expand).map(|row| (GroupManager::from_row(&row), row)).collect::<Vec<_>>();
    let possibilities_p2: usize = managers.iter()
        .map(|(manager, row)| manager.possibility_count(row))
        .enumerate()
        .inspect(|(idx, possibs)| println!("{idx:4}: {possibs}"))
        .map(|(_, val)| val)
        .sum();
    

    let part1 = possibilities.len();
    let part2 = possibilities_p2;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
