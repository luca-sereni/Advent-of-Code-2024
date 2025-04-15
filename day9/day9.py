def move_blocks(blocks: list):
    i = 0
    j = len(blocks) - 1
    while i < j:
        if blocks[i][0] == -1:
            while j > i:
                if blocks[j][0] != -1:
                    if blocks[j][1] > blocks[i][1]:
                        # Block to move > free space
                        free_space = blocks[i][1]
                        blocks[j] = (blocks[j][0], blocks[j][1] - free_space)
                        blocks.insert(j + 1, (-1, free_space))
                        blocks[i] = (blocks[j][0], free_space)
                        i += 1
                        break
                    elif blocks[j][1] == blocks[i][1]:
                        # Block to move == free space
                        free_space = blocks[i][1]
                        blocks[i] = (blocks[j][0], blocks[j][1])
                        blocks[j] = (-1, free_space)
                        j -= 1
                        i += 1
                        break
                    else:
                        # Block to move < free space
                        temp_size = blocks[i][1]
                        dim_block = blocks[j][1]
                        blocks[i] = (blocks[j][0], dim_block)
                        blocks[j] = (-1, dim_block)
                        blocks.insert(i + 1, (-1, temp_size - blocks[j][1]))
                        i += 1
                        break
                else:
                    j -= 1
        else:
            i += 1

def compute_checksum(blocks: list) -> int:
    # Transform the blocks into a list of integers
    new_blocks = []
    for block in blocks:
        if block[0] != -1:
            i = 0
            while i < block[1]:
                new_blocks.append(block[0])
                i += 1
    # Compute the checksum
    checksum = 0
    for i in range(len(new_blocks)):
        if new_blocks[i] != -1:
            checksum += new_blocks[i] * i

    return checksum

file = open("day9/input.txt", "r")
content = file.read()
file.close()

blocks = []

index = 0
id_block = 0
while content[index] != '\n':
    if index % 2 == 0:
        blocks.append((id_block, int(content[index])))
        id_block += 1
    else:
        if int(content[index]) > 0:
            blocks.append((-1, int(content[index])))
    index += 1

# 1st part
move_blocks(blocks)
print(compute_checksum(blocks))