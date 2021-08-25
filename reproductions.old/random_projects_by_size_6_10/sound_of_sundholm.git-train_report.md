# Train report for javascript / file:///tmp/top-repos-quality-repos-d6c3j4rh/sound_of_sundholm.git HEAD bdd0df84422f79d4f010f393fd6ab36f4fe2a233

### Classification report

PPCR: 0.218

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.370| 1.000| 0.541| 808| 2181| 0.370 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 921| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 270| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 172| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 154| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.218| 1.000| 0.319| 808| 3698| 0.218 |
| `micro avg` | 1.000| 1.000| 0.218| 1.000| 0.359| 808| 3698| 0.218 |
| `macro avg` | 0.200| 0.200| 0.074| 0.200| 0.108| 808| 3698| 0.218 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1373 |808 |0 |0 |0 |0 |
|921 |0 |0 |0 |0 |0 |
|270 |0 |0 |0 |0 |0 |
|172 |0 |0 |0 |0 |0 |
|154 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2, "precision": 0.2, "recall": 0.2, "support": 808}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 808}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 808}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 808}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "macro avg": {"f1-score": 0.1081298093007695, "precision": 0.2, "recall": 0.074094452086199, "support": 3698}, "micro avg": {"f1-score": 0.35863293386595646, "precision": 1.0, "recall": 0.21849648458626283, "support": 3698}, "weighted avg": {"f1-score": 0.3188630531165201, "precision": 0.5897782585181179, "recall": 0.21849648458626283, "support": 3698}, "\u2205": {"f1-score": 0.5406490465038475, "precision": 1.0, "recall": 0.37047226043099496, "support": 2181}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 270}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 921}},
  "ppcr": 0.21849648458626283
}
```
</details>
