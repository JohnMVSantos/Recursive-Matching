# Recursive-Matching

An algorithm designed to formulate unique assignments with the highest (default)
or lowest values found by recursively matching and rematching pairs to 
assert the best matching pair. 

This algorithm differs from the Hungarian Algorithm which seeks to formulate
assignments based on the minimum (default) or maximum sum of the assignments. 

* The assignments formulated with the maximum sum does not always guarantee individual assignments with the highest possible outcome. 
* Similarly, the assignments formulated with the minimum sum does not always guarantee individual assignments with the lowest possible outcome.

# Application

## Computer Vision

Consider the following figure which shows a set of ground truth (blue) bounding boxes and a set of prediction (red) bounding boxes. 

![Computer Vision Sample](/docs/images/cv_demo_bbx_graph.png)

Visually, it is clear which ground truth bounding box closely correlates to the prediction bounding box. 

Graphically, the coordinates of the bounding boxes are as follows.

## Ground Truths

* G0 [10, 110, 60, 140]
* G1 [20, 60, 50, 90]
* G2 [60, 40, 100, 100]
* G3 [100, 10, 160, 70]
* G4 [20, 100, 50, 140]

## Predictions

* P0 [120, 100, 160, 140]
* P1 [30, 80, 70, 130]
* P2 [70, 30, 90, 90]
* P3 [90, 20, 150, 60]
* P4 [20, 100, 50, 140]

The ground truth to prediction bounding boxes are matched as follows.

* (G0, P1)
* (G2, P2)
* (G3, P3)
* (G4, P4)

The IoU (intersection over union) is a metric that best describes
how well a bounding box intersects with another which is the metric used to
measure the best matches between a set of bounding boxes.

In this example, given a set of ground truth and prediction bounding boxes,
an IoU 2D matrix is generated which is taken as an input to the recursive matching
algorithm to find the best matches for the ground truth and prediction bounding
boxes.

The recursive matching algorithm will match based on the highest IoU matches which
differs from the hungarian algorithm where the assignments are based on 
the maximum sum overall where the individual matches may not be the maximum
within a set of options.

For more information, see `/python/demo.ipynb`

# Modules
The algorithm will be implemented in three languages: Python, Rust, C.

## Python

This implementation can be found under `/python`. However, it can be
installed via pip with the following command.

```shell
pip install recursive-algorithm
```

To use the module, first import the algorithm.

```python
from matching import recursive_match
```

The following is an example deployment of the algorithm.

```python
import numpy as np

matrix = np.array([
        [0.,         0.,         0.,         0.,         0.        ],
        [0.20689655, 0.07407407, 0.04761905, 0.,         0.23076923],
        [0.,         0.,         0.38461538, 0.,         0.,        ],
        [0.,         0.,         0.04347826, 0.5,        0.,        ],
        [0.5,        0.,         0.,         0.,         1.,        ]
    ], dtype=np.float32)
    
matches = recursive_match(matrix=matrix)
print(f"{matches=}")
```

## Rust
TBA.

## C
TBA.