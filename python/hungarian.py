import numpy as np
from scipy.optimize import linear_sum_assignment

# Given matrix
matrix = np.array([[0.        , 0.        , 0.        , 0.        , 0.        ],
                   [0.20689655, 0.07407407, 0.04761905, 0.        , 0.23076923],
                   [0.        , 0.        , 0.38461538, 0.        , 0.        ],
                   [0.        , 0.        , 0.04347826, 0.5       , 0.        ],
                   [0.5       , 0.        , 0.        , 0.        , 1.        ]])

# Step 1: Convert to minimization by subtracting each element from the max in each row
row_max = matrix.max(axis=1).reshape(-1, 1)  # Get row-wise max
cost_matrix = row_max - matrix  # Convert to cost matrix for minimization

# Step 2: Apply the Hungarian algorithm (linear_sum_assignment)
row_ind, col_ind = linear_sum_assignment(cost_matrix)

# Step 3: Calculate the maximum total by summing the values in the original matrix
max_total = matrix[row_ind, col_ind].sum()

print("Optimal assignment:", list(zip(row_ind, col_ind)))
print("Maximum total:", max_total)