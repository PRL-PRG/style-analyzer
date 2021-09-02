# Test report for javascript / file:///tmp/top-repos-quality-repos-7hsbvtpn/lbry-desktop.git HEAD 79be67831b67ab52a0b04d8cb224131b072a86da

### Classification report

PPCR: 0.978

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.990| 0.989| 0.983| 0.983| 5887| 5892| 0.999 |
| `␣` | 0.957| 0.973| 0.963| 0.965| 0.960| 3670| 3709| 0.989 |
| `⏎` | 0.875| 0.844| 0.776| 0.859| 0.822| 770| 838| 0.919 |
| `'` | 1.000| 0.978| 0.977| 0.989| 0.988| 769| 770| 0.999 |
| `⏎␣⁺␣⁺` | 0.951| 0.848| 0.835| 0.897| 0.889| 435| 442| 0.984 |
| `⏎␣⁻␣⁻` | 0.982| 0.950| 0.913| 0.966| 0.946| 398| 414| 0.961 |
| `⏎⏎` | 0.836| 0.695| 0.378| 0.759| 0.521| 154| 283| 0.544 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 10| 1.000 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 4| 0.500 |
| `macro avg` | 0.731| 0.698| 0.648| 0.713| 0.679| 12095| 12362| 0.978 |
| `weighted avg` | 0.962| 0.963| 0.943| 0.962| 0.949| 12095| 12362| 0.978 |
| `micro avg` | 0.963| 0.963| 0.943| 0.963| 0.953| 12095| 12362| 0.978 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5 |5826 |61 |0 |0 |0 |0 |0 |0 |0 |
|39 |29 |3571 |0 |50 |19 |1 |0 |0 |0 |
|1 |13 |4 |752 |0 |0 |0 |0 |0 |0 |
|68 |56 |43 |0 |650 |0 |0 |21 |0 |0 |
|7 |23 |43 |0 |0 |369 |0 |0 |0 |0 |
|16 |15 |4 |0 |1 |0 |378 |0 |0 |0 |
|129 |1 |5 |0 |41 |0 |0 |107 |0 |0 |
|0 |3 |0 |0 |1 |0 |6 |0 |0 |0 |
|2 |0 |2 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9888231426692965, "precision": 1.0, "recall": 0.9778933680104032, "support": 769}, "macro avg": {"f1-score": 0.7129923705381488, "precision": 0.7307505924266486, "recall": 0.6975046357503284, "support": 12095}, "micro avg": {"f1-score": 0.9634559735427862, "precision": 0.9634559735427862, "recall": 0.9634559735427862, "support": 12095}, "weighted avg": {"f1-score": 0.962463735095108, "precision": 0.962001219933899, "recall": 0.9634559735427862, "support": 12095}, "\u2205": {"f1-score": 0.9830422677803087, "precision": 0.976533690915186, "recall": 0.9896381858331917, "support": 5887}, "\u23ce": {"f1-score": 0.8592200925313945, "precision": 0.8748317631224765, "recall": 0.8441558441558441, "support": 770}, "\u23ce\u23ce": {"f1-score": 0.7588652482269503, "precision": 0.8359375, "recall": 0.6948051948051948, "support": 154}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.896719319562576, "precision": 0.9510309278350515, "recall": 0.8482758620689655, "support": 435}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9655172413793104, "precision": 0.9818181818181818, "recall": 0.949748743718593, "support": 398}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.9647440226935027, "precision": 0.9566032681489419, "recall": 0.9730245231607629, "support": 3670}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9881734559789751, "precision": 1.0, "recall": 0.9766233766233766, "support": 770}, "macro avg": {"f1-score": 0.6787527203803209, "precision": 0.7307505924266486, "recall": 0.6477609175117005, "support": 12362}, "micro avg": {"f1-score": 0.9529378092161753, "precision": 0.9634559735427862, "recall": 0.9426468209027665, "support": 12362}, "weighted avg": {"f1-score": 0.9489692048118177, "precision": 0.9600619443301374, "recall": 0.9426468209027665, "support": 12362}, "\u2205": {"f1-score": 0.9826277618485411, "precision": 0.976533690915186, "recall": 0.9887983706720977, "support": 5892}, "\u23ce": {"f1-score": 0.8222643896268185, "precision": 0.8748317631224765, "recall": 0.7756563245823389, "support": 838}, "\u23ce\u23ce": {"f1-score": 0.5206812652068127, "precision": 0.8359375, "recall": 0.37809187279151946, "support": 283}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8891566265060241, "precision": 0.9510309278350515, "recall": 0.834841628959276, "support": 442}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9461827284105131, "precision": 0.9818181818181818, "recall": 0.9130434782608695, "support": 414}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.959688255845203, "precision": 0.9566032681489419, "recall": 0.9627932057158264, "support": 3709}},
  "ppcr": 0.97840155314674
}
```
</details>