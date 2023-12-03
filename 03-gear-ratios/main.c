#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_COLS                                                               \
  142 // Input file has 140 characters
      // + 1 for UNIX newline
      // and + 1 for NULL

#define MAX_ROWS 140 // Input file has 140 lines

int max(int num1, int num2) { return (num1 > num2) ? num1 : num2; }

int min(int num1, int num2) { return (num1 < num2) ? num1 : num2; }

bool is_digit(const char c) { return c >= '0' && c <= '9'; }

bool is_cntrl(const char c) { return c >= 0 && c <= 31; }

// For the purposes of this problem (Part 1), I will be considering all
// non-digit and non-control characters to be 'special'.
bool is_special(const char c) {
  return !is_digit(c) && !is_cntrl(c) && c != '.';
}

int main() {
  FILE *file;

  file = fopen("input-small.txt", "r");

  char line_buffer[MAX_COLS];

  char file_contents_container[MAX_ROWS][MAX_COLS];

  int row_count = 0;

  while (fgets(line_buffer, sizeof(line_buffer), file) != NULL) {
    strcpy(file_contents_container[row_count], line_buffer);
    row_count++;
  }

  fclose(file);

  int total_num = 0;
  char num_buffer[4]; // Supports up to 32-digit nums
  int num_buffer_index = 0;

  for (int i = 0; i < row_count; i++) {
    printf("> %s", file_contents_container[i]);

    for (int j = 0; file_contents_container[i][j] != '\0'; j++) {
      if (is_digit(file_contents_container[i][j])) {
        bool is_valid = false;
        int num_col_idx_l, num_col_idx_r;

        // Number detected. Find the bounds of the number and store the number.
        num_col_idx_l = j;
        while (is_digit(file_contents_container[i][j])) {
          num_buffer[num_buffer_index] = file_contents_container[i][j];
          num_col_idx_r =
              j; // This will be continuously updated until condition is false.
                 // Last value will be index of last digit.
          num_buffer_index++;
          j++;
        }

        // Check if number is valid.

        // Cached for readability
        int search_area_row_start_idx = max(i - 1, 0);
        int search_area_col_start_idx = max(num_col_idx_l - 1, 0);
        int search_area_row_end_idx = min(i + 1, MAX_ROWS);
        int search_area_col_end_idx = min(num_col_idx_r + 1, MAX_COLS);
        // printf("Searching from [%d][%d] to [%d][%d]\n",
        //        search_area_row_start_idx, search_area_col_start_idx,
        //        search_area_row_end_idx, search_area_col_end_idx);
        for (int row_idx = search_area_row_start_idx;
             row_idx <= search_area_row_end_idx; row_idx++) {
          for (int col_idx = search_area_col_start_idx;
               col_idx <= search_area_col_end_idx; col_idx++) {
            // printf("Assessing %s. [%d][%d] = %c ...\n", num_buffer, row_idx,
            //        col_idx, file_contents_container[row_idx][col_idx]);
            if (is_special(file_contents_container[row_idx][col_idx])) {
              // If a special character is found, STOP the search and proceed
              // to str->int conversion.
              printf("%s is valid!\n", num_buffer);
              is_valid = true;
              break;
            }
          }
          if (is_valid)
            break;
        }

        // If `is_valid` is still false, skip conversion.
        if (is_valid) {
          // printf("%d is valid. Added to total total_num (%d) to form (%d)\n",
          //        atoi(num_buffer), total_num, total_num + atoi(num_buffer));
          total_num += atoi(num_buffer);
        } else {
          printf("%s is not valid!\n", num_buffer);
        }

        num_buffer_index = 0;
        memset(num_buffer, '\0', sizeof(num_buffer));
      }
    }
  }

  printf("Total num is %d\n", total_num);

  return 0;
}