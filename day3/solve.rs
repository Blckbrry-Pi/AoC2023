#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SymbolType {
    Gear,
    Other(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenType {
    Dot,
    Number(u8),
    Symbol(SymbolType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Token(TokenType, (usize, usize));

impl Token {
    fn is_num(&self) -> bool {
        matches!(self.0, TokenType::Number(_))
    }
    fn as_num(&self) -> Option<u8> {
        let TokenType::Number(num) = self.0 else { return None };
        Some(num)
    }

    fn is_gear(&self) -> bool {
        matches!(self.0, TokenType::Symbol(SymbolType::Gear))
    }
    fn is_symbol(&self) -> bool {
        matches!(self.0, TokenType::Symbol(_))
    }
    

    fn num_id(&self, schematic: &Schematic) -> (usize, usize) {
        let mut pos = self.1;
        while pos.0 != 0 && schematic.get(pos.0 - 1, pos.1).is_num() {
            pos.0 -= 1;
        }

        pos
    }

    fn num_val(&self, schematic: &Schematic) -> u32 {
        let mut val = 0;
        let mut pos = self.1;

        while pos.0 != 0 && schematic.get(pos.0 - 1, pos.1).is_num() {
            pos.0 -= 1;
        }

        while let Some(num) = schematic.get(pos.0, pos.1).as_num() {
            val *= 10;
            val += num as u32;

            pos.0 += 1;
            if pos.0 >= schematic.width {
                pos.0 = 0;
                pos.1 += 1;
            }
        }

        val
    }

    fn surrounding_numbers(&self, schematic: &Schematic) -> Vec<(usize, usize)> {
        let mut nums = std::collections::HashSet::new();

        for x in (self.1.0 - 1)..=(self.1.0 + 1) {
            for y in (self.1.1 - 1)..=(self.1.1 + 1) {
                if schematic.get(x, y).is_num() {
                    nums.insert(schematic.get(x, y).num_id(schematic));
                }
            }
        }

        nums.into_iter().collect()
    }
}

impl From<(char, (usize, usize))> for Token {
    fn from((c, (x, y)): (char, (usize, usize))) -> Self {
        let token_type = match c {
            '.' => TokenType::Dot,
            '0'..='9' => TokenType::Number(c as u8 - '0' as u8),
            '*' => TokenType::Symbol(SymbolType::Gear),
            _ => TokenType::Symbol(SymbolType::Other(c)),
        };
        Token(token_type, (x, y))
    }
}

struct Schematic {
    width: usize,
    data: Vec<Vec<Token>>
}

impl Schematic {
    fn new(data: Vec<Vec<char>>) -> Self {
        let data: Vec<Vec<Token>> = data
            .into_iter()
            .enumerate()
            .map(|(y, line)| line
                .into_iter()
                .enumerate()
                .map(|(x, chr)| Token::from((chr, (x, y))))
                .collect()
            )
            .collect();
        Schematic {
            width: data[0].len(),
            data,
        }
    }

    fn get(&self, x: usize, y: usize) -> &Token {
        &self.data[y][x]
    }
}

fn main() {
    let input = std::fs::read_to_string("./day3/input.txt")
        .expect("Couldn't read the input file");

    let schematic = Schematic::new(input.lines().map(|s| s.chars().collect()).collect());

    let mut symbols = vec![];
    for line in &schematic.data {
        for token in line {
            let token = *token;
            if token.is_symbol() {
                symbols.push(token);
            }
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;

    let mut used_numbers = std::collections::HashSet::new();

    for symbol in symbols {
        let nums = symbol.surrounding_numbers(&schematic);
        for number in nums.iter().copied() {
            if !used_numbers.contains(&number) {
                used_numbers.insert(number);

                part1 += schematic.get(number.0, number.1).num_val(&schematic);
            }
        }
        if nums.len() == 2 && symbol.is_gear() {
            let num0 = schematic.get(nums[0].0, nums[0].1).num_val(&schematic);
            let num1 = schematic.get(nums[1].0, nums[1].1).num_val(&schematic);

            part2 += num0 * num1;
        }
    }
    

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}