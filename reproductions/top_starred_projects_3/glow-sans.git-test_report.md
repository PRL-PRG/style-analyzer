# Test report for javascript / file:///tmp/top-repos-quality-repos-wqaaepdw/glow-sans.git HEAD 52434233ac43f40be3b28730689339d19c8f2964

### Classification report

PPCR: 0.881

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.903| 0.982| 0.930| 0.941| 0.916| 1911| 2018| 0.947 |
| `␣` | 0.952| 0.846| 0.741| 0.896| 0.834| 1033| 1179| 0.876 |
| `'` | 0.856| 0.955| 0.955| 0.903| 0.903| 112| 112| 1.000 |
| `⏎` | 0.737| 0.519| 0.250| 0.609| 0.373| 108| 224| 0.482 |
| `⏎␣⁺␣⁺` | 0.833| 0.556| 0.474| 0.667| 0.604| 81| 95| 0.853 |
| `⏎␣⁻␣⁻` | 0.870| 0.959| 0.540| 0.913| 0.667| 49| 87| 0.563 |
| `"` | 0.667| 0.773| 0.773| 0.716| 0.716| 44| 44| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 50| 0.380 |
| `weighted avg` | 0.901| 0.906| 0.798| 0.900| 0.831| 3357| 3809| 0.881 |
| `micro avg` | 0.906| 0.906| 0.798| 0.906| 0.848| 3357| 3809| 0.881 |
| `macro avg` | 0.727| 0.699| 0.583| 0.705| 0.627| 3357| 3809| 0.881 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|107 |1877 |34 |0 |0 |0 |0 |0 |0 |
|146 |107 |874 |18 |8 |10 |9 |7 |0 |
|116 |51 |1 |56 |0 |0 |0 |0 |0 |
|0 |0 |1 |0 |107 |4 |0 |0 |0 |
|0 |0 |0 |0 |10 |34 |0 |0 |0 |
|14 |24 |8 |1 |0 |3 |45 |0 |0 |
|38 |2 |0 |0 |0 |0 |0 |47 |0 |
|31 |18 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.7157894736842105, "precision": 0.6666666666666666, "recall": 0.7727272727272727, "support": 44}, "\u0027": {"f1-score": 0.9029535864978904, "precision": 0.856, "recall": 0.9553571428571429, "support": 112}, "macro avg": {"f1-score": 0.7054412078795218, "precision": 0.7272650119058788, "recall": 0.698703726436967, "support": 3357}, "micro avg": {"f1-score": 0.9055704498063747, "precision": 0.9055704498063747, "recall": 0.9055704498063747, "support": 3357}, "weighted avg": {"f1-score": 0.8997821588623552, "precision": 0.9007279352404911, "recall": 0.9055704498063747, "support": 3357}, "\u2205": {"f1-score": 0.9408521303258146, "precision": 0.9028379028379029, "recall": 0.9822082679225537, "support": 1911}, "\u23ce": {"f1-score": 0.6086956521739131, "precision": 0.7368421052631579, "recall": 0.5185185185185185, "support": 108}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6666666666666667, "precision": 0.8333333333333334, "recall": 0.5555555555555556, "support": 81}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.912621359223301, "precision": 0.8703703703703703, "recall": 0.9591836734693877, "support": 49}, "\u2423": {"f1-score": 0.8959507944643773, "precision": 0.9520697167755992, "recall": 0.846079380445305, "support": 1033}},
  "cl_report_full": {"\"": {"f1-score": 0.7157894736842105, "precision": 0.6666666666666666, "recall": 0.7727272727272727, "support": 44}, "\u0027": {"f1-score": 0.9029535864978904, "precision": 0.856, "recall": 0.9553571428571429, "support": 112}, "macro avg": {"f1-score": 0.6265777350052302, "precision": 0.7272650119058788, "recall": 0.5829291929115186, "support": 3809}, "micro avg": {"f1-score": 0.8484510186994139, "precision": 0.9055704498063747, "recall": 0.7981097400892623, "support": 3809}, "weighted avg": {"f1-score": 0.8305248950271932, "precision": 0.889882892571932, "recall": 0.7981097400892623, "support": 3809}, "\u2205": {"f1-score": 0.9162802050280694, "precision": 0.9028379028379029, "recall": 0.9301288404360754, "support": 2018}, "\u23ce": {"f1-score": 0.37333333333333335, "precision": 0.7368421052631579, "recall": 0.25, "support": 224}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6040268456375839, "precision": 0.8333333333333334, "recall": 0.47368421052631576, "support": 95}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6666666666666666, "precision": 0.8703703703703703, "recall": 0.5402298850574713, "support": 87}, "\u2423": {"f1-score": 0.8335717691940867, "precision": 0.9520697167755992, "recall": 0.741306191687871, "support": 1179}},
  "ppcr": 0.881333683381465
}
```
</details>
