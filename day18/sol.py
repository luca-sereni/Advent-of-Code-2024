possible_directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]

def bfs(start, grid, end) -> int:
    queue = [(start, 0)]
    visited = set()
    visited.add(start)

    while queue:
        current = queue.pop(0)
        curr_pos = current[0]
        curr_score = current[1]
        if curr_pos == end:
            return curr_score

        for direction in possible_directions:
            next_x = curr_pos[0] + direction[0]
            next_y = curr_pos[1] + direction[1]

            if 0 <= next_x < len(grid[0]) and 0 <= next_y < len(grid) and grid[next_y][next_x] != '#':
                next_pos = (next_x, next_y)
                if next_pos not in visited:
                    visited.add(next_pos)
                    queue.append((next_pos, curr_score + 1))

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

print("Score: {}".format(bfs((0, 0), grid, (70, 70))))
#print_grid(grid)