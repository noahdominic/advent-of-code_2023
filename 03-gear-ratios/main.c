#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Author's Note:  For some reason which I've never fully investigated yet,
 * there is a problem with this program when I compiled it with GCC.
 * Code in Part 2, which supposedly are in different scopes, affect variables in Part 1.
 * I used clang to compile this, which did not cause problems.
 */

/* Input file has 140 characters + 1 for UNIX newline and + 1 for NULL */
#define MAX_COLS 142 

/* Input file has 140 rows  */
#define MAX_ROWS 140 

/* I'm using '^' just because my input file has no '^'.
   This may be different for someone else. */
#define GEAR_CHAR '^'

static int max(const int num1, const int num2);
static int min(const int num1, const int num2);
static int is_digit(const char c);
static int is_ctrl(const char c);
static int is_special(const char c);

int main() {
  FILE *file;
  file = fopen("input.txt", "r");

  char line_buffer[MAX_COLS];
  char file_contents_container[MAX_ROWS][MAX_COLS];
  int row_count = 0;

  while (fgets(line_buffer, sizeof(line_buffer), file) != NULL) {
    strcpy(file_contents_container[row_count], line_buffer);
    row_count++;
  }

  fclose(file);

  /* PART ONE.
   *
   * The solution is as follows:
   *  - Read the character grid line by line.
   *  - When you encounter a digit, note the current index as the 'left' index,
   *      then store the digit into a number buffer,
   *      the move into the next.
   *  - Stop when you've stopped encountering a digit.  Note
   *      the index of the last digit as the 'right' index.
   *  - Now scan the adjacent cells from 'left' to 'right' indices.
   *  - If you encounter a 'special' character, set the flag as valid
   *       and immediately stop the scan.
   *  - If the valid flag is true, convert the number in the number buffer
   *       to an integer and add it to the total.
   *  - Continue until all lines are scanned.
   *  - The total should contain the sum of all 'valid' numbers.
   */

  int total_num = 0;
  char num_buffer[8];
  int num_buffer_index = 0;

  for (int i = 0; i < row_count; i++) {
    for (int j = 0; file_contents_container[i][j] != '\0'; j++) {
      if (is_digit(file_contents_container[i][j])) {
        int num_col_idx_l, num_col_idx_r, is_valid;
        num_col_idx_l = j;
        is_valid = 0;

        /* Find the bounds of the number and store the number into a buffer. */
        while (is_digit(file_contents_container[i][j])) {
          num_buffer[num_buffer_index] = file_contents_container[i][j];
          num_buffer_index++;
          j++;
        }
        num_col_idx_r = j - 1;

        int srch_row_start_idx = max(i - 1, 0);
        int srch_col_start_idx = max(num_col_idx_l - 1, 0);
        int srch_row_end_idx = min(i + 1, row_count);
        int srch_col_end_idx = min(num_col_idx_r + 1, MAX_COLS);

        for (int row_idx = srch_row_start_idx; row_idx <= srch_row_end_idx; row_idx++) {
          for (int col_idx = srch_col_start_idx; col_idx <= srch_col_end_idx; col_idx++) {
            if (is_special(file_contents_container[row_idx][col_idx])) {
              is_valid = 1;
              break;
            }
          }
          if (is_valid)
            break;
        }

        if (is_valid) {
          total_num += atoi(num_buffer);
        }

        /* Reset the buffer */
        num_buffer_index = 0;
        memset(num_buffer, '\0', sizeof(num_buffer));
      }
    }
  }

  printf("Part One total num is %d\n", total_num);

  /* PART TWO
   *
   * My solution is as follows:
   * - Find all the 'gear' asterisks and replace it with a special character (See #defines above).
   * - Create a grid of ints with similar dimensions to the input file.
   *     Important: Set the default value to 1.
   * - Similar to part one, loop through the input and detect numbers.
   * - Scan the adjacent cells of the number and find the special character.
   * - If it exists, multiply the value of the number to the value of the
   *      int in the int grid with the same coordinates as the special character.
   * - Finally, calculate the sum of all the ints in the int grid with the same coordinates as the special character.
   * - Display the final total.
   * (This is the most ridiculous algorithm I've written, but I was desperate.)
   */

  int values[MAX_ROWS][MAX_COLS];

  /* Init all values to 1.  Side note: Is this really the only way to do this? */
  for (int i = 0; i < row_count; i++) {
    for (int j = 0; file_contents_container[i][j] != '\0'; j++) {
      values[i][j] = 1;
    }
  }

  /* Find all 'gear' asterisks and replace them. */
  for (int i = 0; i < row_count; i++) {
    for (int j = 0; file_contents_container[i][j] != '\0'; j++) {
      if (file_contents_container[i][j] == '*') {
        int amount_of_adjacent_nums = 0;
        int waiting_for_num = 1;
        int srch_row_start_idx = max(i - 1, 0);
        int srch_col_start_idx = max(j - 1, 0);
        int srch_row_end_idx = min(i + 1, row_count);
        int srch_col_end_idx = min(j + 1, MAX_COLS);

        for (int row_idx = srch_row_start_idx; row_idx <= srch_row_end_idx; row_idx++) {
          for (int col_idx = srch_col_start_idx; col_idx <= srch_col_end_idx; col_idx++) {
            if (is_digit(file_contents_container[row_idx][col_idx])) {
              if (waiting_for_num) {
                waiting_for_num = 0;
                amount_of_adjacent_nums++;
              }
            } else {
              waiting_for_num = 1;
            }
          }
          waiting_for_num = 1;
        }
        if (amount_of_adjacent_nums > 1) {
          file_contents_container[i][j] = GEAR_CHAR;
        }
      }
    }
  }

  /* Calculate the 'gear ratios' of all the gears. */
  for (int i = 0; i < row_count; i++) {
    for (int j = 0; file_contents_container[i][j] != '\0'; j++) {
      if (is_digit(file_contents_container[i][j])) {
        int num_col_idx_l, num_col_idx_r;

        num_col_idx_l = j;
        while (is_digit(file_contents_container[i][j])) {
          num_buffer[num_buffer_index] = file_contents_container[i][j];
          num_buffer_index++;
          j++;
        }
        num_col_idx_r = j - 1;

        int srch_row_start_idx = max(i - 1, 0);
        int srch_col_start_idx = max(num_col_idx_l - 1, 0);
        int srch_row_end_idx = min(i + 1, row_count);
        int srch_col_end_idx = min(num_col_idx_r + 1, MAX_COLS);

        for (int row_idx = srch_row_start_idx; row_idx <= srch_row_end_idx; row_idx++) {
          for (int col_idx = srch_col_start_idx; col_idx <= srch_col_end_idx; col_idx++) {
            if (file_contents_container[row_idx][col_idx] == GEAR_CHAR) {
              values[row_idx][col_idx] *= atoi(num_buffer);
            }
          }
        }
        num_buffer_index = 0;
        memset(num_buffer, '\0', sizeof(num_buffer));
      }
    }
  }

  /* Calculate the sum of all ints in the int grid with the same coordinates as gear characters in the char grid. */
  total_num = 0;
  for (int i = 0; i < row_count; i++) {
    for (int j = 0; file_contents_container[i][j] != '\0'; j++) {
      if (file_contents_container[i][j] == GEAR_CHAR){
        total_num += values[i][j];
      }
    }
  }

  printf("Part Two total num is %d\n", total_num);

  return 0;
}

