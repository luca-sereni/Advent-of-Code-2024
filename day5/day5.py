file_to_read = open("day5/input.txt", "r")

def is_update_valid(update: list) -> bool:
    elements_checked = []
    for elem in update:
        if len(elements_checked) == 0 or elem not in rules:
            elements_checked.append(elem)
            continue
        for checked_elem in elements_checked:
            if checked_elem in rules[elem]:
                return False
        elements_checked.append(elem)
    return True
    
def fix_update(update: list):
    temp_update = update
    fixed = False
    while not fixed:
        elements_checked = {}
        ok = True
        for i, elem in enumerate(temp_update):
            if len(elements_checked) == 0 or elem not in rules:
                elements_checked[elem] = i
                continue
            for elem_checked_key, elem_checked_index in elements_checked.items():
                if elem_checked_key in rules[elem]:
                    #Switch side
                    temp_update.pop(elem_checked_index)
                    temp_update.insert(i, elem_checked_key)
                    ok = False
                    break
            if not ok:
                break
        if ok:
            fixed = True
    fixed_updates.append(temp_update)
    


#Add rules
rules = {}
for line in file_to_read:
    if line == "\n":
        break
    content = line.strip("\n").split('|')
    key = int(content[0])
    if key not in rules:
        rules[key] = []
    #new_content = [int(content[0]), int(content[1])]
    rules[key].append(int(content[1]))

#Add updates
updates = []
for line in file_to_read:
    content = line.strip("\n").split(',')
    new_content = []
    for elem in content:
        new_content.append(int(elem))
    updates.append(new_content)

file_to_read.close()

updates_ok = []
updates_ko = []

for update in updates:
    if is_update_valid(update):
        updates_ok.append(update)
    else:
        updates_ko.append(update)

# 1st Part
sum = 0
for update in updates_ok:
    middle_element = update[int(len(update) / 2)]
    sum += middle_element
print(sum)

# 2nd Part
fixed_updates = []

for update in updates_ko:
    fix_update(update)

for update in fixed_updates:
    if not is_update_valid(update):
            print(update)

sum = 0
for update in fixed_updates:
    middle_element = update[int(len(update) / 2)]
    sum += middle_element
print(sum)
