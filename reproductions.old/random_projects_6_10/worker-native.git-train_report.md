# Train report for javascript / file:///tmp/top-repos-quality-repos-ioaweb38/worker-native.git HEAD f2cd49bbb85f6c59decc71363ee86d483f05cf5f

### Classification report

PPCR: 0.180

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 1.000| 0.320| 0.991| 0.483| 490| 1532| 0.320 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 117| 0.034 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 119| 0.025 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 141| 0.014 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 677| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 182| 0.000 |
| `micro avg` | 0.982| 0.982| 0.177| 0.982| 0.300| 499| 2768| 0.180 |
| `weighted avg` | 0.964| 0.982| 0.177| 0.973| 0.267| 499| 2768| 0.180 |
| `macro avg` | 0.164| 0.167| 0.053| 0.165| 0.080| 499| 2768| 0.180 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1042 |490 |0 |0 |0 |0 |0 |
|677 |0 |0 |0 |0 |0 |0 |
|182 |0 |0 |0 |0 |0 |0 |
|139 |2 |0 |0 |0 |0 |0 |
|116 |3 |0 |0 |0 |0 |0 |
|113 |4 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| request.js | 6 |
| example.js | 2 |
| child.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1651499831479609, "precision": 0.16366065464261856, "recall": 0.16666666666666666, "support": 499}, "micro avg": {"f1-score": 0.9819639278557114, "precision": 0.9819639278557114, "recall": 0.9819639278557114, "support": 499}, "weighted avg": {"f1-score": 0.9730279568236574, "precision": 0.9642531556098167, "recall": 0.9819639278557114, "support": 499}, "\u2205": {"f1-score": 0.9908998988877654, "precision": 0.9819639278557114, "recall": 1.0, "support": 490}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "macro avg": {"f1-score": 0.08042015427539799, "precision": 0.16366065464261856, "recall": 0.05330722367275892, "support": 2768}, "micro avg": {"f1-score": 0.2999693908784818, "precision": 0.9819639278557114, "recall": 0.17702312138728324, "support": 2768}, "weighted avg": {"f1-score": 0.2670599920879546, "precision": 0.5434858155617593, "recall": 0.17702312138728324, "support": 2768}, "\u2205": {"f1-score": 0.48252092565238797, "precision": 0.9819639278557114, "recall": 0.3198433420365535, "support": 1532}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 119}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 677}},
  "ppcr": 0.18027456647398843
}
```
</details>
