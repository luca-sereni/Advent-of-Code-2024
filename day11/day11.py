def add_occurrence(dict: dict, value: int, times: int):
    if value in dict:
        dict[value] += times
    else:
        dict[value] = times

def apply_rules(stones: list, num_iterations: int) -> int:
    # Create a dictionary to store the occurrences of the numbers
    stones_dict = {}
    for stone in stones:
        stones_dict[stone] = 1
    for _ in range(num_iterations):
        next = {}
        for item in stones_dict.keys():
            times = stones_dict[item]
            if item == 0:
                # 0 becomes 1
                add_occurrence(next, 1, times)
            else:
                value = item
                length = len(str(item))
                if length % 2 == 0:
                    # Even length -> split in half
                    item = str(item)
                    left_part = item[:length // 2]
                    right_part = item[length // 2:]
                    add_occurrence(next, int(left_part), times)
                    add_occurrence(next, int(right_part), times)
                else:
                    # multiply by 2024
                    new_number = 2024 * int(item)
                    add_occurrence(next, new_number, times)
        stones_dict = next

    return sum(stones_dict.values())

file = open("day11/input.txt", "r")
data = file.read().splitlines()
file.close()

stones = []

for line in data:
    words = line.split()
    for word in words:
        if word.isdigit():
            stones.append(int(word))

stones_part2 = [stone for stone in stones]
# Part 1
NUM_BLINK_PART1 = 25
print(apply_rules(stones, NUM_BLINK_PART1))

# Part 2
NUM_BLINK_PART2 = 75
print(apply_rules(stones_part2, NUM_BLINK_PART2))

