const WINNING_NUM_COUNT: usize = 10;
const GAME_NUM_COUNT: usize = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Game {
    num: u32,
    winning: [u32; WINNING_NUM_COUNT],
    game: [u32; GAME_NUM_COUNT],
}

impl Game {
    pub fn parse(line: &str) -> Self {
        let mut output = Game {
            num: 0,
            winning: [0; WINNING_NUM_COUNT],
            game: [0; GAME_NUM_COUNT],
        };

        let line = line.trim().as_bytes();
        let mut i = 5;

        while line[i] != b':' {
            if b'0' <= line[i] && line[i] <= b'9' {
                output.num *= 10;
                output.num += (line[i] - b'0') as u32;
            }
            i += 1;
        }

        i += 2;

        for j in 0..WINNING_NUM_COUNT {
            while line[i] == b' ' { i += 1; }
            while line[i] != b' ' && line[i] != b'|' {
                if b'0' <= line[i] && line[i] <= b'9' {
                    output.winning[j] *= 10;
                    output.winning[j] += (line[i] - b'0') as u32;
                }
                i += 1;
            }
            if line[i] == b'|' { break; }
        }

        i += 2;

        for j in 0..GAME_NUM_COUNT {
            while i < line.len() && line[i] == b' ' { i += 1; }
            while i < line.len() && line[i] != b' ' {
                if b'0' <= line[i] && line[i] <= b'9' {
                    output.game[j] *= 10;
                    output.game[j] += (line[i] - b'0') as u32;
                }
                i += 1;
            }
            if i >= line.len() { break; }
        }

        output
    }

    pub fn count_matches(self) -> u32 {
        let mut count = 0;

        for game_num in self.game {
            if game_num == 0 { continue; }

            for winning_num in self.winning {
                if game_num == winning_num {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    pub fn get_game_score(self) -> u32 {
        let matches = self.count_matches();

        if matches == 0 {
            0
        } else {
            1 << (matches - 1)
        }
    }
}



fn main() {
    let input = std::fs::read_to_string("./day4/input.txt")
        .expect("Couldn't read the input file");

    let games: Vec<_> = input.lines().map(|s| Game::parse(&s)).collect();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut totals = vec![1; games.len()];

    for game in games {
        part1 += game.get_game_score();
        for i in game.num..game.num + game.count_matches() {
            totals[i as usize] += totals[game.num as usize - 1];
        }
    }

    part2 = totals.into_iter().sum();
    // let mut symbols = vec![];
    // for line in &schematic.data {
    //     for token in line {
    //         let token = *token;
    //         if token.is_symbol() {
    //             symbols.push(token);
    //         }
    //     }
    // }

    // let mut part1 = 0;
    // let mut part2 = 0;

    // let mut used_numbers = std::collections::HashSet::new();

    // for symbol in symbols {
    //     let nums = symbol.surrounding_numbers(&schematic);
    //     for number in nums.iter().copied() {
    //         if !used_numbers.contains(&number) {
    //             used_numbers.insert(number);

    //             part1 += schematic.get(number.0, number.1).num_val(&schematic);
    //         }
    //     }
    //     if nums.len() == 2 && symbol.is_gear() {
    //         let num0 = schematic.get(nums[0].0, nums[0].1).num_val(&schematic);
    //         let num1 = schematic.get(nums[1].0, nums[1].1).num_val(&schematic);

    //         part2 += num0 * num1;
    //     }
    // }
    

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}