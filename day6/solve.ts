import { readFileSync } from "node:fs";

type Pair = { time: number, distance: number };

const pairValue = ({ time, distance }: Pair) => {
    let ways = 0;
    for (let i = 0; i <= time; i++) if (i * (time - i) > distance) ways++;
    return ways;
};
const splitLine = (line: string) => {
    return line.split(" ").slice(1).filter(Boolean);
};

const getPairs = (time: string, distance: string) => {
    return splitLine(time).map((time, i) => [time, splitLine(distance)[i]] as const);
};

const part1Value = (time: string, distance: string) => {
    const pairs = getPairs(time, distance);
    const values = pairs.map(([time, distance]) => pairValue({ time: Number(time), distance: Number(distance) }));
    return values.reduce((a, b) => a * b, 1);
};

const part2Value = (time: string, distance: string) => {
    const raceTime = Number(splitLine(time).join(""));
    const dist = Number(splitLine(distance).join(""));
    return pairValue({ time: raceTime, distance: dist });
};

const values = readFileSync("./day6/input.txt", "utf-8").split("\n");

const part1 = part1Value(values[0], values[1]);
const part2 = part2Value(values[0], values[1]);

console.log("Part 1:", part1);
console.log("Part 2:", part2);
