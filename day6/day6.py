def find_cross(i: int, j:int, grid: list) -> int:
    for k in range(len(grid)):
        if grid[k][0] == i and grid[k][1] == j:
            return k
    return -1

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

i = starting_position[0]
j = starting_position[1]

obstacles = []
directions = [("up", '^', 'U'), ("down", 'v', 'D'), ("left", '<', 'L'), ("right", '>', 'R')]
crosses = []


add_new_obstacle = True
current_direction = directions[0]

while add_new_obstacle:
    temp_grid = grid.copy()
    i = starting_position[0]
    j = starting_position[1]
    add_new_obstacle = False
    while i >= 0 and j >= 0 and i < len(temp_grid) and j < len(temp_grid[i]):
        match current_direction[0]:
            case "up":
                if i == 0:
                    break
                if grid[i - 1][j] != "#":
                    if grid[i - 1][j] == "R": # a new obstacle could be added
                        if obstacles.count((i - 2, j)) == 0 and i-2 != starting_position[0] and j != starting_position[1]:
                            obstacles.append((i - 2, j))
                            add_new_obstacle = True
                            break
                        else:
                            grid[i - 1][j] = "+"
                            crosses.append((i - 1, j, ['R', 'U']))
                    elif grid[i - 1][j] == "+":
                        index_cross = find_cross(i - 1, j, crosses)
                        if index_cross != -1:
                            crosses[index_cross][2].append('U')
                            if crosses[index_cross][2].count('R') > 0:
                                if obstacles.count((i - 2, j)) == 0 and i-2 != starting_position[0] and j != starting_position[1]:
                                    obstacles.append((i - 2, j))
                                    add_new_obstacle = True
                                    break
                    grid[i][j] = current_direction[2]
                    i -= 1
                    grid[i][j] = current_direction[1]
                else:
                    if grid[i][j] == '.':
                        grid[i][j] = '+'
                        crosses.append((i, j, ['U', 'R']))
                    else:
                        if grid[i][j] == "+":
                            index_cross = find_cross(i, j, crosses)
                            if index_cross != -1:
                                crosses[index_cross][2].append('U')
                                crosses[index_cross][2].append('R')
                        else:
                            crosses.append((i, j, ['U', 'R']))
                            grid[i][j] = '+'
                    current_direction = directions[3]  # right
            case "down":
                if i == len(grid) - 1:
                    break
                if grid[i + 1][j] != "#":
                    if grid[i + 1][j] == "L": # a new obstacle could be added
                        if obstacles.count((i + 2, j)) == 0 and i+2 != starting_position[0] and j != starting_position[1]:
                            obstacles.append((i + 2, j))
                            add_new_obstacle = True
                            break
                        else:
                            grid[i + 1][j] = "+"
                            crosses.append((i + 1, j, ['L', 'D']))
                    elif grid[i + 1][j] == "+":
                        index_cross = find_cross(i + 1, j, crosses)
                        if index_cross != -1:
                            crosses[index_cross][2].append('D')
                            if crosses[index_cross][2].count('L') > 0:
                                if obstacles.count((i + 2, j)) == 0 and i+2 != starting_position[0] and j != starting_position[1]:
                                    obstacles.append((i + 2, j))
                                    add_new_obstacle = True
                                    break
                    grid[i][j] = current_direction[2]
                    i += 1
                    grid[i][j] = current_direction[1]
                else:
                    if grid[i][j] == "+":
                        index_cross = find_cross(i, j, crosses)
                        if index_cross != -1:
                            crosses[index_cross][2].append('D')
                            crosses[index_cross][2].append('L')
                    else:
                        crosses.append((i, j, ['D', 'L']))
                        grid[i][j] = '+'
                    current_direction = directions[2]  # left
            case "left":
                if j == 0:
                    break
                if grid[i][j - 1] != "#":
                    if grid[i][j - 1] == "U": # a new obstacle could be added
                        if obstacles.count((i, j - 2)) == 0 and i != starting_position[0] and j-2 != starting_position[1]:
                            obstacles.append((i, j - 2))
                            add_new_obstacle = True
                            break
                        else:
                            grid[i][j - 1] = "+"
                            crosses.append((i, j - 1, ['U', 'L']))
                    elif grid[i][j - 1] == "+":
                        index_cross = find_cross(i, j - 1, crosses)
                        if index_cross != -1:
                            crosses[index_cross][2].append('L')
                            if crosses[index_cross][2].count('U') > 0:
                                if obstacles.count((i, j - 2)) == 0 and i != starting_position[0] and j-2 != starting_position[1]:
                                    obstacles.append((i, j - 2))
                                    add_new_obstacle = True
                                    break
                    grid[i][j] = current_direction[2]
                    j -= 1
                    grid[i][j] = current_direction[1]
                else:
                    if grid[i][j] == "+":
                        index_cross = find_cross(i, j, crosses)
                        if index_cross != -1:
                            crosses[index_cross][2].append('L')
                            crosses[index_cross][2].append('U')
                    else:
                        crosses.append((i, j, ['L', 'U']))
                        grid[i][j] = '+'
                    current_direction = directions[0]  # up
            case "right":
                if j == len(grid[i]) - 1:
                    break
                if grid[i][j + 1] != "#":
                    if grid[i][j + 1] == "D": # a new obstacle could be added
                        if obstacles.count((i, j + 2)) == 0 and i != starting_position[0] and j+2 != starting_position[1]:
                            obstacles.append((i, j + 2))
                            add_new_obstacle = True
                            break
                        else:
                            grid[i][j + 1] = "+"
                            crosses.append((i, j + 1, ['D', 'R']))
                    elif grid[i][j - 1] == "+":
                        index_cross = find_cross(i, j - 1, crosses)
                        if index_cross != -1:
                            crosses[index_cross][2].append('R')
                            if crosses[index_cross][2].count('D') > 0:
                                if obstacles.count((i, j + 2)) == 0 and i != starting_position[0] and j+2 != starting_position[1]:
                                    obstacles.append((i, j + 2))
                                    add_new_obstacle = True
                                    break
                    grid[i][j] = current_direction[2]
                    j += 1
                    grid[i][j] = current_direction[1]
                else:
                    if grid[i][j] == "+":
                        index_cross = find_cross(i, j, crosses)
                        if index_cross != -1:
                            crosses[index_cross][2].append('R')
                            crosses[index_cross][2].append('D')
                    else:
                        crosses.append((i, j, ['R', 'D']))
                        grid[i][j] = '+'
                    current_direction = directions[1]  # down

