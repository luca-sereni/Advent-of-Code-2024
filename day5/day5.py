from collections import defaultdict, deque

# Function to check if an update is valid
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

def compute_sum(updates_list: list) -> int:
    total_sum = 0
    for update in updates_list:
        middle_element = update[int(len(update) / 2)]
        total_sum += middle_element
    return total_sum

# Build the graph used for topological sorting
def build_graph(rules: dict) -> dict:
    graph = defaultdict(list)
    for src, targets in rules.items():
        for dst in targets:
            graph[src].append(dst)
    return graph

def fix_update(rules_dict, sequence):
    indegree = defaultdict(int)   # Contains the number of incoming edges for each node
    all_nodes = set()             # Unique nodes

    graph = build_graph(rules_dict)
    for src, targets in graph.items():
        all_nodes.add(src)
        for dst in targets:
            indegree[dst] += 1
            all_nodes.add(dst)

    queue = deque([node for node in all_nodes if indegree[node] == 0])
    topo_order = []

    # Topological sort
    while queue:
        current = queue.popleft()
        topo_order.append(current)
        for neighbor in graph[current]:
            indegree[neighbor] -= 1
            if indegree[neighbor] == 0:
                queue.append(neighbor)

    rank = {value: index for index, value in enumerate(topo_order)}

    sorted_sequence = sorted(sequence, key=lambda x: rank.get(x, float('inf')))

    return sorted_sequence

# Open file
file_to_read = open("day5/input.txt", "r")

# Add rules
rules = {}
for line in file_to_read:
    if line == "\n":
        break
    content = line.strip("\n").split('|')
    key = int(content[0])
    if key not in rules:
        rules[key] = []
    rules[key].append(int(content[1]))

# Add updates
updates = []
for line in file_to_read:
    content = line.strip("\n").split(',')
    new_content = []
    for elem in content:
        new_content.append(int(elem))
    updates.append(new_content)

file_to_read.close()

# 1st Part
updates_ok = []
updates_ko = []

for update in updates:
    if is_update_valid(update):
        updates_ok.append(update)
    else:
        updates_ko.append(update)

print(compute_sum(updates_ok))

# 2nd Part
fixed_updates = []

for update in updates_ko:
    #Take the rules that are related to the update
    filtered_rules = {
        k: [x for x in v if x in update]
        for k, v in rules.items()
    }
    new_update = fix_update(filtered_rules, update)
    fixed_updates.append(new_update)

for update in fixed_updates:
    if not is_update_valid(update):
        print(update)

print(compute_sum(fixed_updates))
