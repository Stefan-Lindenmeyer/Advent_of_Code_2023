from os import path, getcwd

# Task:
    
# Part One:
# Combine first digit and last digit in each line to get a single two digit number
# Then add all resulting numbers for the answer

# Part Two:
# Same as Part One but digits can be spelled out


INPUT_PATH = path.join(getcwd(), 'day_01/input.txt')

def part_one():
    final_value = 0
    
    with open(INPUT_PATH, 'r') as input_file:
        for line in input_file:
            value_str = '-'
            for letter in line:
                if letter.isdigit():
                    value_str = letter
                    break
            for letter in reversed(line):
                if letter.isdigit():
                    value_str += letter
                    break
            if value_str == '-' or not value_str.isdigit() or len(value_str) != 2:
                print('Invalid Input:', value_str)
                exit()
            final_value += int(value_str)
            
    assert final_value == 55712
    # print(final_value)
            
def part_two():
    digit_strings = {1: ['1', 'one'], 2: ['2', 'two'], 3: ['3', 'three'], 4: ['4', 'four'], 5: ['5', 'five'], 6: ['6', 'six'], 7: ['7', 'seven'], 8: ['8', 'eight'], 9: ['9', 'nine']}
    #digit_strings = {1: ['1'], 2: ['2'], 3: ['3'], 4: ['4'], 5: ['5'], 6: ['6'], 7: ['7'], 8: ['8'], 9: ['9']}
    final_value=0
    
    with open(INPUT_PATH, 'r') as input_file:
        for line in input_file:

            leftmost_digit = 0
            rightmost_digit = 0
            leftmost_pos = 9999999999999999999999
            rightmost_pos = -1
            for key, value in digit_strings.items():
                for digit in value:
                    position = line.find(digit)
                    if position != -1 and position < leftmost_pos:
                        leftmost_pos = position
                        leftmost_digit = key

                    position = line.rfind(digit)
                    if position != -1 and position > rightmost_pos:
                        rightmost_pos = position
                        rightmost_digit = key

            if leftmost_digit == 0 or rightmost_digit == 0:
                print('Invalid Input:', line)
                exit()

            final_value += leftmost_digit * 10 +rightmost_digit

    assert final_value == 55413
    # print(final_value)
        

if __name__ == '__main__':
    part_one()
    part_two()