# Train report for javascript / file:///tmp/top-repos-quality-repos-0bw21y39/fardelcms.git HEAD f53cd614e912700f27f70494db17f81aaaeafb06

### Classification report

PPCR: 0.221

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.421| 1.000| 0.592| 1067| 2537| 0.421 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1390| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 252| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 276| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 107| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 94| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 176| 0.000 |
| `micro avg` | 1.000| 1.000| 0.221| 1.000| 0.362| 1067| 4832| 0.221 |
| `weighted avg` | 1.000| 1.000| 0.221| 1.000| 0.311| 1067| 4832| 0.221 |
| `macro avg` | 0.143| 0.143| 0.060| 0.143| 0.085| 1067| 4832| 0.221 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⇥⁻| ⏎⇥⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1470 |1067 |0 |0 |0 |0 |0 |0 |
|1390 |0 |0 |0 |0 |0 |0 |0 |
|252 |0 |0 |0 |0 |0 |0 |0 |
|276 |0 |0 |0 |0 |0 |0 |0 |
|107 |0 |0 |0 |0 |0 |0 |0 |
|94 |0 |0 |0 |0 |0 |0 |0 |
|176 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.14285714285714285, "precision": 0.14285714285714285, "recall": 0.14285714285714285, "support": 1067}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1067}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1067}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1067}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 276}, "macro avg": {"f1-score": 0.08458855240209293, "precision": 0.14285714285714285, "recall": 0.06008221183625204, "support": 4832}, "micro avg": {"f1-score": 0.3617562298694694, "precision": 1.0, "recall": 0.22081953642384106, "support": 4832}, "weighted avg": {"f1-score": 0.3108874383503245, "precision": 0.5250413907284768, "recall": 0.22081953642384106, "support": 4832}, "\u2205": {"f1-score": 0.5921198668146505, "precision": 1.0, "recall": 0.4205754828537643, "support": 2537}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 252}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 107}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 176}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1390}},
  "ppcr": 0.22081953642384106
}
```
</details>