print(crosses)
print(len(obstacles))


""" FIRST PART
directions = [("up", '^'), ("down", 'v'), ("left", '<'), ("right", '>')]

current_direction = directions[0]
position_count = 1

while i >= 0 and j >= 0 and i < len(grid) and j < len(grid[i]):
    match current_direction[0]:
        case "up":
            if i == 0:
                break
            if grid[i - 1][j] != "#":
                if grid[i - 1][j] == ".":
                    position_count += 1
                grid[i][j] = 'X'
                i -= 1
                grid[i][j] = current_direction[1]
            else:
                current_direction = directions[3]  # right
        case "down":
            if i == len(grid) - 1:
                break
            if grid[i + 1][j] != "#":
                if grid[i + 1][j] == ".":
                    position_count += 1
                grid[i][j] = "X"
                i += 1
                grid[i][j] = current_direction[1]
            else:
                current_direction = directions[2]  # left
        case "left":
            if j == 0:
                break
            if grid[i][j - 1] != "#":
                if grid[i][j - 1] == ".":
                    position_count += 1
                grid[i][j] = "X"
                j -= 1
                grid[i][j] = current_direction[1]
            else:
                current_direction = directions[0]  # up
        case "right":
            if j == len(grid[i]) - 1:
                break
            if grid[i][j + 1] != "#":
                if grid[i][j + 1] == ".":
                    position_count += 1
                grid[i][j] = "X"
                j += 1
                grid[i][j] = current_direction[1]
            else:
                current_direction = directions[1]  # down

print(position_count)
"""