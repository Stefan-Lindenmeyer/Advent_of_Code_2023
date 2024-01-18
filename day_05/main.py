from os import path, getcwd
from multiprocessing import Process, Queue

# Task:

# The input lists maps with a source, a destination and a range.
    
# Part One:
# Follow all seeds given at the top of the input and find the lowest location number.

# Part Two:
# Seeds now come in pairs of start and range.
# Follow all seeds given at the top of the input and find the lowest location number.

INPUT_PATH = path.join(getcwd(), 'day_05/input.txt')

def part_one():
    seeds = list()
    maps = {'seed-to-soil': list(), 
            'soil-to-fertilizer': list(), 
            'fertilizer-to-water': list(), 
            'water-to-light': list(), 
            'light-to-temperature': list(), 
            'temperature-to-humidity': list(), 
            'humidity-to-location': list()}
    
    with open(INPUT_PATH, 'r') as file:

        current_map = ''

        for line in file:
            line = line.strip('\t\r\n')

            if line == '': continue
            if 'seeds' in line: 
                seeds = [int(x) for x in line.split(':')[1].split()]
                continue
    
            if line.split()[0] in maps.keys(): 
                current_map = line.split()[0]
                continue

            maps[current_map].append([int(x) for x in line.split()])

    locations = list()
    for seed in seeds:
        
        position = seed
        for value in maps.values():# dict order is insertion order since python 3.7 
            for dest, source, r in value:
                if position in range(source, source + r):
                    position = dest + (position - source)
                    break
            #else: pos stays same
        locations.append(position)
    
    assert min(locations) == 251346198
    # print(min(locations))


def part_two():
    seeds = list()
    maps = {'seed-to-soil': list(), 
            'soil-to-fertilizer': list(), 
            'fertilizer-to-water': list(), 
            'water-to-light': list(), 
            'light-to-temperature': list(), 
            'temperature-to-humidity': list(), 
            'humidity-to-location': list()}
    
    with open(INPUT_PATH, 'r') as file:

        current_map = ''

        for line in file:
            line = line.strip('\t\r\n')

            if line == '': continue
            if 'seeds' in line: 
                temp = [int(x) for x in line.split(':')[1].split()]
                for start, r in zip(temp[0::2], temp[1::2]):
                    seeds.append((start, r))  
                continue
    
            if line.split()[0] in maps.keys(): 
                current_map = line.split()[0]
                continue

            maps[current_map].append([int(x) for x in line.split()])

    # total_iterations = sum([r for seeds, r in seeds])
    # print(total_iterations)

    location = -1

    processes = list()

    for seed, r in seeds:
        q = Queue()
        p = Process(target=solve, args=(seed, r, maps, q))
        p.start()
        processes.append((p, q))
        
    for p, q in processes:
        p.join()
        result = q.get()
        location = min(location, result) if location >= 0 else result

    assert location == 72263011
    # print(location)

def solve(seed: int, r: int, maps: dict, q: Queue):
    location = -1

    for s in range(seed, seed + r):
        #count += 1
        #print(count, count/total_iterations)
        position = s
        for value in maps.values():# dict order is insertion order since python 3.7 
            for dest, source, r in value:
                if position >= source and position < source + r:
                    position = dest + (position - source)
                    break
            #else: pos stays same

        location = min(location, position) if location >= 0 else position
    q.put(location)

if __name__ == '__main__':
    part_one()
    
    # Caution: Takes about an hour to run with 20 cores
    part_two()
    