use day23::graph::Graph;

fn main() {
    let input = std::fs::read_to_string("./day23/input.txt")
        .expect("Couldn't read the input file");

    let mut p1_graph = Graph::parse(&input);
    p1_graph.reduce();

    let p1_paths = p1_graph.paths();
    let p1_longest_path = p1_paths.into_iter().max_by_key(|p| p.len(&p1_graph)).unwrap();

    let part1 = p1_longest_path.len(&p1_graph);
    println!("Part 1: {part1}");


    let mut p2_graph = Graph::parse_p2(&input);
    p2_graph.reduce();

    let p2_paths = p2_graph.paths();
    let p2_longest_path = p2_paths.into_iter().max_by_key(|p| p.len(&p2_graph)).unwrap();

    let part2 = p2_longest_path.len(&p2_graph);
    println!("Part 2: {part2}");
}
