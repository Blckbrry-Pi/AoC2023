from typing import Union

def get_file_lines(filename: str) -> list[str]:
    with open(filename) as file:
        return [line.rstrip() for line in file if line.strip() != ""]

def get_round(round: str) -> tuple[int, int, int]:
    (r, g, b) = (0, 0, 0)
    for part in round.strip().split(","):
        if part.strip().endswith("red"):
            r += int(part.strip()[:-3].strip())
        elif part.strip().endswith("green"):
            g += int(part.strip()[:-5].strip())
        elif part.strip().endswith("blue"):
            b += int(part.strip()[:-4].strip())
    return (r, g, b)

def process_game(line: str) -> tuple[int, list[tuple[int, int, int]]]:
    (game_num, rounds) = line.split(":")
    rounds = rounds.split(";")
    rounds = list(map(get_round, rounds))
    return (int(game_num.strip()[4:]), rounds)

def process_line_p1(line: str) -> Union[int, float]:
    id, rounds = process_game(line)
    for round in rounds:
        if round[0] > 12 or round[1] > 13 or round[2] > 14:
            return 0

    return id

def process_line_p2(line: str) -> Union[int, float]:
    r, g, b = 0, 0, 0

    id, rounds = process_game(line)
    for round in rounds:
        r = max(r, round[0])
        g = max(g, round[1])
        b = max(b, round[2])
    return r * g * b


processed_lines_p1 = map(process_line_p1, get_file_lines("./day02/input.txt"))
processed_lines_p2 = map(process_line_p2, get_file_lines("./day02/input.txt"))

print("Part 1:", sum(processed_lines_p1))
print("Part 2:", sum(processed_lines_p2))

