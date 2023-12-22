use day18::chunk_set::ChunkSet;
use day18::line::Line;
use day18::map::Map;
use day18::pos::Pos;

fn main() {
    let input = std::fs::read_to_string("./day18/input.txt")
        .expect("Couldn't read the input file");

    let p1_lines: Vec<_> = input.lines().map(Line::parse).collect();
    let p2_lines: Vec<_> = p1_lines.iter().copied().map(Line::to_part_2).collect();
    
    let mut p1_map = Map::new(Pos::new(0, 0));
    let mut p2_map = Map::new(Pos::new(0, 0));

    for line in p1_lines.clone() {
        p1_map.exec_line(line);
    }

    for line in p2_lines.clone() {
        println!("Building Map... ({}/{})", p2_lines.iter().position(|l| l == &line).unwrap() + 1, p2_lines.len());
        p2_map.exec_line(line);
    }

    let offset = p2_map.range().0;
    let offset = Pos::new(-offset.x, -offset.y);
    p2_map = p2_map.offset(offset);
    // println!("Map bounds ({:?})", p2_map.range());
    
    println!("Partitioning map...");
    let clear_set = ChunkSet::full_from_map(&p2_map, 18);
    
    println!("Getting trench area...");
    let nonclear_area = clear_set.nonclear_area();

    println!("Getting area inside trench...");
    let internal_area = clear_set.internal_area(&p2_map);
    
    
    let part1 = p1_map.count_internal(); // TODO: Part 1
    let part2 = nonclear_area + internal_area; // TODO: Part 2

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
