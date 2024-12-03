file_read = open("/home/luca/Downloads/input.txt", "r")

column1 = []
column2 = []

for line in file_read:
    column1.append(line.split()[0])
    column2.append(line.split()[1])

file_read.close()
""" FIRST PART
column1.sort()
column2.sort()

sum = 0
for i in range(len(column1)):
    val = abs(int(column1[i]) - int(column2[i]))
    sum += val

print(sum)"""

column2.sort()

similarity = 0
found = False
for i in range(len(column1)):
    num_elements = 0
    found = False
    for j in range(len(column1)):
        if column1[i] != column2[j] and found == False:
            continue
        elif column1[i] == column2[j]:
            found = True
            num_elements += 1
        else:
            similarity += int(column1[i]) * num_elements
            break

print(similarity)
