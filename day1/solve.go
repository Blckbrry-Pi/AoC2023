package main
import (
    "fmt"
    "os"
	"strconv"
	"strings"
)

func doReplacement(input string) string {
	if (len(input) < 3) {
		return input
	}

	switch input[0] {
	case 'o':
		if (input[1] == 'n' && input[2] == 'e') {
			return strconv.Itoa(1) + doReplacement(input[1:])
		}
		return input
		
	case 't':
		if (input[1] == 'w' && input[2] == 'o') {
			return strconv.Itoa(2) + doReplacement(input[1:])
		}
		if (len(input) >= 5 && input[1] == 'h' && input[2] == 'r' && input[3] == 'e' && input[4] == 'e') {
			return strconv.Itoa(3) + doReplacement(input[1:])
		}
		return input
	
	case 'f':
		if (len(input) >= 4 && input[1] == 'o' && input[2] == 'u' && input[3] == 'r') {
			return strconv.Itoa(4) + doReplacement(input[1:])
		}
		if (len(input) >= 4 && input[1] == 'i' && input[2] == 'v' && input[3] == 'e') {
			return strconv.Itoa(5) + doReplacement(input[1:])
		}
		return input

	case 's':
		if (input[1] == 'i' && input[2] == 'x') {
			return strconv.Itoa(6) + doReplacement(input[1:])
		}
		if (len(input) >= 5 && input[1] == 'e' && input[2] == 'v' && input[3] == 'e' && input[4] == 'n') {
			return strconv.Itoa(7) + doReplacement(input[1:])
		}
		return input

	case 'e':
		if (len(input) >= 5 &&  input[1] == 'i' && input[2] == 'g' && input[3] == 'h' && input[4] == 't') {
			return strconv.Itoa(8) + doReplacement(input[1:])
		}
		return input

	case 'n':
		if (len(input) >= 4 && input[1] == 'i' && input[2] == 'n' && input[3] == 'e') {
			return strconv.Itoa(9) + doReplacement(input[1:])
		}
		return input
	}
	return input;
}

func toPart2(input string) string {
	var running string = input;
	for i := 0; i < len(input); i++ {
		running = running[:i] + doReplacement(running[i:]);
	}
	return running;
}

func value(input string) int {
	var first int = -1;
	var last int = -1;
	for i := 0; i < len(input); i++ {
		if '0' <= input[i] && input[i] <= '9' {
			if (first == -1) {
				first = int(input[i] - '0');
			}
			last = int(input[i] - '0');
		}
	}
	return first * 10 + last;
}

func main() {
	data, err := os.ReadFile("day1/input.txt")
	if err != nil {
		panic(err)
	}

	var input string = string(data[:])

	var part1 int = 0;
	var part2 int = 0;

	for _, line := range strings.Split(input, "\n") {
		part1 += value(line);
		part2 += value(toPart2(line));
	}

	fmt.Printf("Part 1: %d\n", part1);
	fmt.Printf("Part 2: %d\n", part2);
}