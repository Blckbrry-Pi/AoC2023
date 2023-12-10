import functools

Card = int
HandType = int
Hand = tuple[Card, Card, Card, Card, Card]

C2, C3, C4, C5, C6, C7, C8, C9, CT, CJ, CQ, CK, CA = range(13)

def jokers_to_neg1(hand: Hand) -> Hand:
    return tuple(map(lambda x: -1 if x == CJ else x, hand))

def from_part1_to_part(hand: Hand, part2: bool) -> Hand:
    if part2:
        return jokers_to_neg1(hand)
    else:
        return hand

def hand_to_num(hand: Hand) -> int:
    return hand[0] * 1e12 + hand[1] * 1e9 + hand[2] * 1e6 + hand[3] * 1e3 + hand[4]

HIGH_CARD, ONE_PAIR, TWO_PAIR, THREE_OF_A_KIND, FULL_HOUSE, FOUR_OF_A_KIND, FIVE_OR_A_KIND = range(7)

def get_hand_type(hand: Hand) -> HandType:
    hand = list(hand)
    hand.sort()
    hand = tuple(hand)
    if hand[0] == hand[1] == hand[2] == hand[3] == hand[4]:
        return FIVE_OR_A_KIND
    elif hand[0] == hand[1] == hand[2] == hand[3] or hand[1] == hand[2] == hand[3] == hand[4]:
        return FOUR_OF_A_KIND
    elif hand[0] == hand[1] == hand[2] and hand[3] == hand[4] or hand[0] == hand[1] and hand[2] == hand[3] == hand[4]:
        return FULL_HOUSE
    elif hand[0] == hand[1] == hand[2] or hand[1] == hand[2] == hand[3] or hand[2] == hand[3] == hand[4]:
        return THREE_OF_A_KIND
    elif hand[0] == hand[1] and hand[2] == hand[3] or hand[0] == hand[1] and hand[3] == hand[4] or hand[1] == hand[2] and hand[3] == hand[4]:
        return TWO_PAIR
    elif hand[0] == hand[1] or hand[1] == hand[2] or hand[2] == hand[3] or hand[3] == hand[4]:
        return ONE_PAIR
    else:
        return HIGH_CARD

def get_hand_type_from_part1(hand: Hand, part2: bool) -> HandType:
    if part2:
        hand_joker = jokers_to_neg1(hand)
        for i in range(5):
            if hand_joker[i] == -1:
                best_hand_type_so_far = HIGH_CARD

                new_hand = list(hand)
                for card in range(13):
                    if card == CJ:
                        continue
                    new_hand[i] = card
                    new_hand_type = get_hand_type_from_part1(tuple(new_hand), part2)
                    best_hand_type_so_far = max(best_hand_type_so_far, new_hand_type)
                
                return best_hand_type_so_far
    
    return get_hand_type(from_part1_to_part(hand, part2))

def hand_a_better_than_b(hand_a: Hand, hand_b: Hand, part2: bool) -> bool:
    hand_a_type = get_hand_type_from_part1(hand_a, part2)
    hand_b_type = get_hand_type_from_part1(hand_b, part2)
    if hand_a_type > hand_b_type:
        return True
    elif hand_a_type < hand_b_type:
        return False
    else:
        if hand_to_num(from_part1_to_part(hand_a, part2)) > hand_to_num(from_part1_to_part(hand_b, part2)):
            return True
        else:
            return False




def get_file_lines(filename: str) -> list[str]:
    with open(filename) as file:
        return [line.rstrip() for line in file if line.strip() != ""]

def process_line(line: str) -> tuple[Hand, int]:
    cards, bid = line.split(" ")

    card_map = "23456789TJQKA"
    hand = tuple(card_map.index(c) for c in cards)

    bid = int(bid)

    return (hand, bid)

def get_sorted_part(input_list: list[tuple[Hand, int]], part2: bool) -> list[tuple[Hand, int]]:
    return sorted(input_list, key=functools.cmp_to_key(lambda a, b: hand_a_better_than_b(a[0], b[0], part2) * 2 - 1))

def get_val_list(input_list: list[tuple[Hand, int]]) -> list[int]:
    total = 0
    for i in range(len(input_list)):
        total += input_list[i][1] * (i + 1)
    return total


processed_lines = map(process_line, get_file_lines("./day07/input.txt"))
part1 = get_sorted_part(processed_lines, False)
print("Part 1:", get_val_list(part1))

processed_lines = map(process_line, get_file_lines("./day07/input.txt"))
part2 = get_sorted_part(processed_lines, True)
print("Part 2:", get_val_list(part2))
