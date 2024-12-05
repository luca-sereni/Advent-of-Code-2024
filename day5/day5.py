file_to_read = open("day5/input.txt", "r")

def find_first_index(n: int) -> int:
    for i in range(len(rules)):
        if rules[i][0] == n:
            return i
    return -1

def find_previous_element(n: int, elements_already_found: list) -> int:
    first_index = find_first_index(n)
    if first_index == -1:
        return -1
    i = first_index
    equal = True
    while equal and i < len(rules):
        if rules[i][0] == n:
            # if the element that must be after the current element has been already found, then it doesn't respect the rule
            if len(elements_already_found) != 0 and elements_already_found.count(rules[i][1]) > 0:
                return rules[i][1]
            i += 1
        else:
            equal = False
    return -1
rules = []

for line in file_to_read:
    if line == "\n":
        break
    content = line.strip("\n").split('|')
    new_content = [int(content[0]), int(content[1])]
    rules.append(new_content)

updates = []

for line in file_to_read:
    content = line.strip("\n").split(',')
    new_content = []
    for elem in content:
        new_content.append(int(elem))
    updates.append(new_content)

file_to_read.close()

rules.sort()

updates_ok = []
updates_ko = []
ok = True

for update in updates:
    ok = True
    elements_already_found = []
    for elem in update:
        if find_previous_element(elem, elements_already_found) != -1:
            ok = False
            break
        elements_already_found.append(elem)
    if ok:
        updates_ok.append(update)
    else:
        updates_ko.append(update)

sum = 0
for update in updates_ok:
    middle_element = update[int(len(update) / 2)]
    sum += middle_element
print(sum)
