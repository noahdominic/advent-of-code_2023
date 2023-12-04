# with open("input-small.txt", newline='') as f:
#   parse_score = lambda x: len(set(x[0].split()) & set(x[1].split()))
#   scores = [[parse_score(ln.split(":")[1].split("|")), 1] for ln in f.readlines()]
#   for i, score in enumerate(scores):
#     for j in range(score[0], 0, -1):
#       scores[i + j][1] += score[1]
#   print(sum(1 << arr[0] >> 1 for arr in scores), sum(arr[1] for arr in scores))



with open("input-small.txt", newline='') as f: lns = f.readlines()
find_wins = lambda x: len(set(x[0].split()) & set(x[1].split()))
nfo = list(map(lambda ln: [find_wins(ln.split(":")[-1].split("|")), 1], lns))
[exec('nfo[i+j][1]+=x[1]') for i,x in enumerate(nfo) for j in range(x[0],0,-1)]
print(sum(1 << x[0] >> 1 for x in nfo), sum(x[1] for x in nfo))



