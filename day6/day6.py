import copy

file_to_read = open("day6/input.txt", "r")

grid = []
starting_position = ()

for line in file_to_read:
    grid.append(list(line.strip("\n")))

file_to_read.close()

#find starting position
found = False
i = 0
while i < len(grid) and not found:
    for j in range(len(grid[i])):
        if grid[i][j] == "^":
            starting_position = (i, j)
            found = True
            break
    i += 1

directions = {"up": ('^', 'U'), "down": ('v', 'D'), "left": ('<', 'L'), "right": ('>', 'R')}
current_direction = directions["up"]  #Start going up

""" FIRST PART
position_count = 1

while i >= 0 and j >= 0 and i < len(grid) and j < len(grid[i]):
    match current_direction[1]:
        case "U":
            if i == 0:
                break
            if grid[i - 1][j] != "#":
                if grid[i - 1][j] == ".":
                    position_count += 1
                grid[i][j] = 'X'
                i -= 1
                grid[i][j] = current_direction[0]
            else:
                current_direction = directions["right"]  # right
        case "D":
            if i == len(grid) - 1:
                break
            if grid[i + 1][j] != "#":
                if grid[i + 1][j] == ".":
                    position_count += 1
                grid[i][j] = "X"
                i += 1
                grid[i][j] = current_direction[0]
            else:
                current_direction = directions["left"]  # left
        case "L":
            if j == 0:
                break
            if grid[i][j - 1] != "#":
                if grid[i][j - 1] == ".":
                    position_count += 1
                grid[i][j] = "X"
                j -= 1
                grid[i][j] = current_direction[0]
            else:
                current_direction = directions["up"]  # up
        case "R":
            if j == len(grid[i]) - 1:
                break
            if grid[i][j + 1] != "#":
                if grid[i][j + 1] == ".":
                    position_count += 1
                grid[i][j] = "X"
                j += 1
                grid[i][j] = current_direction[0]
            else:
                current_direction = directions["down"]  # down

print(position_count)
"""

MAX_LOOP_COUNT = 50

def run_simulation(temp_grid: list[list], current_direction: tuple) -> bool:
    i = starting_position[0]
    j = starting_position[1]
    current_direction = current_direction
    to_add = False
    obstacles_found = []
    while i >= 0 and j >= 0 and i < len(temp_grid) and j < len(temp_grid[0]):
        match current_direction[1]:
            case "U":
                if i == 0:
                    #Reached the top of the grid
                    to_add = False
                    break
                if temp_grid[i - 1][j] != "#":
                    i -= 1 #Moving up
                else:
                    obstacles_found.append((i - 1, j, current_direction[1]))
                    if obstacles_found.count((i - 1, j, current_direction[1])) > MAX_LOOP_COUNT:
                        to_add = True
                        break
                    current_direction = directions["right"]  # right
            case "D":
                if i == len(temp_grid) - 1:
                    #Reached the bottom of the grid
                    to_add = False
                    break
                if temp_grid[i + 1][j] != "#":
                    i += 1 #Moving down
                else:
                    obstacles_found.append((i + 1, j, current_direction[1]))
                    if obstacles_found.count((i + 1, j, current_direction[1])) > MAX_LOOP_COUNT:
                        to_add = True
                        break
                    current_direction = directions["left"]  # left
            case "L":
                if j == 0:
                    #Reached the left part of the grid
                    to_add = False
                    break
                if temp_grid[i][j - 1] != "#":
                    j -= 1 #Moving left
                else:
                    obstacles_found.append((i, j - 1, current_direction[1]))
                    if obstacles_found.count((i, j - 1, current_direction[1])) > MAX_LOOP_COUNT:
                        to_add = True
                        break
                    current_direction = directions["up"]  # up
            case "R":
                if j == len(temp_grid[i]) - 1:
                    #Reached the right part of the grid
                    to_add = False
                    break
                if temp_grid[i][j + 1] != "#":
                    j += 1 #Moving right
                else:
                    obstacles_found.append((i, j + 1, current_direction[1]))
                    if obstacles_found.count((i, j + 1, current_direction[1])) > MAX_LOOP_COUNT:
                        to_add = True
                        break
                    current_direction = directions["down"]  # down
    return to_add

obstacles = []
for i in range(len(grid)):
    for j in range(len(grid[i])):
        temp_grid = grid
        if temp_grid[i][j] != "#" and temp_grid[i][j] != '^' and not (i == starting_position[0] - 1 and j == starting_position[1]):
            temp_grid[i][j] = '#'
            to_add = run_simulation(temp_grid, current_direction)
            if to_add:
                print((i, j))
                obstacles.append((i, j))
            temp_grid[i][j] = '.'

print(len(obstacles))