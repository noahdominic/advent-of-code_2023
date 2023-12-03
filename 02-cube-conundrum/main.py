import csv

# The input was modified via Find and Replace w/ regex to remove the "Game N: " labels

file_path = "input-modified.txt"

# Part 1

total = 0
limits = {"red": 12, "green": 13, "blue": 14}

with open(file_path, newline='') as csvfile:
    
    # I literally don't have to do this.  The semicolon is lowkey useless.
    csv_reader = csv.reader(csvfile, delimiter=';')
    
    for i, row in enumerate(csv_reader):
        err_found = False

        for round in row:
            ball_counts = round.split(',')

            for ball_count in ball_counts:
                ball_count = ball_count.strip().split(' ')

                number = int(ball_count[0])
                colour_limit = limits[ball_count[1]]

                if number > colour_limit:
                    err_found = True
                    break
            
            if err_found:
                break

        if err_found:
            continue
        
        total += i + 1

    print(f"Part 1 Total: {total}")


# Part 2

total = 0

with open(file_path, newline="") as f:
    rows = f.read().splitlines()
    for row in rows:
        counts = {"red": [], "green": [], "blue": []}

        for ball_count in row.replace("; ", ", ").split(", "):
            ball_count = ball_count.split(" ")
            counts[ball_count[1]].append(int(ball_count[0]))

        total += max(counts["red"]) * max(counts["green"]) * max(counts["blue"])

    print(f"Part 2 Total: {total}")