def gardenmap(dests, sources, lengths, x):
	for source, length, dest in zip(sources, lengths, dests):
		if (x >= source) and (x < source + length):
			return dest - source + x
	return x

source_arrays = []
dest_arrays = []
length_arrays = []

with open("input-full.txt", 'r') as f:
	lines = [line for line in f.read().splitlines() if line]
	
seeds = [int(x) for x in lines[0].split(':')[-1].split()]
print(seeds)

temp_sources = []
temp_dests = []
temp_lengths = []
for line in lines[2:]:
	if ":" in line:
		source_arrays.append(temp_sources)
		dest_arrays.append(temp_dests)
		length_arrays.append(temp_lengths)
		temp_sources = []
		temp_dests = []
		temp_lengths = []
	else:
		dest, source, length = [int(x) for x in line.split()]
		temp_sources.append(source)
		temp_dests.append(dest)
		temp_lengths.append(length)
source_arrays.append(temp_sources)
dest_arrays.append(temp_dests)
length_arrays.append(temp_lengths)

# print(dest_arrays, "\n")
# print(source_arrays, "\n")
# print(length_arrays, "\n")

locations = []
for seed in seeds:
	for dests, sources, lengths in zip(dest_arrays, source_arrays, length_arrays):
		seed = gardenmap(dests, sources, lengths, seed)
	locations.append(seed)

print(min(locations))

locations = []

it = iter(seeds)
seeds_srclenpairs = [x for x in zip(it, it)]
for seed_src, seed_len in seeds_srclenpairs:
	print("Calculating", seed_src, seed_len)
	seed_len_i = 0
	while seed_len_i < seed_len:
		seed = seed_src + seed_len_i
		for dests, sources, lengths in zip(dest_arrays, source_arrays, length_arrays):
			seed = gardenmap(dests, sources, lengths, seed)
		locations.append(seed)
		print("         SEED:", seed_src + seed_len_i, "==>", seed)
		seed_len_i += 1
		
		
print(locations)
print(min(locations))
	
