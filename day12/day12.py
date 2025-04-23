def add_plant(plant: chr, position: tuple, plants_dict: dict):
    if plant not in plants_dict:
        plants_dict[plant] = []
    plants_dict[plant].append(position)

def build_border(plant: chr, plants_dict: dict, pos: tuple, visited: list) -> tuple:
    # RECURSIVE:
    # if above there is a different plant, add 1 to the perimeter
    # if on the right there is a different plant, add 1 to the perimete
    # if on the bottom there is a different plant, add 1 to the perimete
    # if on the left there is a different plant, add 1 to the perimete
    # if up there is the same plant, call build_border on that position
    # if on the right there is the same plant, call build_border on that position
    # if on the bottom there is the same plant, call build_border on that position
    # if on the left there is the same plant, call build_border on that position
    # if up there is out of bounds, increment 1 in the perimeter
    # if on the right there is out of bounds, increment 1 in the perimeter
    # if on the bottom there is out of bounds, increment 1 in the perimeter
    # if on the left there is out of bounds, increment 1 in the perimeter

    if pos in visited:
        return (0, 0)
    if pos not in plants_dict[plant]:
        perimeter = 1
        return (0, perimeter)
    
    visited.append(pos)
    area = 0
    perimeter = 0
    if pos[0] < 0 or pos[0] >= HEIGHT or pos[1] < 0 or pos[1] >= WIDTH:
        return (0, 1)
    
    up = build_border(plant, plants_dict, (pos[0] - 1, pos[1]), visited)
    right = build_border(plant, plants_dict, (pos[0], pos[1] + 1), visited)
    down = build_border(plant, plants_dict, (pos[0] + 1, pos[1]), visited)
    left = build_border(plant, plants_dict, (pos[0], pos[1] - 1), visited)

    perimeter += up[1] + right[1] + down[1] + left[1]
    area += up[0] + right[0] + down[0] + left[0]
    return (1 + area, perimeter)

def compute_side_number(border: list) -> int:
    horizontal = 0
    vertical = 0
    # Group tuples according to x[0] and x[1]
    horizontal_border_up = []
    horizontal_border_down = []
    vertical_border_left = []
    vertical_border_right = []

    for (x, y) in border:
        if (x + 1, y) not in border:
            horizontal_border_down.append((x, y))
        if (x - 1, y) not in border:
            horizontal_border_up.append((x, y))
        if (x, y + 1) not in border:
            vertical_border_right.append((x, y))
        if (x, y - 1) not in border:
            vertical_border_left.append((x, y))
    
    for (x, y) in horizontal_border_up:
        if (x, y + 1) not in horizontal_border_up:
            horizontal += 1

    for (x, y) in horizontal_border_down:
        if (x, y + 1) not in horizontal_border_down:
            horizontal += 1

    for (x, y) in vertical_border_left:
        if (x + 1, y) not in vertical_border_left:
            vertical += 1

    for (x, y) in vertical_border_right:
        if (x + 1, y) not in vertical_border_right:
            vertical += 1

    return horizontal + vertical

# Return a list of prices where each price corresponds to a region of the same plant
def compute_price_plant(plant: chr, plants_dict: dict, is_first_part: bool) -> list:
    if plants_dict[plant] == []:
        return []
    price_list = []
    visited = []
    for pos in plants_dict[plant]:
        if pos in visited:
            continue
        if is_first_part:
            (area, perimeter) = build_border(plant, plants_dict, pos, visited)
            price_list.append(area * perimeter)
        else:
            # To detect the elements of a new region
            old_elements_visited = visited.copy()
            (area, perimeter) = build_border(plant, plants_dict, pos, visited)
            # Elements that belong to the new region to compute the price
            new_elements_visited = sorted(list(set(visited) - set(old_elements_visited)), key=lambda x: (x[0], x[1]))
            side_number = compute_side_number(new_elements_visited)
            price_list.append(area * side_number)
    return price_list

file = open("day12/input.txt", "r")
lines = file.readlines()
file.close()

HEIGHT = len(lines)
WIDTH = len(lines[0].strip())

plants_dict = {}

for i, line in enumerate(lines):
    for j in range(len(line.strip())):
        add_plant(line[j], (i, j), plants_dict)

# First part
price_list = []
for key in plants_dict.keys():
    price = compute_price_plant(key, plants_dict, True)
    for price in price:
        if price != 0:
            price_list.append(price)

print(sum(price_list))

# Second part
price_list = []
for key in plants_dict.keys():
    price = compute_price_plant(key, plants_dict, False)
    for price in price:
        if price != 0:
            price_list.append(price)
print(sum(price_list))