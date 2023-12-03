#include <stdio.h>

#define MODE 1 // `1` for Part 1 mode, `2` for Part 2
#define START 0
#define STATE_O 1
#define STATE_ON 2
#define STATE_ONE (-1)
#define STATE_T 4
#define STATE_TW 5
#define STATE_TWO (-2)
#define STATE_TH 7
#define STATE_THR 8
#define STATE_THRE 9
#define STATE_THREE (-3)
#define STATE_F 11
#define STATE_FO 12
#define STATE_FOU 13
#define STATE_FOUR (-4)
#define STATE_FI 15
#define STATE_FIV 16
#define STATE_FIVE (-5)
#define STATE_S 18
#define STATE_SI 19
#define STATE_SIX (-6)
#define STATE_SE 21
#define STATE_SEV 22
#define STATE_SEVE 23
#define STATE_SEVEN (-7)
#define STATE_E 25
#define STATE_EI 26
#define STATE_EIG 27
#define STATE_EIGH 28
#define STATE_EIGHT (-8)
#define STATE_N 30
#define STATE_NI 31
#define STATE_NIN 32
#define STATE_NINE (-9)

void StateMachine_functions(int *status, char readout) {
  switch (readout) {
  case 'e':
    switch (*status) {
    case STATE_ON:
      *status = STATE_ONE;
      break;
    case STATE_THR:
      *status = STATE_THRE;
      break;
    case STATE_THRE:
      *status = STATE_THREE;
      break;
    case STATE_FIV:
      *status = STATE_FIVE;
      break;
    case STATE_S:
      *status = STATE_SE;
      break;
    case STATE_SEV:
      *status = STATE_SEVE;
      break;
    case STATE_NIN:
      *status = STATE_NINE;
      break;
    default:
      *status = STATE_E;
    }
    break;

  case 'f':
    *status = STATE_F;
    break;

  case 'g':
    if (*status == STATE_EI) {
      *status = STATE_EIG;
    } else {
      *status = START;
    }
    break;

  case 'h':
    if (*status == STATE_T || *status == STATE_EIGHT) {
      *status = STATE_TH;
    } else if (*status == STATE_EIG) {
      *status = STATE_EIGH;
    } else {
      *status = START;
    }
    break;

  case 'i':
    switch (*status) {
    case STATE_F:
      *status = STATE_FI;
      break;
    case STATE_S:
      *status = STATE_SI;
      break;
    case STATE_E:
    case STATE_SE:
    case STATE_SEVE:
    case STATE_ONE:
    case STATE_THREE:
    case STATE_THRE:
    case STATE_FIVE:
    case STATE_NINE:
      *status = STATE_EI;
      break;
    case STATE_N:
    case STATE_SEVEN:
      *status = STATE_NI;
      break;
    default:
      *status = START;
    }
    break;

  case 'n':
    switch (*status) {
    case STATE_SEVE:
      *status = STATE_SEVEN;
      break;
    case STATE_NI:
      *status = STATE_NIN;
      break;
    case STATE_O:
    case STATE_TWO:
      *status = STATE_ON;
      break;
    default:
      *status = STATE_N;
    }
    break;

  case 'o':
    switch (*status) {
    case STATE_TW:
      *status = STATE_TWO;
      break;
    case STATE_F:
      *status = STATE_FO;
      break;
    default:
      *status = STATE_O;
    }
    break;

  case 'r':
    if (*status == STATE_TH) {
      *status = STATE_THR;
    } else if (*status == STATE_FOU) {
      *status = STATE_FOUR;
    } else {
      *status = START;
    }
    break;

  case 's':
    *status = STATE_S;
    break;

  case 't':
    if (*status == STATE_EIGH) {
      *status = STATE_EIGHT;
    } else {
      *status = STATE_T;
    }
    break;

  case 'u':
    if (*status == STATE_FO) {
      *status = STATE_FOU;
    } else {
      *status = START;
    }
    break;

  case 'v':
    if (*status == STATE_FI) {
      *status = STATE_FIV;
    } else if (*status == STATE_SE) {
      *status = STATE_SEV;
    } else {
      *status = START;
    }
    break;

  case 'w':
    if (*status == STATE_T) {
      *status = STATE_TW;
    } else if (*status == STATE_EIGHT) {
      *status = STATE_TW;
    } else {
      *status = START;
    }
    break;

  case 'x':
    if (*status == STATE_SI) {
      *status = STATE_SIX;
    } else {
      *status = START;
    }
    break;

  default:
    *status = START;
  }
}

int process_file(FILE *file) {
  if (file == NULL) {
    puts("Unable to open the file.\n");
    return -1;
  }

  char line[100];
  int status;
  int total_value = 0;
  int j = 0;

  while (fgets(line, sizeof(line), file) != NULL) {
    j++;
    status = 0;

    int int_l = -1;
    int int_r = -1;

    int i = 0;
    for (; line[i] != '\0'; i++) {

      if (line[i] >= '0' && line[i] <= '9') {
        int_l = line[i] - '0';
        break;
      }

      if (MODE == 1) {
        continue;
      }

      StateMachine_functions(&status, line[i]);

      if (status < 0) {
        int_l = 0 - status;
        break;
      }
    }

    status = 0;

    for (; line[i] != '\0'; i++) {
      if (line[i] >= '0' && line[i] <= '9') {
        // Overwrite idx_r if another digit is found
        int_r = line[i] - '0';
      }

      if (MODE == 1) {
        continue;
      }

      StateMachine_functions(&status, line[i]);

      if (status < 0) {
        // Overwrite if another letter is found
        int_r = 0 - status;
      }
    }

    if (i == 1) {
      continue;
    }

    int int_line = 10 * int_l + int_r;

    printf("%sAt line %i \t %i + %i = %i\n", line, j, int_l, int_r, int_line);

    total_value += int_line;
  }
  return total_value;
}

int main() {
  FILE *file;

  file = fopen("input.txt", "r");

  const int value = process_file(file);

  if (value < 0) {
    return -1;
  }

  printf("TOTAL VALUE: %i\n", value);

  fclose(file);

  return 0;
}
