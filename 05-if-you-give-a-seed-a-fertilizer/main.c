#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define MAX_LINE_CHARS                     500
#define MAX_MAPS                             7
#define MAX_MAP_ITEMS                      100
#define SOME_LARGE_NUMBER             90168667

int num_is_a_seed(const unsigned int arr[], const int size, 
    const unsigned int x);
unsigned int forward_step(const unsigned int dest[], const unsigned int src[],
    const unsigned int length[], const int map_item_count, unsigned int x);
unsigned int reverse_step(const unsigned int dest[],
    const unsigned int src[], const unsigned int length[],
    const int map_item_count, unsigned int x);
int process_file(FILE *file);
        

int
main(int argc, char *argv[])
{
        FILE *file = argc > 1 ? fopen(argv[1], "r") : fopen("input.txt", "r");

        if (!file) {
                puts("Unable to open the file.\n");
                return -1;
        }
        
        const int err_code = process_file(file);

        fclose(file);

        return err_code;
}


int
process_file(FILE *file)
{
        /* Parsing */
        char                 seeds_line[MAX_LINE_CHARS];
        char                 current_line[MAX_LINE_CHARS];
        char                 *p_endpoint;
        bool                 is_anticipating_numbers = false;
        
        /* Calculation */
        unsigned int         seeds[36];
        unsigned int         seed_count = 0;
        unsigned int         dest[MAX_MAPS][MAX_MAP_ITEMS];
        unsigned int         src[MAX_MAPS][MAX_MAP_ITEMS];
        unsigned int         length[MAX_MAPS][MAX_MAP_ITEMS];
        unsigned int         map_item_count[MAX_MAPS];
        unsigned int         map_count = 0;
        unsigned int         minloc = UINT_MAX;

        /*
         * Parsing the seeds
         */
        fgets(seeds_line, sizeof(seeds_line), file);

        char *token = strtok(seeds_line, "sed: ");

        while (token != NULL) {
                seeds[seed_count] = 
                        (unsigned int)strtoul(token, &p_endpoint, 10);
                seed_count++;
                token = strtok(NULL, " ");
        }

        /*
         * Parsing the maps
         */
        while (fgets(current_line, sizeof(current_line), file) != NULL) {
                /* If the line is blank */
                if (current_line[0] == '\n') {
                        if (is_anticipating_numbers){
                                is_anticipating_numbers = false;
                                map_count += 1;
                        }
                        continue;
                }

                /* If the line is a map label, e.g. "seed-to-soil map:" */
                if (strstr(current_line, "map:")) {
                        is_anticipating_numbers = true;
                        continue;
                }

                /* Processing the map numbers, e.g. "0 15 37" */
                char *token = strtok(current_line, " ");

                /* This WILL cause errors if there aren't three numbers in a
                 * non-seed line */
                while (token != NULL) {
                        dest[map_count][map_item_count[map_count]] =
                            strtoul(token, &p_endpoint, 10);
                        token = strtok(NULL, " ");
                        src[map_count][map_item_count[map_count]] =
                            strtoul(token, &p_endpoint, 10);
                        token = strtok(NULL, " ");
                        length[map_count][map_item_count[map_count]] =
                            strtoul(token, &p_endpoint, 10);
                        token = strtok(NULL, " ");
                        map_item_count[map_count] =
                            map_item_count[map_count] + 1;
                }
        }

        /*
         * Moving seeds through the pipeline Part ONE: Forward pass
         */
        for (unsigned int i = 0; i < seed_count; i++) {
                unsigned int seed = seeds[i];
                for (unsigned int j = 0; j < map_count; j++) {
                        seed = forward_step(dest[j], src[j], length[j],
                            map_item_count[j], seed);
                }
                if (seed < minloc) {
                        minloc = seed;
                }
        }

        printf("PART ONE: %u\n", minloc);

        /*
         * Deduce seed from generated locations for Part TWO - Reverse pass
         */
        minloc = UINT_MAX;
        for (unsigned int candidate_loc = 0; candidate_loc < SOME_LARGE_NUMBER;
             candidate_loc++) {
                unsigned int result  = candidate_loc;
                for (int j = map_count - 1; j >= 0; j--) {
                        result  = reverse_step(dest[j], src[j], length[j],
                            map_item_count[j], result );
                }

                if (num_is_a_seed(seeds, seed_count, result ) == 1) {
                        minloc = candidate_loc;
                        break;
                }
        }

        printf("PART TWO: %u\n", minloc);

        return 0;
}

unsigned int
forward_step(const unsigned int dest[], const unsigned int src[],
    const unsigned int length[], const int map_item_count, unsigned int x)
{
        for (int i = 0; i < map_item_count; i++) {
                if (x >= src[i] && x < src[i] + length[i]) {
                        return dest[i] - src[i] + x;
                }
        }
        return x;
}

unsigned int
reverse_step(const unsigned int dest[], const unsigned int src[],
    const unsigned int length[], const int map_item_count, unsigned int x)
{
        for (int i = 0; i < map_item_count; i++) {
                if (x >= dest[i] && x < dest[i] + length[i]) {
                        return x - dest[i] + src[i];
                }
        }
        return x;
}

int
num_is_a_seed(const unsigned int arr[], const int size, const unsigned int x)
{
        for (int i = 0; i < size; i = i + 2) {
                if (x >= arr[i] && x < arr[i] + arr[i + 1]) {
                        return 1;
                }
        }

        return 0;
}

