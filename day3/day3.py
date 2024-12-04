import re

file_to_read = open("day3/input.txt", "r")

reg_expr_mul = r"mul\(\d{1,3},\d{1,3}\)"

"""FIRST PART
strings_ok = []
for match in re.finditer(reg_expr_mul, file_to_read.read()):
    strings_ok.append(match.group())

result = 0

for string in strings_ok:
    numbers = re.findall(r"\d{1,3}", string)
    result += int(numbers[0]) * int(numbers[1])

print(result)
"""

reg_expr_do = r"do\(\)"
reg_expr_dont = r"don\'t\(\)"

pattern = reg_expr_mul + "|" + reg_expr_do + "|" + reg_expr_dont

mul = []
to_consider = True
for match in re.finditer(pattern, file_to_read.read()):
    if match.group() == "do()":
        to_consider = True
    elif match.group() == "don't()":
        to_consider = False
    elif to_consider:
        mul.append((match.group()))

print(mul)
result = 0

for match in mul:
    numbers = re.findall(r"\d{1,3}", match)
    print(numbers)
    result += int(numbers[0]) * int(numbers[1])

print(result)