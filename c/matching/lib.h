#ifndef LIB_H
#define LIB_H

#include <stdbool.h>
#include "matrix.h"

/** Function Prototypes */
int* recursive_match(Matrix* matrix, int axis, bool limit, bool minimum);

#endif