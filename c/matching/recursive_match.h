#ifndef RECURSIVE_MATCH_H
#define RECURSIVE_MATCH_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include "matrix.h"

// Structure to represent the matcher
typedef struct {
    Matrix* matrix;
    int axis;
    float min_value;
    bool limit;
    int* matches;
    int size;
    
} Matcher;

// Function prototypes
Matcher* create_matcher(Matrix* matrix, int axis, bool limit);
void rematch(Matcher* matcher, int i, float* items);
void run(Matcher* matcher);

#endif
