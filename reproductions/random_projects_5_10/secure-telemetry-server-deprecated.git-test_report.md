# Test report for javascript / file:///tmp/top-repos-quality-repos-n5znpwc5/secure-telemetry-server-deprecated.git HEAD ebde174bc64739e41d19a6d1627359af8c2e53a2

### Classification report

PPCR: 0.904

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.893| 0.953| 0.950| 0.922| 0.921| 422| 423| 0.998 |
| `␣` | 0.819| 0.972| 0.886| 0.889| 0.851| 144| 158| 0.911 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 70| 0.629 |
| `⏎` | 0.857| 0.600| 0.511| 0.706| 0.640| 40| 47| 0.851 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 22| 0.636 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 12| 0.500 |
| `'` | 0.037| 0.250| 0.125| 0.065| 0.057| 4| 8| 0.500 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 8| 0.250 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.783| 0.839| 0.758| 0.807| 0.741| 676| 748| 0.904 |
| `macro avg` | 0.261| 0.277| 0.247| 0.258| 0.247| 676| 748| 0.904 |
| `micro avg` | 0.839| 0.839| 0.758| 0.839| 0.796| 676| 748| 0.904 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |402 |20 |0 |0 |0 |0 |0 |0 |
|14 |4 |140 |0 |0 |0 |0 |0 |0 |
|7 |14 |2 |24 |0 |0 |0 |0 |0 |
|4 |2 |1 |0 |1 |0 |0 |0 |0 |
|26 |13 |5 |0 |26 |0 |0 |0 |0 |
|6 |0 |0 |2 |0 |0 |0 |0 |0 |
|8 |10 |2 |2 |0 |0 |0 |0 |0 |
|6 |5 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u0027": {"f1-score": 0.06451612903225806, "precision": 0.037037037037037035, "recall": 0.25, "support": 4}, "macro avg": {"f1-score": 0.25813057194861766, "precision": 0.26062266778056253, "recall": 0.2774828857293312, "support": 676}, "micro avg": {"f1-score": 0.8387573964497042, "precision": 0.8387573964497042, "recall": 0.8387573964497042, "support": 676}, "weighted avg": {"f1-score": 0.8070785528891157, "precision": 0.7830107484358496, "recall": 0.8387573964497042, "support": 676}, "\u2205": {"f1-score": 0.9220183486238532, "precision": 0.8933333333333333, "recall": 0.95260663507109, "support": 422}, "\u23ce": {"f1-score": 0.7058823529411764, "precision": 0.8571428571428571, "recall": 0.6, "support": 40}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.8888888888888888, "precision": 0.8187134502923976, "recall": 0.9722222222222222, "support": 144}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 70}, "\u0027": {"f1-score": 0.05714285714285714, "precision": 0.037037037037037035, "recall": 0.125, "support": 8}, "macro avg": {"f1-score": 0.24691688862428057, "precision": 0.26062266778056253, "recall": 0.2472068857168507, "support": 748}, "micro avg": {"f1-score": 0.7963483146067415, "precision": 0.8387573964497042, "recall": 0.7580213903743316, "support": 748}, "weighted avg": {"f1-score": 0.7414067356588291, "precision": 0.7323779889414564, "recall": 0.7580213903743316, "support": 748}, "\u2205": {"f1-score": 0.9209621993127147, "precision": 0.8933333333333333, "recall": 0.950354609929078, "support": 423}, "\u23ce": {"f1-score": 0.64, "precision": 0.8571428571428571, "recall": 0.5106382978723404, "support": 47}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.8510638297872339, "precision": 0.8187134502923976, "recall": 0.8860759493670886, "support": 158}},
  "ppcr": 0.9037433155080213
}
```
</details>
