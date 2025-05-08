from collections import deque

possible_directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]

def bfs(start, grid, end) -> list:
    # BFS algorithm to find the shortest path in a grid
    minimum_path = []
    queue = deque([(start, 0)])
    visited = set()
    visited.add(start)
    parent = {}

    while queue:
        curr_pos, curr_score = queue.popleft()
        if curr_pos == end:
            path = []
            while curr_pos != start:
                path.append(curr_pos)
                curr_pos = parent[curr_pos]
            path.append(start)
            path.reverse()
            if not minimum_path or len(path) < len(minimum_path):
                minimum_path = path

            return minimum_path

        for direction in possible_directions:
            next_x = curr_pos[0] + direction[0]
            next_y = curr_pos[1] + direction[1]

            if 0 <= next_x < len(grid[0]) and 0 <= next_y < len(grid) and grid[next_y][next_x] != '#':
                next_pos = (next_x, next_y)
                if next_pos not in visited:
                    visited.add(next_pos)
                    parent[next_pos] = curr_pos
                    queue.append((next_pos, curr_score + 1))

    return minimum_path
                    

def print_grid(grid):
    for row in grid:
        print(''.join(row))
    print()

file = open("input.txt", "r")

lines = file.readlines()
file.close()
lines = [line.strip() for line in lines]

for i in range(len(lines)):
    lines[i] = lines[i].split(",")
    for j in range(len(lines[i])):
        lines[i][j] = int(lines[i][j])
lines = [tuple(line) for line in lines]

DIM_GRID = 71

grid = [['.' for _ in range(DIM_GRID)] for _ in range(DIM_GRID)]

for i in range(1024):
    x = lines[i][0]
    y = lines[i][1]

    grid[y][x] = '#'

minimum_path = bfs((0, 0), grid, (70, 70))
print("Score: {}".format(len(minimum_path) - 1))

# Part 2
# We add one obstacle at a time and check if a path can be found
# If not, we print the coordinates of this last obstacle added.

for i in range(1025, len(lines)):
    x = lines[i][0]
    y = lines[i][1]

    grid[y][x] = '#'
    minimum_path = bfs((0, 0), grid, (70, 70))
    if len(minimum_path) == 0:
        print(x, y)
        break
