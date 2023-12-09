def get_next(sequence):
    sum = 0
    while not all(x == 0 for x in sequence):
        sum += sequence[-1]
        sequence = [sequence[i + 1] - sequence[i] for i in range(len(sequence) - 1)]
    return sum

with open("input.txt") as f:
    lines = [[int(num) for num in ln.split()] for ln in f.readlines()]
    
print(sum(get_next(sequence) for sequence in lines))
print(sum(get_next(sequence[::-1]) for sequence in lines))