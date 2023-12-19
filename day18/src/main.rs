use std::collections::HashSet;

use day18::{line::Line, map::Map, pos::Pos};

fn main() {
    let input = std::fs::read_to_string("./day18/test.txt")
        .expect("Couldn't read the input file");

    let p1_lines: Vec<_> = input.lines().map(Line::parse).collect();
    let p2_lines: Vec<_> = p1_lines.iter().copied().map(Line::to_part_2).collect();
    
    let mut p1_map = Map::new(Pos::new(0, 0));
    let mut p2_map = Map::new(Pos::new(0, 0));

    println!("Part 2 lines: {p2_lines:#?}");

    for line in p1_lines.clone() {
        p1_map.exec_line(line);
    }

    for line in p2_lines.clone() {
        p2_map.exec_line(line);
    }

    let external = p2_map.superchunked(8).get_external(HashSet::new(), None);
    println!("Tp1");
    println!("{:?}", Map::visualize(external.clone()));

    // let (border, double_border) = p2_map.superchunked(2048).get_superchunked_border(external, 2);
    // println!("Tp1.5");
    
    // let external = p2_map.superchunked(2048).get_external(double_border, Some(border));
    // println!("Tp2");
    // let (border, double_border) = p2_map.superchunked(1024).get_superchunked_border(external, 2);
    // println!("Tp2.5");

    // let external = p2_map.superchunked(1024).get_external(double_border, Some(border));
    // println!("Tp3");
    // let (border, double_border) = p2_map.superchunked(512).get_superchunked_border(external, 2);
    // println!("Tp3.5");

    // let external = p2_map.superchunked(512).get_external(double_border, Some(border));
    // println!("Tp4");
    // let (border, double_border) = p2_map.superchunked(256).get_superchunked_border(external, 2);
    // println!("Tp4.5");

    // let external = p2_map.superchunked(256).get_external(double_border, Some(border));
    // println!("Tp5");
    // let (border, double_border) = p2_map.superchunked(128).get_superchunked_border(external, 2);
    // println!("Tp5.5");

    // let external = p2_map.superchunked(128).get_external(double_border, Some(border));
    // println!("Tp6");
    // let (border, double_border) = p2_map.superchunked(4).get_superchunked_border(external, 2);
    // let external = p2_map.superchunked(4).get_external(double_border, Some(border));
    
    // let (border, double_border) = p2_map.superchunked(2).get_superchunked_border(external, 2);
    // let external = p2_map.superchunked(2).get_external(double_border, Some(border));

    // let (border, double_border) = p2_map.get_superchunked_border(external, 2);
    
    // // println!("{:?}", p2_map.superchunked(1));
    // println!("{:?}", Map::visualize(border.clone()));
    // println!("{:?}", Map::visualize(double_border.clone()));



    // // println!("Tp1");
    // let fine_grained_external = p2_map.get_external(double_border.clone(), Some(border.clone()));
    // println!("{:?}", Map::visualize(fine_grained_external.clone()));
    // // println!("Tp2");
    // let fine_grained_external = p2_map.cap_range_superchunks(fine_grained_external, 2);


    // println!("{:?}", Map::visualize(fine_grained_external.clone()));

    // println!("Tp3");
    // println!("b\n{:?}", Map::visualize(border));
    // println!();
    // println!("c\n{:?}", Map::visualize(double_border));
    // println!();
    // println!("d\n{:?}", Map::visualize(fine_grained_external.clone()));

    // println!("{}", p2_map.estimate_internal());
    // println!("{:?}", fine_grained_external.len());

    let part1 = p1_map.count_internal(); // TODO: Part 1
    // let part2 = p2_map.estimate_internal(2) - fine_grained_external.len(); // TODO: Part 2

    println!("Part 1: {part1}");
    // println!("Part 2: {part2}");
}
