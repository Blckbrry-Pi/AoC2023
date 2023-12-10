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
    let input = std::fs::read_to_string("./day04/input.txt")
        .expect("Couldn't read the input file");

    let games: Vec<_> = input.lines().map(|s| Game::parse(&s)).collect();

    let mut part1: u32 = 0;

    let mut totals = vec![1; games.len()];

    for game in games {
        part1 += game.get_game_score();
        for i in game.num..game.num + game.count_matches() {
            totals[i as usize] += totals[game.num as usize - 1];
        }
    }

    let part2: u32 = totals.into_iter().sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}