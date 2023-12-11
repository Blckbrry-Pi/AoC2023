use day11::map::Map;

fn main() {
    let input = std::fs::read_to_string("./day11/input.txt")
        .expect("Couldn't read the input file");

    let map = Map::parse(&input.lines().collect::<Vec<_>>());

    let mut part1 = 0;
    let mut part2 = 0;


    let galaxies_p1 = map.clone().expanded_galaxies(2);
    let galaxies_p2 = map.clone().expanded_galaxies(1_000_000);

    for i in 0..galaxies_p1.len() {
        for j in i+1..galaxies_p1.len() {
            let x_diff = galaxies_p1[i].0.max(galaxies_p1[j].0) - galaxies_p1[i].0.min(galaxies_p1[j].0);
            let y_diff = galaxies_p1[i].1.max(galaxies_p1[j].1) - galaxies_p1[i].1.min(galaxies_p1[j].1);
            part1 += x_diff + y_diff;

            let x_diff = galaxies_p2[i].0.max(galaxies_p2[j].0) - galaxies_p2[i].0.min(galaxies_p2[j].0);
            let y_diff = galaxies_p2[i].1.max(galaxies_p2[j].1) - galaxies_p2[i].1.min(galaxies_p2[j].1);
            part2 += x_diff + y_diff;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
