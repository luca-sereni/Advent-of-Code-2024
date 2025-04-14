class TreeNode:
    def __init__(self, data):
        self.data = data
        self.children = []
    def add_child(self, child):
        self.children.append(child)

def build_tree(tree: TreeNode, numbers: list):
    if len(numbers) == 0:
        return
    
    # Left child contains the sum
    sum_node = TreeNode(tree.data + numbers[0])
    tree.add_child(sum_node)
    build_tree(sum_node, numbers[1:])

    # Right child contains the product
    mul_node = TreeNode(tree.data * numbers[0])
    tree.add_child(mul_node)
    build_tree(mul_node, numbers[1:])

def build_tree_v2(tree: TreeNode, numbers: list):
    if len(numbers) == 0:
        return
    
    # Left child contains the sum
    sum_node = TreeNode(tree.data + numbers[0])
    tree.add_child(sum_node)
    build_tree_v2(sum_node, numbers[1:])

    # Middle child contains the concatenation
    concat_node = TreeNode(int(str(tree.data) + str(numbers[0])))
    tree.add_child(concat_node)
    build_tree_v2(concat_node, numbers[1:])

    # Right child contains the product
    mul_node = TreeNode(tree.data * numbers[0])
    tree.add_child(mul_node)
    build_tree_v2(mul_node, numbers[1:])

def take_leaf_values(tree: TreeNode) -> list:
    if tree.children == []:
        return [tree.data]
    values = []
    for child in tree.children:
        values.extend(take_leaf_values(child))
    return values

def is_valid(result: int, tree: TreeNode) -> bool:
    leaves = take_leaf_values(tree)
    if result in leaves:
        return True
    return False

def print_graph(tree: TreeNode):
    print(tree.data)
    for child in tree.children:
        print_graph(child)

# 1st part
file_to_read = open("day7/input.txt", "r")

# Read the file
lines = file_to_read.readlines()
file_to_read.close()
equations = []

# Load equations with results and numbers
for i, line in enumerate(lines):
    # Remove the newline character
    line = line.strip()
    # Split the line into parts
    parts = line.split(': ')
    # Get the first part (the rule)
    result = int(parts[0])
    numbers = parts[1].split(' ')
    numbers_int = [int(num) for num in numbers]
    equations.append({result: numbers_int})

sum = 0
wrong_equations = []
for equation in equations:
    for key, values in equation.items():
        tree = TreeNode(values[0])
        build_tree(tree, values[1:])
        if is_valid(key, tree):
            sum += key
        elif equation not in wrong_equations:
            wrong_equations.append(equation)

print(sum)

# 2nd part
sum_v2 = 0
for equation in wrong_equations:
    for key, values in equation.items():
        tree = TreeNode(values[0])
        build_tree_v2(tree, values[1:])
        if is_valid(key, tree):
            sum_v2 += key

print (sum + sum_v2)