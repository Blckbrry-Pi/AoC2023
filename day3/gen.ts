const WIDTH = 2500;
const HEIGHT = 2500;

const NUMBERS = Math.floor(WIDTH * HEIGHT / 20);
const SYMBOLS = Math.floor(NUMBERS / 5 * 4);

const output = Array(HEIGHT).fill(null).map(() => Array(WIDTH).fill("."));

for (let i = 0; i < NUMBERS; i++) {
    while (true) {
        const x = Math.floor(Math.random() * WIDTH);
        const y = Math.floor(Math.random() * HEIGHT);
        const value = Math.floor(Math.random() * 1000);

        const digits = value.toString().split("");
        const numDig = digits.length;

        if (x - numDig + 1 < 0) continue;
        if (output[y].slice(x - numDig, x + 2).some((v) => v !== ".")) continue;

        for (let j = x - numDig + 1; j <= x; j++) {
            output[y][j] = digits.shift();
        }
        break;
    } 
}


const SYMBOL_DICT = "!#$*+:=;";

for (let i = 0; i < SYMBOLS; i++) {
    while (true) {
        const x = Math.floor(Math.random() * WIDTH);
        const y = Math.floor(Math.random() * HEIGHT);
        if (output[y][x] !== ".") continue;
        
        output[y][x] = SYMBOL_DICT[Math.floor(Math.random() * SYMBOL_DICT.length)];
        break;
    } 
}


console.log(output.map((row) => row.join("")).join("\n"));

