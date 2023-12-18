def get_hash(raw):
    hash = 0
    for c in raw:
        hash = ((hash + ord(c))* 17) % 256
    return hash

with open("input.txt") as f:
    seq = f.read().split(',')

sum = 0
for step in seq:
    sum += get_hash(step)

print(sum)