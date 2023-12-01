fn get_val(line: &str) -> u32 {
    let mut first = None;
    let mut last = None;

    for chr in line.chars() {
        if ('0'..='9').contains(&chr) {
            let num = chr as u32 - '0' as u32;

            if first.is_none() {
                first = Some(num);
            }
            last = Some(num);
        }
    }

    first.unwrap() * 10 + last.unwrap()
}

fn part_2_conversion(line: &str) -> String {
    let mut output = line.to_string();

    for i in 0..line.len() - 2 {
        match &output[i..i+3] {
            "one" => output.replace_range(i..i+3, "1ne"),
            "two" => output.replace_range(i..i+3, "2wo"),
            "six" => output.replace_range(i..i+3, "6ix"),

            "fou" => if output.len() - i >= 4 && &output[i+3..i+4] == "r" {
                output.replace_range(i..i+4, "4our");
            },
            "fiv" => if output.len() - i >= 4 && &output[i+3..i+4] == "e" {
                output.replace_range(i..i+4, "5ive");
            },
            "nin" => if output.len() - i >= 4 && &output[i+3..i+4] == "e" {
                output.replace_range(i..i+4, "9ine");
            },

            "thr" => if output.len() - i >= 5 && &output[i+3..i+5] == "ee" {
                output.replace_range(i..i+5, "3hree");
            },
            "sev" => if output.len() - i >= 5 && &output[i+3..i+5] == "en" {
                output.replace_range(i..i+5, "7even");
            },
            "eig" => if output.len() - i >= 5 && &output[i+3..i+5] == "ht" {
                output.replace_range(i..i+5, "8ight");
            },
            
            _ => ()
        }
    }
    
    output
}

fn main() {
    let input = std::fs::read_to_string("./day1/input.txt")
        .expect("Couldn't read the input file");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        part1 += get_val(line);
        part2 += get_val(&part_2_conversion(line));
    }
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}