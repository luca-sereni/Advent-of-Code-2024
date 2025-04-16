def walk(map: list, i: int, j: int, last_position_value: int, visited: list) -> int:
    if map[i][j] != last_position_value + 1:
        return 0
    if map[i][j] == 9 and (i, j) not in visited:
        visited.append((i, j))
        return 1
    count = 0

    # Go left
    if j > 0:
        count += walk(map, i, j - 1, map[i][j], visited)

    # Go right
    if j < len(map[i]) - 1:
        count += walk(map, i, j + 1, map[i][j], visited)

    # Go up
    if i > 0:
        count += walk(map, i - 1, j, map[i][j], visited)

    # Go down
    if i < len(map) - 1:
        count += walk(map, i + 1, j, map[i][j], visited)
    
    return count

file = open("day10/input.txt", "r")
lines = file.readlines()
file.close()

map = [[] for _ in lines]
trailheads = []

for i, line in enumerate(lines):
    for j, num in enumerate(line.strip()):
        n = int(num)
        map[i].append(n)
        if n == 0:
            trailheads.append((i, j))

sum = 0
for trailhead in trailheads:
    visited = []
    count = walk(map, trailhead[0], trailhead[1], -1, visited)
    sum += count

print(sum)