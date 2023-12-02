#include <stdio.h>

typedef struct {
    int red;
    int green;
    int blue;
} Round;

typedef struct {
    int id;
    int round_count;
    Round rounds[20];
} Game;

Round parseRound(char *line, int line_len) {
    Round round;
    round.red = 0;
    round.green = 0;
    round.blue = 0;

    for (int i = 0; i < line_len; i++) {
        if (line[i] == ' ' || line[i] == '\n' || line[i] == '\t') continue;

        int value = 0;
        int done = 0;
        while (1) {
            switch (line[i]) {
                case '0': value = value * 10 + 0; break;
                case '1': value = value * 10 + 1; break;
                case '2': value = value * 10 + 2; break;
                case '3': value = value * 10 + 3; break;
                case '4': value = value * 10 + 4; break;
                case '5': value = value * 10 + 5; break;
                case '6': value = value * 10 + 6; break;
                case '7': value = value * 10 + 7; break;
                case '8': value = value * 10 + 8; break;
                case '9': value = value * 10 + 9; break;
                default: done = 1; break;
            }
            i++;
            if (done) break;
        }

        switch (line[i]) {
            case 'r':
                round.red += value;
                i += 4;
                break;

            case 'g':
                round.green += value;
                i += 6;
                break;

            case 'b':
                round.blue += value;
                i += 5;
                break;

            default:
                break;
        }
        
    }

    return round;
}

Game parseGame(char *line) {
    Game game;
    game.id = 0;
    game.round_count = 0;

    int i = 0;
    int value = 0;
    while (line[i] != ':') {
        switch (line[i]) {
            case '0': value = value * 10 + 0; break;
            case '1': value = value * 10 + 1; break;
            case '2': value = value * 10 + 2; break;
            case '3': value = value * 10 + 3; break;
            case '4': value = value * 10 + 4; break;
            case '5': value = value * 10 + 5; break;
            case '6': value = value * 10 + 6; break;
            case '7': value = value * 10 + 7; break;
            case '8': value = value * 10 + 8; break;
            case '9': value = value * 10 + 9; break;
            default: break;
        }
        i++;
    }
    game.id = value;
    i += 2;

    int curr_idx = i;
    int next_idx = i+1;
    while (1) {
        while (line[next_idx] != '\0' && line[next_idx] != ';') {
            next_idx++;
        }

        game.rounds[game.round_count] = parseRound(line + sizeof(char) * curr_idx, next_idx - curr_idx);
        game.round_count++;

        if (line[next_idx] == '\0') break;

        curr_idx = next_idx + 2;
        next_idx = curr_idx + 1;
    }

    return game;
}

Round get_max_round(Game game) {
    int round_r, round_g, round_b;
    int r = 0, g = 0, b = 0;
    
    for (int i = 0; i < game.round_count; i++) {
        round_r = game.rounds[i].red;
        round_g = game.rounds[i].green;
        round_b = game.rounds[i].blue;

        if (round_r > r) r = round_r;
        if (round_g > g) g = round_g;
        if (round_b > b) b = round_b;
    }
    return (Round) {r, g, b};
}

int get_value_p1(Game game) {
    Round max_round = get_max_round(game);

    if (max_round.red <= 12 && max_round.green <= 13 && max_round.blue <= 14) {
        return game.id;
    } else {
        return 0;
    }
}

int get_value_p2(Game game) {
    Round max_round = get_max_round(game);

    return max_round.red * max_round.green * max_round.blue;
}

int main() {
    FILE *fptr;
    char myString[1000];

    fptr = fopen("day2/input.txt", "r");

    int part1 = 0;
    int part2 = 0;

    while (fgets(myString, 1000, fptr)) {
        part1 += get_value_p1(parseGame(myString));
        part2 += get_value_p2(parseGame(myString));
    }

    // Close the file
    fclose(fptr);

    printf("Part 1: %d\n", part1);
    printf("Part 2: %d\n", part2);

    return 0;
}