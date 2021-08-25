# Train report for javascript / file:///tmp/top-repos-quality-repos-t3g5_svt/ghosttext.git HEAD 58de51cf9a1dc4e017e649f806c104f36461263a

### Classification report

PPCR: 0.317

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.502| 1.000| 0.668| 852| 1697| 0.502 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 640| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 217| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 130| 0.000 |
| `macro avg` | 0.250| 0.250| 0.126| 0.250| 0.167| 852| 2684| 0.317 |
| `weighted avg` | 1.000| 1.000| 0.317| 1.000| 0.423| 852| 2684| 0.317 |
| `micro avg` | 1.000| 1.000| 0.317| 1.000| 0.482| 852| 2684| 0.317 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|845 |852 |0 |0 |0 |
|640 |0 |0 |0 |0 |
|217 |0 |0 |0 |0 |
|130 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 852}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 852}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 852}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 852}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 217}, "macro avg": {"f1-score": 0.16712436249509613, "precision": 0.25, "recall": 0.12551561579257514, "support": 2684}, "micro avg": {"f1-score": 0.4819004524886878, "precision": 1.0, "recall": 0.3174366616989568, "support": 2684}, "weighted avg": {"f1-score": 0.422667724521875, "precision": 0.6322652757078987, "recall": 0.31743666169895685, "support": 2684}, "\u2205": {"f1-score": 0.6684974499803845, "precision": 1.0, "recall": 0.5020624631703006, "support": 1697}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 640}},
  "ppcr": 0.3174366616989568
}
```
</details>
