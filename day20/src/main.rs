use day20::module_set::ModuleSet;

fn main() {
    let input = std::fs::read_to_string("./day20/input.txt")
        .expect("Couldn't read the input file");

    let mut modules = ModuleSet::parse(&input);

    modules.update_inputs();
    while modules.reduce_flip_flop_to_inverter() {
        modules.update_inputs();
    }
    while modules.reduce_flip_flop_to_flip_flop() {
        modules.update_inputs();
    }

    let part1 = {
        let mut modules = modules.clone();
        let mut total_lo = 0;
        let mut total_hi = 0;
    
        for i in 0_usize..1000 {
            let (lo, hi, rx_pulsed) = modules.pulse();
            total_lo += lo;
            total_hi += hi;
        }

        total_lo * total_hi
    };

    println!("Part 1: {part1}");

    
    let part2 = 'part2: {
        println!("Has loops: {}", modules.has_loops());
        println!("{} bits of state", modules.bits_of_state());
        println!("{} modules", modules.len());
    
        for i in 0_usize.. {
            if i % 1_000_000 == 0 {
                println!("{i}");
            }
            let (.., rx_pulsed) = modules.pulse();
            if rx_pulsed {
                break 'part2 i;
            }
        }

        unreachable!()
    };

    println!("Part 2: {part2}");
}
