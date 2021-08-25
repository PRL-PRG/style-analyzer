# Train report for javascript / file:///tmp/top-repos-quality-repos-8lvps42i/tt.git HEAD 0a5d7c2520e9cbd93b2677f39cbe8e1b42cc611f

### Classification report

PPCR: 0.099

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.986| 1.000| 0.200| 0.993| 0.332| 216| 1082| 0.200 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 157| 314| 0.500 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 1828| 0.002 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 185| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 159| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 145| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 99| 0.000 |
| `micro avg` | 0.992| 0.992| 0.098| 0.992| 0.178| 376| 3812| 0.099 |
| `weighted avg` | 0.984| 0.992| 0.098| 0.988| 0.149| 376| 3812| 0.099 |
| `macro avg` | 0.284| 0.286| 0.100| 0.285| 0.143| 376| 3812| 0.099 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1825 |0 |3 |0 |0 |0 |0 |0 |
|866 |0 |216 |0 |0 |0 |0 |0 |
|157 |0 |0 |157 |0 |0 |0 |0 |
|185 |0 |0 |0 |0 |0 |0 |0 |
|159 |0 |0 |0 |0 |0 |0 |0 |
|145 |0 |0 |0 |0 |0 |0 |0 |
|99 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| ui/src/containers/RouteList.js | 1 |
| ui/src/reducers/trips.js | 1 |
| ui/src/actions/routeAction.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 157}, "macro avg": {"f1-score": 0.28472906403940884, "precision": 0.2837573385518591, "recall": 0.2857142857142857, "support": 376}, "micro avg": {"f1-score": 0.9920212765957447, "precision": 0.9920212765957447, "recall": 0.9920212765957447, "support": 376}, "weighted avg": {"f1-score": 0.9880594277329421, "precision": 0.9841518507723694, "recall": 0.9920212765957447, "support": 376}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.993103448275862, "precision": 0.9863013698630136, "recall": 1.0, "support": 216}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 314}, "macro avg": {"f1-score": 0.14267413345045935, "precision": 0.2837573385518591, "recall": 0.09994718774755743, "support": 3812}, "micro avg": {"f1-score": 0.1781279847182426, "precision": 0.9920212765957447, "recall": 0.0978488982161595, "support": 3812}, "weighted avg": {"f1-score": 0.1491641885503093, "precision": 0.362323736146847, "recall": 0.0978488982161595, "support": 3812}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1828}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 185}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 99}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 159}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 145}, "\u2423": {"f1-score": 0.3320522674865488, "precision": 0.9863013698630136, "recall": 0.19963031423290203, "support": 1082}},
  "ppcr": 0.09863588667366212
}
```
</details>
