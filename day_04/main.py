from os import path, getcwd

# Task:
    
# A card consists of two sets of numbers, a list of winning numbers and a list of numbers you have seperated by a line (|) 

# Part One:
# Find out how many of the numbers you have are winning numbers for each card. 
# Each found number doubles the value of a card.
# Add up all card values for the answer

# Part Two:
# Find out how many of the numbers you have are winning numbers for each card. 
# For each found number you win a copy of "the next card down". 
# Example: If you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15
# Add up how many scratchcards you won for the answer.

INPUT_PATH = path.join(getcwd(), 'day_04/input.txt')

def part_one():

    result = 0

    with open(INPUT_PATH, 'r') as input_file:
        for line in input_file:
            winning_numbers, numbers = line.split(':')[1].split('|')
            winning_numbers = set(winning_numbers.split())
            numbers = numbers.split()

            points = 0
            for number in numbers:
                if number in winning_numbers:
                    points = 1 if points == 0 else points * 2

            result += points

    assert result == 23847
    # print(result)


def part_two():
    result = 0

    copies = {1: 1}
    current_card = 1

    with open(INPUT_PATH, 'r') as input_file:
        for line in input_file:
            winning_numbers, numbers = line.split(':')[1].split('|')
            winning_numbers = set(winning_numbers.split())
            numbers = numbers.split()

            wins = 0
            for number in numbers:
                if number in winning_numbers:
                    wins += 1
                    if current_card + wins not in copies: copies[current_card + wins] = 0
                    copies[current_card + wins] += 1 * copies[current_card]

            if current_card + 1 not in copies: copies[current_card + 1] = 0
            copies[current_card + 1] += 1
            current_card += 1

    result = sum([0 if key >= current_card else x for key, x in copies.items()])

    assert result == 8570000
    # print(result)


if __name__ == '__main__':
    part_one()
    part_two()