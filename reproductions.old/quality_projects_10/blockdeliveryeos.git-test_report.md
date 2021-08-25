# Test report for javascript / file:///tmp/top-repos-quality-repos-t79wzl_9/blockdeliveryeos.git HEAD af8c3b962b37cc55a4f90c90145b3aeb2a1e16fc

### Classification report

PPCR: 0.455

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.982| 1.000| 0.643| 0.991| 0.777| 54| 84| 0.643 |
| `∅` | 1.000| 0.969| 0.674| 0.984| 0.805| 32| 46| 0.696 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 36| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 18| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `macro avg` | 0.248| 0.246| 0.165| 0.247| 0.198| 86| 189| 0.455 |
| `micro avg` | 0.988| 0.988| 0.450| 0.988| 0.618| 86| 189| 0.455 |
| `weighted avg` | 0.989| 0.988| 0.450| 0.988| 0.541| 86| 189| 0.455 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|14 |31 |1 |0 |0 |0 |0 |0 |
|30 |0 |54 |0 |0 |0 |0 |0 |
|18 |0 |0 |0 |0 |0 |0 |0 |
|36 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24686908402504731, "precision": 0.24772727272727274, "recall": 0.24609375, "support": 86}, "micro avg": {"f1-score": 0.9883720930232558, "precision": 0.9883720930232558, "recall": 0.9883720930232558, "support": 86}, "weighted avg": {"f1-score": 0.9883331470700789, "precision": 0.9885835095137421, "recall": 0.9883720930232558, "support": 86}, "\u2205": {"f1-score": 0.9841269841269841, "precision": 1.0, "recall": 0.96875, "support": 32}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9908256880733944, "precision": 0.9818181818181818, "recall": 1.0, "support": 54}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "macro avg": {"f1-score": 0.19777165280762404, "precision": 0.24772727272727274, "recall": 0.16459627329192547, "support": 189}, "micro avg": {"f1-score": 0.6181818181818182, "precision": 0.9883720930232558, "recall": 0.4497354497354497, "support": 189}, "weighted avg": {"f1-score": 0.5412970798376758, "precision": 0.6797498797498798, "recall": 0.4497354497354497, "support": 189}, "\u2205": {"f1-score": 0.8051948051948052, "precision": 1.0, "recall": 0.6739130434782609, "support": 46}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.776978417266187, "precision": 0.9818181818181818, "recall": 0.6428571428571429, "support": 84}},
  "ppcr": 0.455026455026455
}
```
</details>
