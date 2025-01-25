


def iou_tch(box1, box2, eps: float=1e-7):
    """
    """
    import torch

    (a1, a2), (b1, b2) = box1.unsqueeze(1).chunk(2, 2), box2.unsqueeze(0).chunk(2, 2)
    inter = (torch.min(a2, b2) - torch.max(a1, b1)).clamp(0).prod(2)
    return inter / ((a2 - a1).prod(2) + (b2 - b1).prod(2) - inter + eps)

def iou_npy(box1, box2, eps: float=1e-7): 
    """
    """
    import numpy as np

    a1, a2 = np.expand_dims(box1[:, [0,1]], 1), np.expand_dims(box1[:, [2,3]], 1)
    b1, b2 = np.expand_dims(box2[:, [0,1]], 0), np.expand_dims(box2[:, [2,3]], 0)

    inter = np.minimum(a2,b2) - np.maximum(a1,b1)
    inter = np.prod(np.clip(inter, a_min=0., a_max=np.max(inter)), axis=2)
    
    return inter / ((a2 - a1).prod(2) + (b2 - b1).prod(2) - inter + eps)