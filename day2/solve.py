from typing import Union

def get_file_lines(filename: str) -> list[str]:
    with open(filename) as file:
        return [line.rstrip() for line in file if line.strip() != ""]

def process_line_p1(line: str) -> Union[int, float]:
    # TODO: Wait for Day 2 to come out
    return -1

def process_line_p2(line: str) -> Union[int, float]:
    # TODO: Wait for Day 2 to come out
    return -1


processed_lines_p1 = map(process_line_p1, get_file_lines("./day2/input.txt"))
processed_lines_p2 = map(process_line_p2, get_file_lines("./day2/input.txt"))

print("Part 1:", sum(processed_lines_p1))
print("Part 2:", sum(processed_lines_p2))

