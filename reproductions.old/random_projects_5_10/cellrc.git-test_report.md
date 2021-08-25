# Test report for javascript / file:///tmp/top-repos-quality-repos-9tcw37dh/cellrc.git HEAD 2694e69ad5e3ec92eb045d03e2ad35522e2d90a6

### Classification report

PPCR: 0.133

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.221| 1.000| 0.361| 30| 136| 0.221 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 21| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 69| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.133| 1.000| 0.218| 30| 226| 0.133 |
| `macro avg` | 0.333| 0.333| 0.074| 0.333| 0.120| 30| 226| 0.133 |
| `micro avg` | 1.000| 1.000| 0.133| 1.000| 0.234| 30| 226| 0.133 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|106 |30 |0 |0 |
|69 |0 |0 |0 |
|21 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 30}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 30}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 30}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 30}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"macro avg": {"f1-score": 0.12048192771084336, "precision": 0.3333333333333333, "recall": 0.07352941176470588, "support": 226}, "micro avg": {"f1-score": 0.23437500000000003, "precision": 1.0, "recall": 0.13274336283185842, "support": 226}, "weighted avg": {"f1-score": 0.21750719692931014, "precision": 0.6017699115044248, "recall": 0.13274336283185842, "support": 226}, "\u2205": {"f1-score": 0.36144578313253006, "precision": 1.0, "recall": 0.22058823529411764, "support": 136}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}},
  "ppcr": 0.13274336283185842
}
```
</details>