/* max - Accepts two ints and returns the value of the *larger* one. */
int max(const int num1, const int num2) {
    return (num1 > num2) ? num1 : num2;
}

/* min - Accepts two ints and returns the value of the *smaller* one. */
int min(const int num1, const int num2) {
    return (num1 < num2) ? num1 : num2;
}

/* is_digit
 *
 * Checks if a given ASCII/ASCII-like character is a digit '0' -- '9' (U+0030 -- U+0039)
 * Accepts: a single `char`, the character to be checked.
 * Returns: 1 if the character is a digit, 0 otherwise.
 * NOTE: I don't quite know the truthy-int interaction in non clang compilers.
 * This _may_ not work as expected.
 */
int is_digit(const char c) {
    return c >= '0' && c <= '9';
}

/* is_ctrl
 *
 * Checks if a given ASCII/ASCII-like character is a control character (U+0000 -- U+001F)
 * Accepts a single `char`, the character to be checked.
 * Returns 1 if the character is a control character, 0 otherwise.
 * NOTE: I don't quite know the truthy-int interaction in non clang compilers.
 * This _may_ not work as expected.
 */
int is_ctrl(const char c) {
    return c >= 0 && c <= 31;
}
  
/* is_special
 *
 * Checks if a given ASCII/ASCII-like character is 'special'.
 * Accepts a single `char`, the character to be checked.
 * Returns 1 if the character is 'special', 0 otherwise.
 * NOTE: For the purposes of this problem,
 * I will be considering all non-digit and non-control characters
 * to be 'special'.
 */
int is_special(const char c) {
    return !is_digit(c) && !is_ctrl(c) && c != '.';
}
