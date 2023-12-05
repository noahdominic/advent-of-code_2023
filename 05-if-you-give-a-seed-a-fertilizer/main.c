#include <stdio.h>
#include <string.h>
#include <stdlib.h>

unsigned long garden_map(const unsigned long dest[], const unsigned long src[], const unsigned long length[], const int map_entry_count, unsigned long x);
int process_file(FILE *file);

int main(int argc, char *argv[]) {
	FILE *file;
	
	if (argc > 1) {
		file = fopen(argv[1], "r");
	} else {
		file = fopen("input.txt", "r");
	}
	
	if (file == NULL) {
		puts("Unable to open the file.\n");
		return -1;
	}
	
	const int err_value = process_file(file);
	
	fclose(file);
	return err_value;
}


int process_file(FILE *file) {
	char seeds_line[300];
	char current_line[300];
	unsigned long seeds[36], seed_count = 0;
	unsigned long dest[7][100];
	unsigned long src[7][100];
	unsigned long length[7][100];
	int map_entry_count[7];
	int map_count = -1;
	char *p_endpoint;
	
	/*
	 * Parsing the seeds 
	 */
	fgets(seeds_line, sizeof(seeds_line), file);
	
	char *token = strtok(seeds_line, "sed: ");
	
	while (token != NULL) {
		seeds[seed_count] = strtoul(token, &p_endpoint, 10);
		seed_count++;
		token = strtok(NULL, " ");
	}
	
	/* 
	 * Parsing the maps
	 */
	for (int i = 0; fgets(current_line, sizeof(current_line), file) != NULL; i++) {
		/* If the line is blank */
		if (current_line[0] == '\n') {
			continue;
		}
		
		/* If the line is a map label, e.g. "seed-to-soil map:" */
		if (strstr(current_line, "map:")) {
			map_count++;
			continue;
		}
		
		/* Processing the map numbers, e.g. "0 15 37" */
		char *token = strtok(current_line, " ");
		
		/* This WILL cause errors if there aren't three numbers in a non-seed line */
		while (token != NULL) {
			dest[map_count][map_entry_count[map_count]] = strtoul(token, &p_endpoint, 10);
			token = strtok(NULL, " ");
			src[map_count][map_entry_count[map_count]] = strtoul(token, &p_endpoint, 10);
			token = strtok(NULL, " ");
			length[map_count][map_entry_count[map_count]] = strtoul(token, &p_endpoint, 10);
			token = strtok(NULL, " ");
			map_entry_count[map_count] = map_entry_count[map_count] + 1;
		}
	}
	
	for (int i = 0; i < seed_count; i++) {
		printf("SEED %lu\n", seeds[i]);
	}
	
	for (int i = 0; i < map_count + 1; i++) {
		printf("\nMAP %i\n", i);
		for (int j = 0; j < map_entry_count[i]; j++) {
			printf("%lu %lu %lu\n", dest[i][j], src[i][j], length[i][j]);
		}
	}
	
	unsigned long a = garden_map(dest[0], src[0], length[0], map_entry_count[0], 79);
	
	printf("79 => %lu\n", garden_map(dest[0], src[0], length[0], map_entry_count[0], 79));
	printf("14 => %lu\n", garden_map(dest[0], src[0], length[0], map_entry_count[0], 14));
	printf("55 => %lu\n", garden_map(dest[0], src[0], length[0], map_entry_count[0], 55));
	printf("13 => %lu\n", garden_map(dest[0], src[0], length[0], map_entry_count[0], 13));
	return 0;
}

unsigned long garden_map(const unsigned long dest[], const unsigned long src[], const unsigned long length[], const int map_entry_count, unsigned long x) {
	for (int i = 0; i < map_entry_count; i++) {
		if (x >= src[i] && x < src[i] + length[i]){
			return dest[i] - src[i] + x;
		}
	}
	return x;
}


