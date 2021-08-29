# Train report for javascript / file:///tmp/top-repos-quality-repos-z9pqb3_a/electron-with-server-example.git HEAD 7afdab9354186d796d28ef6e38a7dbb5dc5aab8d

### Classification report

PPCR: 0.183

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.335| 1.000| 0.501| 262| 783| 0.335 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 513| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 138| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.183| 1.000| 0.274| 262| 1434| 0.183 |
| `macro avg` | 0.333| 0.333| 0.112| 0.333| 0.167| 262| 1434| 0.183 |
| `micro avg` | 1.000| 1.000| 0.183| 1.000| 0.309| 262| 1434| 0.183 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|521 |262 |0 |0 |
|513 |0 |0 |0 |
|138 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 262}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 262}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 262}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 262}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 138}, "macro avg": {"f1-score": 0.16714513556618818, "precision": 0.3333333333333333, "recall": 0.11153682418050233, "support": 1434}, "micro avg": {"f1-score": 0.3089622641509434, "precision": 1.0, "recall": 0.18270571827057183, "support": 1434}, "weighted avg": {"f1-score": 0.2737963203939861, "precision": 0.5460251046025104, "recall": 0.18270571827057183, "support": 1434}, "\u2205": {"f1-score": 0.5014354066985646, "precision": 1.0, "recall": 0.334610472541507, "support": 783}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 513}},
  "ppcr": 0.18270571827057183
}
```
</details>
