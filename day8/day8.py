from collections import defaultdict

file = open("day8/input.txt", "r")
lines = file.readlines()
file.close()

height = len(lines)
width = len(lines[0].strip())

def difference(ant1: tuple, ant2: tuple) -> tuple:
    return (ant1[0] - ant2[0], ant1[1] - ant2[1])

def compute_differences(antenna: tuple, others: list) -> list:
    differences = []
    for other in others:
        differences.append(difference(antenna, other))
    return differences

def is_valid_position(pos: tuple) -> bool:
    return 0 <= pos[0] < height and 0 <= pos[1] < width

def compute_antinodes(antennas: dict) -> list:
    antinodes = []
    for key, values in antennas.items():
        for i in range(len(values)):
            antenna = values[i]
            for j in range(len(values)):
                if i == j:
                    continue
                diffs = compute_differences(antenna, values)
                for diff in diffs:
                    possible_pos1 = (antenna[0] + diff[0], antenna[1] + diff[1])
                    possible_pos2 = (antenna[0] - diff[0], antenna[1] - diff[1])
                    if is_valid_position(possible_pos1) and possible_pos1 not in antinodes and possible_pos1 not in antennas[key]:
                        antinodes.append(possible_pos1)
                    if is_valid_position(possible_pos2) and possible_pos2 not in antinodes and possible_pos2 not in antennas[key]:
                        antinodes.append(possible_pos2)

    return antinodes

def compute_antinodes_v2(antennas: dict) -> list:
    antinodes = []
    for key, values in antennas.items():
        # If there is a single antenna, no antinode to add
        if len(values) < 2:
            continue
        for i in range(len(values)):
            antenna = values[i]
            if antenna not in antinodes:
                antinodes.append(antenna)
            for j in range(len(values)):
                diffs = compute_differences(antenna, values)
                for diff in diffs:
                    # To prevent infinite loop
                    if diff[0] == 0 and diff[1] == 0:
                        continue
                    
                    increment = 0
                    possible_pos1 = (antenna[0] + (increment*diff[0]), antenna[1] + (increment*diff[1]))
                    while is_valid_position(possible_pos1) == True:
                        if possible_pos1 not in antinodes:
                            antinodes.append(possible_pos1)
                        increment += 1
                        possible_pos1 = (antenna[0] + increment*diff[0], antenna[1] + increment*diff[1])
                    
                    increment = 0
                    possible_pos2 = (antenna[0] - increment*diff[0], antenna[1] - increment*diff[1])
                    while is_valid_position(possible_pos2) == True:
                        if possible_pos2 not in antinodes:
                            antinodes.append(possible_pos2)
                        increment += 1
                        possible_pos2 = (antenna[0] - increment*diff[0], antenna[1] - increment*diff[1])

    return antinodes

antennas = defaultdict(list)
for i in range(0, height):
    line = lines[i].strip()
    for j in range(0, width):
        if line[j] != '.':
            antennas[line[j]].append((i, j))

# Part 1
antinodes = compute_antinodes(antennas)
print(len(antinodes))

# Part 2
antinodes_v2 = compute_antinodes_v2(antennas)
print(len(antinodes_v2))