import re, time

LEFT=0
RIGHT=1

def gcd(a, b):
    while b:
        a, b  =  b, a % b
    return a

def walk_through_map(lr_seq, map_arr, start):
    current_loc = start
    lr_seq_len = len(lr_seq)
    i = lr_seq_i = 0
    
    # In my input file, this condition works fine for Part 1.  For some reason
    # 'AAA' does not go to any location ending with 'Z' other than 'ZZZ'.  This
    # may not be the same for you.  If that's so, just duplicate this function,
    # name it something else, and then change the condition to
    #    `current_loc != 'ZZZ'`
    while current_loc[2] != 'Z': 
        current_loc = map_arr[current_loc][LEFT if lr_seq[lr_seq_i] == 'L' else RIGHT]
        i += 1
        lr_seq_i = (lr_seq_i + 1) % lr_seq_len

    return i

def main():
    alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    answer = 1

    with open("input.txt") as f: 
        lns = [ln.strip() for ln in f.readlines()]
    lr_seq, maps = lns[0], lns[2:]
    maps = [list(filter(None, re.split(r'\s=\s\(|,\s|\)', x))) for x in maps]
    map_dict = {map[0]: (map[1], map[2]) for map in maps}
    a = time.time()
    for i, j in ((i, j) for i in range(26) for j in range(26)):
        start_loc = alphabet[i] + alphabet[j] + 'A'
        if start_loc in map_dict:
            num_steps = walk_through_map(lr_seq, map_dict, start_loc)
            answer = answer * num_steps // gcd(answer, num_steps)
    b = time.time()
    c = time.time()
    print("Part 1: {0}".format(walk_through_map(lr_seq, map_dict, 'AAA')))
    d = time.time()

    print("Part 2: {0}".format(answer))
    print(d - c)
    print(b - a)
    return

if __name__ == "__main__":
    main()