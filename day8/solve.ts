import { readFileSync } from "node:fs";

function gcd(a: number, b: number): number {
    if (b === 0) return a;
    return gcd(b, a % b);
}

function lcm(a: number, b: number): number {
    return a * b / gcd(a, b);
}

type IdentChar = 'A'|'B'|'C'|'D'|'E'|'F'|'G'|'H'|'I'|'J'|'K'|'M'|'N'|'O'|'P'|'Q'|'R'|'S'|'T'|'V'|'W'|'X'|'Y'|'Z'|'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9';
type Ident = `${IdentChar}${IdentChar}${IdentChar}`;

type Direction = "L"|"R";

class Directions {
    public directions: Direction[];

    constructor() {
        this.directions = [];
    }

    public add(direction: Direction) {
        this.directions.push(direction);
    }

    public *infinite(start: number = 0) {
        while (true) {
            for (let i = start; i < this.directions.length; i++) {
                yield this.directions[i];
            }
        }
    }

    public *infiniteIndexed(start: number = 0) {
        while (true) {
            for (let i = start; i < this.directions.length; i++) {
                yield [i, this.directions[i]] as [number, Direction];
            }
        }
    }

    public static parse(line: string): Directions {
        const directions = new Directions();
        for (let i = 0; i < line.length; i++) {
            directions.add(line[i] as Direction);
        }
        return directions;
    }
}

class State {
    public dirIdx: number;
    public ident: Ident;

    constructor(dirIdx: number, ident: Ident) {
        this.dirIdx = dirIdx;
        this.ident = ident;
    }
}

class Cycle {
    public offset: number;
    public len: number;

    constructor(offset: number, len: number) {
        this.offset = offset;
        this.len = len;
    }
}

class Branch {
    public l: Ident;
    public r: Ident;

    constructor(l: Ident, r: Ident) {
        this.l = l;
        this.r = r;
    }

    public get(d: Direction) {
        return d === "L" ? this.l : this.r;
    }
}

class DesertMap {
    public start: Ident;
    public end: Ident;
    public branches: Map<Ident, Branch>;

    constructor(start: Ident = "AAA", end: Ident = "ZZZ") {
        this.start = start;
        this.end = end;
        this.branches = new Map();
    }

    public static parse(lines: string[], start: Ident = "AAA", end: Ident = "ZZZ"): DesertMap {
        const map = new DesertMap(start, end);

        const lineRegex = /(...) \= \((...), (...)\)/;
        for (const line of lines) {
            const match = lineRegex.exec(line);
            if (!match) throw new Error(`Invalid line: ${line}`);

            const [, ident, l, r] = match;

            const branch = new Branch(l as Ident, r as Ident);
            map.branches.set(ident as Ident, branch);
        }

        return map;
    }

    public *path(directions: Directions, start: Ident = this.start) {
        let curr = start;

        for (const direction of directions.infinite()) {
            yield curr;
            curr = this.branches.get(curr)!.get(direction);
        }
    }
}

const values = readFileSync("./day8/input.txt", "utf-8").split("\n");

const directions = Directions.parse(values[0]);
const map = DesertMap.parse(values.slice(2));



// console.log({ directions, map });

let part1 = 0;
for (const ident of map.path(directions)) {
    if (ident === map.end) break;
    part1++;
}



const currents = [...map.branches.keys()].filter(ident => ident.endsWith("A"));

const zLogs: (number | null)[] = currents.map(() => null);
const cycles: (null | Cycle)[] = currents.map(() => null);

let j = 0;
for (const direction of directions.infinite()) {
    for (let currIdx = 0; currIdx < currents.length; currIdx++) {
        if (currents[currIdx].endsWith("Z")) {
            if (zLogs[currIdx] !== null) {
                cycles[currIdx] = new Cycle(j, j - zLogs[currIdx]!);
            } else if (!cycles[currIdx]) {
                zLogs[currIdx] = j;
            }
        }
    }

    for (let currIdx = 0; currIdx < currents.length; currIdx++) {
        currents[currIdx] = map.branches.get(currents[currIdx])!.get(direction);
    }
    
    j++;

    if (cycles.every(cycle => cycle !== null)) {
        break;
    }
}

let curr_offset = cycles[0]!.offset;
let curr_lcm = cycles[0]!.len;

for (let j = 1; j < cycles.length; j++) {
    const cycle = cycles[j]!;
    while (curr_offset % cycle.len !== cycle.offset % cycle.len) curr_offset += curr_lcm;
    curr_lcm = lcm(curr_lcm, cycle.len);
}

const part2 = curr_offset;

console.log("Part 1:", part1);
console.log("Part 2:", part2);
