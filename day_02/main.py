from os import path, getcwd
from math import prod

# Task:
    
# A game(/line) consists out of multiple draws and replacements of colored cubed out of a bag

# Part One:
# For each game find out if it would have been possible if only 12 red cubes, 13 green cubes, and 14 blue cubes are in the bag.
# Then add all IDs of possible games for the answer

# Part Two:
# For each game find out the lowest number of cubes in the bag that would make the game possible
# Multiply the cube amounts in each game and then add all of them together for the answer

INPUT_PATH = path.join(getcwd(), 'day_02/input.txt')

def part_one():
    cubes_in_bag = {'red': 12, 'blue': 14, 'green': 13}
    result = 0
    
    with open(INPUT_PATH, 'r') as input_file:
        for line in input_file:
        
            g, d = line.split(':')
            game_id = int(g.split(' ')[1])
            draws = d.split(';')
            impossible = False
            
            for draw in draws:
                for c in draw.lstrip().split(', '):
                    count, color = c.split()
                    if int(count) > cubes_in_bag[color]:
                        impossible = True
                        break
                if impossible: break
                
            if not impossible: result += game_id
            
    assert result == 2685
    # print(result)
        


def part_two():
    result = 0
    
    with open(INPUT_PATH, 'r') as input_file:
        for line in input_file:
            cubes_in_bag = {'red': 0, 'blue': 0, 'green': 0}

            draws = line.split(':')[1].split(';')
            for draw in draws:
                for c in draw.lstrip().split(', '):
                    count, color = c.split()
                    if int(count) > cubes_in_bag[color]: cubes_in_bag[color] = int(count)
                
            result += prod([x for x in cubes_in_bag.values()])
            
    assert result == 83707
    # print(result)

if __name__ == '__main__':
    part_one()
    part_two()