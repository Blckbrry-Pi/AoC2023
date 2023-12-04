#include <stdio.h>

#define MAX_CARD_COUNT 200
#define WINNING_NUMS 10
#define CARD_NUMS 25

typedef struct {
    int card_num;
    int winning[WINNING_NUMS];
    int have[CARD_NUMS];
} Card;

Card parse_line(char *line) {
    Card card;

    int i = 5;
    int card_num = 0;
    card.card_num = 0;

    while (line[i] != ':') {
        if ('0' <= line[i] && line[i] <= '9') {
            card.card_num *= 10;
            card.card_num += line[i] - '0';
        }
        i++;
    }

    i += 2;

    for (card_num = 0; card_num < WINNING_NUMS; card_num++) {
        while (line[i] == ' ') i++;
        if (line[i] == '|') break;
        card.winning[card_num] = 0;

        while (line[i] != ' ') {
            card.winning[card_num] = card.winning[card_num] * WINNING_NUMS + line[i] - '0';
            i++;
        }
    }
    for (; card_num < WINNING_NUMS; card_num++) card.winning[card_num] = 0;


    i += 2;

    for (card_num = 0; card_num < CARD_NUMS; card_num++) {
        while (line[i] == ' ') i++;
        card.have[card_num] = 0;

        while ('0' <= line[i] && line[i] <= '9') {
            card.have[card_num] *= 10;
            card.have[card_num] += line[i] - '0';
            i++;
        }

        if (line[i] == '\0') break;
    }
    for (; card_num < CARD_NUMS; card_num++) card.winning[card_num] = 0;


    return card;
}

int matching_count(Card card) {
    int count = 0;

    for (int i = 0; i < CARD_NUMS; i++) {
        int test_for = card.have[i];
        if (test_for == 0) continue;

        for (int j = 0; j < WINNING_NUMS; j++) {
            if (card.winning[j] == test_for) {
                count++;
                break;
            }
        }
    }

    return count;
}

int get_card_value(Card card) {
    int count = matching_count(card);
    return count ? 1 << (count - 1) : 0;
}

void display_card(Card card) {
    printf("Card %3d: ", card.card_num);
    for (int i = 0; i < WINNING_NUMS; i++) {
        printf("%2d ", card.winning[i]);
    }
    printf("| ");
    for (int i = 0; i < CARD_NUMS; i++) {
        printf("%2d ", card.have[i]);
    }

    printf("\n");
}

static int memoized[MAX_CARD_COUNT];

void setup_memoized() {
    for (int i = 0; i < MAX_CARD_COUNT; i++) {
        memoized[i] = -1;
    }
}

int memoized_card_matches(Card card) {
    if (memoized[card.card_num] != -1) return memoized[card.card_num];
    else {
        memoized[card.card_num] = matching_count(card);
        return memoized[card.card_num];
    }
}

void add_copies(Card card, int copies, int *counts) {
    int matches = matching_count(card);
    for (int i = 0; i < matches; i++) {
        counts[card.card_num + i] += copies;
    }
}



int main() {
    FILE *fptr;
    char myString[400];
    int counts[MAX_CARD_COUNT];
    Card curr_card;
    int part1;
    int part2;
    int i;

    for (i = 0; i < MAX_CARD_COUNT; i++) {
        counts[i] = 1;
    }
    

    fptr = fopen("day4/input.txt", "r");

    part1 = 0;
    part2 = 0;

    while (fgets(myString, 400, fptr)) {
        curr_card = parse_line(myString);

        part1 += get_card_value(curr_card);
        add_copies(curr_card, counts[curr_card.card_num - 1], counts);
    }

    // Close the file
    fclose(fptr);

    for (int i = 0; i < curr_card.card_num; i++) {
        part2 += counts[i];
    }


    printf("Part 1: %d\n", part1);
    printf("Part 2: %d\n", part2);

    return 0;
}
