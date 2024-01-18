from os import path, getcwd
from math import prod

# Task:
    
# Part One:
# Find any number adjacent to a symbol (*, #, + ...) and add them together for the answer

# Part Two:
# A gear (*) is adjacent to exactly two numbers
# Find each gear and multiply it's numbers, then add all of them together for the answer

INPUT_PATH = path.join(getcwd(), 'day_03/input.txt')

def clamp(x, minimum, maximum):
    return max(minimum, min(x, maximum))

def part_one():
    
    symbols = set() #positions of symbols
    numbers = list() # list[(part_number, line_num, start, end)] #start and end are generally -1 and +1 of actual position except on the edges (clamp)

    result = 0
    
    with open(INPUT_PATH, 'r') as input_file:
        for line_num, line in enumerate(input_file):
            part_number = ''
            start = 0
            end = 0
            digit_flag = False
            
            line = line.strip('\t\r\n')
            line_len = len(line)

            # Record all places where part numbers and symbols are located
            for pos, letter in enumerate(line):
                if letter == '.': # Case: Found '.', continue
                    if not digit_flag: continue
                    # Special Case: Reached end of number -> record -> reset
                    end = clamp(pos, 0, line_len-1)
                    numbers.append((int(part_number), line_num, start, end))

                    part_number = ''
                    start = 0
                    end = 0
                    digit_flag = False

                elif letter.isdigit(): # Case: Found digit
                    if not digit_flag: start = max(0, pos-1)
                    digit_flag = True
                    part_number += letter

                    if pos == line_len-1: # Special Case: Reached eol, record, break
                        end = line_len-1
                        numbers.append((int(part_number), line_num, start, end))
                        continue # = break
            
                else: # Case: Found symbol
                    if digit_flag: # Special Case: Reached end of number -> record -> reset
                        end = clamp(pos, 0, line_len-1)
                        numbers.append((int(part_number), line_num, start, end))

                        part_number = ''
                        start = 0
                        end = 0
                        digit_flag = False
                    symbols.add(line_num * line_len + pos)

    # Calculate all part numbers
    for part_number, line_n, start, end in numbers:
        for p in range(start, end+1):
            pos = line_n * line_len + p
            if pos in symbols or pos - line_len in symbols or pos + line_len in symbols:
                result += part_number
                break

    assert result == 546563
    # print(result)


def part_two():

    stars = dict() #positions of potential gears
    numbers = list() # list[(part_number, line_num, start, end)] #start and end are generally -1 and +1 of actual position except on the edges (clamp)

    result = 0

    with open(INPUT_PATH, 'r') as input_file:
        for line_num, line in enumerate(input_file):
            part_number = ''
            start = 0
            end = 0
            digit_flag = False
            
            line = line.strip('\t\r\n')
            line_len = len(line)

            for pos, letter in enumerate(line):
                if letter == '.': # Case: Found '.', continue
                    if not digit_flag: continue
                    # Special Case: Reached end of number, record, reset
                    end = clamp(pos, 0, line_len-1)
                    numbers.append((int(part_number), line_num, start, end))

                    part_number = ''
                    start = 0
                    end = 0
                    digit_flag = False

                elif letter.isdigit(): # Case: Found digit
                    if not digit_flag: start = max(0, pos-1)
                    digit_flag = True
                    part_number += letter

                    if pos == line_len-1: # Special Case: Reached eol, record, break
                        end = line_len-1
                        numbers.append((int(part_number), line_num, start, end))
                        continue # = break
            
                else: # Case: Found symbol
                    if digit_flag: # Special Case: Reached end of number, record, reset
                        end = clamp(pos, 0, line_len-1)
                        numbers.append((int(part_number), line_num, start, end))

                        part_number = ''
                        start = 0
                        end = 0
                        digit_flag = False
                    
                    if letter == '*': stars[line_num * line_len + pos] = list()
                
    
    for part_number, line_n, start, end in numbers:
        for p in range(start, end+1):
            pos = line_n * line_len + p
            if pos in stars:
                stars[pos].append(part_number)
            if pos - line_len in stars:
                stars[pos - line_len].append(part_number)
            if pos + line_len in stars:
                stars[pos + line_len].append(part_number)
                
    result = sum([(0 if len(x) != 2 else prod(x)) for x in stars.values()])
    
    assert result == 91031374
    # print(result)


if __name__ == '__main__':
    part_one()
    part_two()