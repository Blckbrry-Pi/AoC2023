from typing import Union
import re

def get_file_lines(filename: str) -> list[str]:
    with open(filename) as file:
        return [line.rstrip() for line in file if line.strip() != ""]

def get_sub(match: str) -> str:
    return {
        "one": "1ne",
        "two": "2wo",
        "three": "3hree",
        "four": "4our",
        "five": "5ive",
        "six": "6ix",
        "seven": "7even",
        "eight": "8ight",
        "nine": "9ine",
    }[match]

def word_replacer(line: str) -> str:
    line = line
    while True:
        line, count = re.subn(
            r"one|two|three|four|five|six|seven|eight|nine",
            lambda m: get_sub(m.group(0)),
            line,
        )
        if count == 0:
            break
    return line

def get_digits(line: str, is_day_2: bool) -> list[int]:
    if is_day_2:
        line = word_replacer(line)
    return [int(digit) for digit in line if digit.isdigit()]

def get_num_from_digits(digits: list[int]) -> int:
    return digits[0] * 10 + digits[-1]


def process_line_day_1(line: str) -> Union[int, float]:
    digits = get_digits(line, False)
    return get_num_from_digits(digits)

def process_line_day_2(line: str) -> Union[int, float]:
    digits = get_digits(line, True)
    return get_num_from_digits(digits)


processed_lines_day_1 = map(process_line_day_1, get_file_lines("./day1/input.txt"))
processed_lines_day_2 = map(process_line_day_2, get_file_lines("./day1/input.txt"))

print(sum(processed_lines_day_1), sum(processed_lines_day_2))
