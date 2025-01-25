from utils import iou_npy
import numpy as np




def match(matrix: np.ndarray, axis: int=1) -> np.ndarray:
    """
    
    Parameters
    ----------
    matrix : np.ndarray
        The input matrix as a 2D array.
    axis : int
        For row by row matching, specify as 0. For column
        by column matching, specify as 1. Row by row means
        to loop through each row and find the best value
        of each row based on the maximum.
    """
    if axis not in [0, 1]:
        print(f"\t - [WARNING]: Axis can only between 0 or 1. Got {axis}")
        return
    
    limit = np.min(matrix) # Do not match if the maximum value is less than this value.
    # Assigning -1 indicates the column/row has not been matched.
    matches = np.ones(matrix.shape[axis], dtype=np.int32) * -1 

    def rematch(i: int, items: np.ndarray):
        # The row/column index that best matches the column/row.
        max_index = np.argmax(items) 
        if items[max_index] <= limit:
            return

        # Maintain unique matches.
        if max_index in matches:
            # The row/column that is already matched.
            j = np.nonzero(matches == max_index)[0] 
            if axis == 0:
                # Previous match <= current match.
                if (matrix[j, max_index] <= items[max_index]):
                    matches[j] = -1 # Unmatch previous because current match is a better fit.
                    matches[i] = max_index # Match the current column/row with the row/column.

                    items = matrix[j, :]
                    items = np.squeeze(items)
                    
                    # Rematch previous match.
                    i = j

            else:
                # Previous match <= current match.
                if (matrix[max_index, j] <= items[max_index]):
                    matches[j] = -1 # Unmatch previous because current match is a better fit.
                    matches[i] = max_index # Match the current column/row with the row/column.

                    items = matrix[:, j]
                    items = np.squeeze(items)
                    
                    # Rematch previous match.
                    i = j

            items[max_index] = np.min(items)
            # If i has not changed to j, rematch current match.
            rematch(i=i, items=items) 

        else:
            matches[i] = max_index # Add a new match.

    if axis == 0:
        for i in range(len(matches)):
            row = matrix[i, :]
            rematch(i=i, items=row)
    else:
        for i in range(len(matches)):
            column = matrix[:, i]
            rematch(i=i, items=column)

    return matches


gt = np.array([[10,110,60,140], [20, 60, 50, 90], [60,40,100,100], [100,10,160,70], [20, 100, 50, 140]])
dt = np.array([[120,100,160,140], [30,80,70,130], [70,30,90,90], [90,20,150,60], [20, 100, 50, 140]])

matrix = iou_npy(dt, gt)

print(f"{matrix=}")

matches = match(matrix, axis=0)
print(f"{matches=}")




