#include <stdio.h>

typedef enum {
    NONE,

    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
} NumberName;

NumberName getNumberName(char *string) {
    if (string[0] == '\0' || string[1] == '\0' || string[2] == '\0') {
        return NONE;
    }

    switch (string[0]) {
        case 'o': // one
            if (string[1] == 'n' && string[2] == 'e') return _1;
            return NONE;

        case 't': // two, three
            if (string[1] == 'w' && string[2] == 'o') return _2;
            if (string[1] == 'h' && string[2] == 'r' && string[3] == 'e' && string[4] == 'e') return _3;
            return NONE;

        case 'f': // four, five
            if (string[1] == 'o' && string[2] == 'u' && string[3] == 'r') return _4;
            if (string[1] == 'i' && string[2] == 'v' && string[3] == 'e') return _5;
            return NONE;
            
        case 's': // six, seven
            if (string[1] == 'i' && string[2] == 'x') return _6;
            if (string[1] == 'e' && string[2] == 'v' && string[3] == 'e' && string[4] == 'n') return _7;
            return NONE;

        case 'e': // eight
            if (string[1] == 'i' && string[2] == 'g' && string[3] == 'h' && string[4] == 't') return _8;
            return NONE;

        case 'n': // nine
            if (string[1] == 'i' && string[2] == 'n' && string[3] == 'e') return _9;
            return NONE;

        default:
            return NONE;
    }
}

char *transform_to_p2(char *origStr) {
    for (int i = 0; origStr[i] != '\0'; i++) {
        NumberName number = getNumberName(origStr + sizeof(char) * i);
        switch (number) {
            case _1:
                origStr[i] = '1';
                break;
            case _2:
                origStr[i] = '2';
                break;
            case _3:
                origStr[i] = '3';
                break;
            case _4:
                origStr[i] = '4';
                break;
            case _5:
                origStr[i] = '5';
                break;
            case _6:
                origStr[i] = '6';
                break;
            case _7:
                origStr[i] = '7';
                break;
            case _8:
                origStr[i] = '8';
                break;
            case _9:
                origStr[i] = '9';
                break;
            default:
                break;
        }
    }
    return origStr;
}

int get_value(char *line) {
    int first = -1;
    int last = -1;
    
    for (int i = 0; line[i] != '\0'; i++) {
        if ('0' <= line[i] && line[i] <= '9') {

            if (first == -1) first = line[i] - '0';
            last = line[i] - '0';
        }
    }
    return first * 10 + last;
}

int main() {
    FILE *fptr;
    char myString[100];

    fptr = fopen("day01/input.txt", "r");

    int part1 = 0;
    int part2 = 0;

    while (fgets(myString, 100, fptr)) {
        part1 += get_value(myString);
        part2 += get_value(transform_to_p2(myString));
    }

    // Close the file
    fclose(fptr);

    printf("Part 1: %d\n", part1);
    printf("Part 2: %d\n", part2);

    return 0;
}