import re, time

def gcd(a, b):
    while b:
        a, b  =  b, a % b
    return a

def walk_through_map(lr_seq, map_dict, start):
    current_loc = start
    lr_seq_len = len(lr_seq)
    i = 0
    # In my input file, this condition works fine for Part 1.  For some reason
    # 'AAA' does not go to any location ending with 'Z' other than 'ZZZ'.  This
    # may not be the same for you.  If that's so, just duplicate this function,
    # name it something else, and then change the condition to
    #    `current_loc != 'ZZZ'`
    while current_loc[2] != 'Z': 
        current_loc = map_dict[current_loc][lr_seq[i % lr_seq_len] == 'R']
        i += 1
    return i

def main():
    alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    answer = 1

    with open("input.txt") as f: 
        lns = [ln.strip() for ln in f.readlines()]
    lr_seq, maps = lns[0], lns[2:]
    maps = [list(filter(None, re.split(r'\s=\s\(|,\s|\)', x))) for x in maps]
    map_dict = {map[0]: (map[1], map[2]) for map in maps}
    
    for i, j in ((i, j) for i in range(26) for j in range(26)):
        start_loc = alphabet[i] + alphabet[j] + 'A'
        if start_loc in map_dict:
            num_steps = walk_through_map(lr_seq, map_dict, start_loc)
            answer = answer * num_steps // gcd(answer, num_steps)

    print("Part 1: {0}".format(walk_through_map(lr_seq, map_dict, 'AAA')))
    print("Part 2: {0}".format(answer))

if __name__ == "__main__":
    main()


with open("input.txt") as f: 
    lines_str = f.readlines()
    lines_no_space = [line.strip() for line in lines_str]

    leftright_seq = lines_no_space[0]
    maps_data_str = lines_no_space[2:]
    maps_data_parsed = [list(filter(None, re.split(r'\s=\s\(|,\s|\)', x))) for x in maps_data_str]
    maps_dict = {map[0]: (map[1], map[2]) for map in maps_data_parsed}