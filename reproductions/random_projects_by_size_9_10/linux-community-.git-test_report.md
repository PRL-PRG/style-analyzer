# Test report for javascript / file:///tmp/top-repos-quality-repos-ys8pl_1m/linux-community-.git HEAD ffd67d7c28195404b593af1e539a82e3d0af13a9

### Classification report

PPCR: 0.819

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.857| 0.769| 0.682| 0.811| 0.759| 39| 44| 0.886 |
| `␣` | 0.516| 0.800| 0.696| 0.627| 0.593| 20| 23| 0.870 |
| `"` | 1.000| 0.154| 0.143| 0.267| 0.250| 13| 14| 0.929 |
| `⏎` | 1.000| 0.667| 0.222| 0.800| 0.364| 3| 9| 0.333 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 4| 0.500 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.649| 0.649| 0.532| 0.649| 0.585| 77| 94| 0.819 |
| `weighted avg` | 0.776| 0.649| 0.532| 0.650| 0.573| 77| 94| 0.819 |
| `macro avg` | 0.422| 0.299| 0.218| 0.313| 0.246| 77| 94| 0.819 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|5 |30 |9 |0 |0 |0 |0 |
|3 |4 |16 |0 |0 |0 |0 |
|6 |0 |1 |2 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |
|2 |1 |1 |0 |0 |0 |0 |
|1 |0 |4 |0 |7 |0 |2 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.2666666666666667, "precision": 1.0, "recall": 0.15384615384615385, "support": 13}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3131160572337043, "precision": 0.4216589861751152, "recall": 0.2987179487179487, "support": 77}, "micro avg": {"f1-score": 0.6493506493506493, "precision": 0.6493506493506493, "recall": 0.6493506493506493, "support": 77}, "weighted avg": {"f1-score": 0.6498351674822263, "precision": 0.7759889879705547, "recall": 0.6493506493506493, "support": 77}, "\u2205": {"f1-score": 0.8108108108108107, "precision": 0.8571428571428571, "recall": 0.7692307692307693, "support": 39}, "\u23ce": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.6274509803921569, "precision": 0.5161290322580645, "recall": 0.8, "support": 20}},
  "cl_report_full": {"\"": {"f1-score": 0.25, "precision": 1.0, "recall": 0.14285714285714285, "support": 14}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.245715328389379, "precision": 0.4216589861751152, "recall": 0.21781871510132378, "support": 94}, "micro avg": {"f1-score": 0.5847953216374269, "precision": 0.6493506493506493, "recall": 0.5319148936170213, "support": 94}, "weighted avg": {"f1-score": 0.5725540257589813, "precision": 0.7721835474066084, "recall": 0.5319148936170213, "support": 94}, "\u2205": {"f1-score": 0.759493670886076, "precision": 0.8571428571428571, "recall": 0.6818181818181818, "support": 44}, "\u23ce": {"f1-score": 0.3636363636363636, "precision": 1.0, "recall": 0.2222222222222222, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.5925925925925926, "precision": 0.5161290322580645, "recall": 0.6956521739130435, "support": 23}},
  "ppcr": 0.8191489361702128
}
```
</details>
