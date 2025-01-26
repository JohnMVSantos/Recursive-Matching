import numpy as np

def match(matrix: np.ndarray, axis: int=1, limit: bool=True) -> np.ndarray: # NOSONAR
    """
    Recursive matching algorithm which iterates
    over the rows or columns depending on the axis
    specified and finds the best match for each
    row or column. 
    
    Parameters
    ----------
    matrix : np.ndarray
        The input matrix as a 2D array. Each row 
        can only be matched to one item in the
        column or vice versa.
    axis : int
        For row by row matching, specify as 0. For column
        by column matching, specify as 1. Row by row means
        to loop through each row and find the best value
        of each row based on the maximum.
    limit : bool
        Do not match if the value of the match is the
        minimum in the matrix.

    Returns
    -------
    np.ndarray
        This is a one-dimensional array. If axis is specified to 0,
        this array will have the same length as the rows. If axis is
        specified to 1, this array will have the same length as the
        columns. The values are the indices of rows (axis=1) or columns
        (axis=0) that are the best match. If the value is -1, this
        means there is no match for this particular row/column.
    """
    if axis not in [0, 1]:
        print(f"\t - [WARNING]: Axis can only between 0 or 1. Got {axis}")
        return
    
    min_value = np.min(matrix) # Do not match if the maximum value is less than this value.
    # Assigning -1 indicates the column/row has not been matched.
    matches = np.ones(matrix.shape[axis], dtype=np.int32) * -1 

    def rematch(i: int, items: np.ndarray):
        """
        Maintain unique matches by rematching 
        a duplicate match that does not 
        fall in favor as the best match for 
        the item.

        Parameters
        ----------
        i : int
            The current index of the row 
            and column to find the best match.
        items : np.ndarray
            The current row or column of 
            items to find the best match.
        """
        # The row/column index that best matches the column/row.
        max_index = np.argmax(items) 
        if limit and items[max_index] <= min_value:
            return

        # Maintain unique matches.
        if max_index in matches:
            # The row/column that is already matched.
            j = np.nonzero(matches == max_index)[0] 
            if (matrix[(j, max_index) if axis == 0 
                       else (max_index, j)] <= items[max_index]):

                matches[j] = -1 # Unmatch previous because current match is a better fit.
                matches[i] = max_index # Match the current column/row with the row/column.

                items = matrix[(j, slice(None)) if axis == 0 else 
                               (slice(None), j)]
                items = np.squeeze(items)
                
                # Rematch previous match.
                i = j

            # Reassign the highest value to the minimum - 1 to take 
            # the second highest value as the next potential match.
            items[max_index] = np.min(items) - 1
            # If i has not changed to j, rematch current match.
            rematch(i=i, items=items) 

        else:
            matches[i] = max_index # Add a new match.

    for i in range(matrix.shape[axis]):
        rematch(
            i=i, 
            items=matrix[(i, slice(None)) if axis == 0 else (slice(None), i)]
        )
        
    return matches
