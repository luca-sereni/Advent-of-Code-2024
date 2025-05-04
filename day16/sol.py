from collections import deque

directions = [(1, 0), (0, -1), (-1, 0), (0, 1)]

def go_right(cur_dir):
    return directions[(directions.index(cur_dir) + 1 + len(directions)) % len(directions)]

def go_left(cur_dir):
    return directions[(directions.index(cur_dir) - 1 + len(directions)) % len(directions)]

INCREMENT_SCORE_VALUE = 1
CHANGE_DIRECTION_SCORE = 1000

def bfs(maze: list, end_x: int, end_y: int):
    queue = deque()
    start = (len(maze) - 2, 1, (0, 1), 0)  # x, y, direction, score
    maze[len(maze) - 2][1] = 0
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

maze = []
with open("input.txt", 'r') as file:
    maze = [list(line.strip()) for line in file if line.strip()]
    file.close()

for line in maze:
    for i in range(len(line)):
        if line[i] == 'E':
            end_x = maze.index(line)
            end_y = i
            break
print(f"Score: {bfs(maze, end_x, end_y)}")