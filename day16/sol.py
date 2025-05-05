from collections import deque

directions = [(1, 0), (0, -1), (-1, 0), (0, 1)]

def go_right(cur_dir):
    return directions[(directions.index(cur_dir) + 1 + len(directions)) % len(directions)]

def go_left(cur_dir):
    return directions[(directions.index(cur_dir) - 1 + len(directions)) % len(directions)]

INCREMENT_SCORE_VALUE = 1
CHANGE_DIRECTION_SCORE = 1000

def bfs(maze: list, start_x: int, start_y: int, end_x: int, end_y: int):
    queue = deque()
    start = (start_x, start_y, (0, 1), 0)  # x, y, direction, score
    maze[start_x][start_y] = 0
    queue.append(start)

    while queue:
        current = queue.popleft()
        current_x = current[0]
        current_y = current[1]
        current_dir = current[2]
        current_score = current[3]

        possible_directions = [
            (current_dir, current_score + INCREMENT_SCORE_VALUE),
            (go_left(current_dir), current_score + CHANGE_DIRECTION_SCORE + INCREMENT_SCORE_VALUE),
            (go_right(current_dir), current_score + CHANGE_DIRECTION_SCORE + INCREMENT_SCORE_VALUE)
        ]

        for new_dir, new_score in possible_directions:
            new_x, new_y = current_x + new_dir[0], current_y + new_dir[1]
            if maze[new_x][new_y] == "#":
                continue

            if (maze[new_x][new_y] == '.' or maze[new_x][new_y] == 'E') or (isinstance(maze[new_x][new_y], int) and maze[new_x][new_y] > new_score):
                maze[new_x][new_y] = new_score
                queue.append((new_x, new_y, new_dir, new_score))

    return maze[end_x][end_y]

def count_num_places(maze: list, end_x: int, end_y: int) -> int:
    num_places = 1
    queue = deque()
    visited = set()

    end = (end_x, end_y, (1, 0), maze[end_x][end_y])
    queue.append(end)

    while queue:
        curr_pos = queue.popleft()
        curr_x = curr_pos[0]
        curr_y = curr_pos[1]
        curr_dir = curr_pos[2]
        curr_score = curr_pos[3]

        possible_directions = [
            curr_dir,
            go_left(curr_dir),
            go_right(curr_dir)
        ]

        for direction in possible_directions:
            new_x, new_y = curr_x + direction[0], curr_y + direction[1]

            if direction == curr_dir:
                new_score = curr_score - INCREMENT_SCORE_VALUE
            else:
                new_score = curr_score - (CHANGE_DIRECTION_SCORE + INCREMENT_SCORE_VALUE)
            if (isinstance(maze[new_x][new_y], int)) and maze[new_x][new_y] <= new_score and (new_x, new_y) not in visited:
                visited.add((new_x, new_y))
                num_places += 1
                queue.append((new_x, new_y, direction, new_score))

    return num_places

maze = []
with open("input.txt", 'r') as file:
    maze = [list(line.strip()) for line in file if line.strip()]
    file.close()

for line in maze:
    for i in range(len(line)):
        if line[i] == 'E':
            end_x = maze.index(line)
            end_y = i
        if line[i] == 'S':
            start_x = maze.index(line)
            start_y = i

print(f"Score (PART 1): {bfs(maze, start_x, start_y, end_x, end_y)}")

print(f"Num places (PART 2): {count_num_places(maze, end_x, end_y)}")