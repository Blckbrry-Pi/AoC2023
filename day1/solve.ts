import { readFileSync } from "node:fs";

const sum = (...values: number[]) => values.reduce((a, b) => a + b, 0);
const digits = (isPart2: boolean) => {
    if (isPart2) {
        return /on(?=e)|tw(?=o)|thre(?=e)|four|fiv(?=e)|six|seve(?=n)|eigh(?=t)|nin(?=e)|\d/g;
    } else {
        return /\d/g;
    }
};

const getDigitsInLine = (line: string, isPart2: boolean) => {
    const regex = digits(isPart2);
    const strToNum: Record<string, number> = {
        "0": 0,
        "1": 1,
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,

        "on": 1,
        "tw": 2,
        "thre": 3,
        "four": 4,
        "fiv": 5,
        "six": 6,
        "seve": 7,
        "eigh": 8,
        "nin": 9,
    };
    const result = [...line.matchAll(regex)];
    const lineDigits = result.map(digit => strToNum[digit[0]]);
    return lineDigits;
};

const lineValue = (line: string, isPart2: boolean) => {
    const digits = getDigitsInLine(line, isPart2);
    return 10 * digits[0] + digits[digits.length - 1];
}

const values = readFileSync("./day1/input.txt", "utf-8").split("\n");

const part1 = sum(...values.map(line => lineValue(line, false)));
const part2 = sum(...values.map(line => lineValue(line, true)));

console.log({ part1, part2 });
