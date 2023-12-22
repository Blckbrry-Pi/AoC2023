use std::collections::HashMap;

// use day20::module_set::ModuleSet;
use day20::ModuleSet;


fn lcm(a: usize, b: usize) -> usize { a * b / gcd(a, b) }
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { return a; }
    gcd(b, a % b)
}

fn main() {
    let input = std::fs::read_to_string("./day20/input.txt")
        .expect("Couldn't read the input file");
    let mut modules = ModuleSet::parse(&input);

    // Optimization!
    modules.optimize();
    
    // Run pulse 1000 times
    let part1 = {
        let mut modules = modules.clone();
        let mut total_lo = 0;
        let mut total_hi = 0;
    
        for _ in 0_usize..1000 {
            let (lo, hi, _, _) = modules.pulse("");
            total_lo += lo;
            total_hi += hi;
        }

        total_lo * total_hi
    };
    println!("Part 1: {part1}");



    // Required assumption 1: There are subsets that have no crosstalk
    let subsets: Vec<_> = modules.subsets().collect();
    let mut loops = vec![];

    for subset in &subsets {
        let mut curr_subset = subset.clone();
        let mut seen = HashMap::new();

        for i in 0.. {
            let _ = curr_subset.pulse("lx");
            if let Some(prev) = seen.insert(curr_subset.clone(), i) {
                loops.push((prev, i - prev));
                break;
            }
        }
    }
    // (Technically not required) assumption 2:
    // Every time a subset loops, it sends out a high pulse on its last
    // iteration before looping
    let index = loops.into_iter().fold(1, |a, b| lcm(a, b.1));


    let part2 = index;
    println!("Part 2: {part2}");
}
