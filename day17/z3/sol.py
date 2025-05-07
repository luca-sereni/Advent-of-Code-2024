from z3 import *

# Create 16 symbolic octal digits (each 3 bits)
digits = [BitVec(f'd{i}', 3) for i in range(16)]

# Constrain digits to be valid octal digits: 0–7
s = Optimize()

# Build the 48-bit value 'a' by packing the digits
a = Sum([ZeroExt(61, digits[i]) << (3 * i) for i in range(16)])

# Initialize b and c
b = BitVecVal(0, 64)
c = BitVecVal(0, 64)

output = [BitVec(f'output_{i}', 64) for i in range(16)]
temp_a = a

for i in range(16):
    b = URem(temp_a, 8)
    b = b ^ 5
    c = LShR(temp_a, b)
    temp_a = UDiv(temp_a, 8)
    b = b ^ 6
    b = b ^ c
    output[i] = URem(b, 8)

# Program: 2,4,1,5,7,5,0,3,1,6,4,3,5,5,3,0
s.add(output[0] == 2)
s.add(output[1] == 4)
s.add(output[2] == 1)
s.add(output[3] == 5)
s.add(output[4] == 7)
s.add(output[5] == 5)
s.add(output[6] == 0)
s.add(output[7] == 3)
s.add(output[8] == 1)
s.add(output[9] == 6)
s.add(output[10] == 4)
s.add(output[11] == 3)
s.add(output[12] == 5)
s.add(output[13] == 5)
s.add(output[14] == 3)
s.add(output[15] == 0)

s.minimize(a)  # We need to take the minimum value of a

if s.check() == sat:
    model = s.model()
    a_val = model.evaluate(a).as_long()
    found_digits = [model[d].as_long() for d in digits]
    print("Minimal a found:", a_val)
else:
    print("No solution found")