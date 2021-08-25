# Train report for javascript / file:///tmp/top-repos-quality-repos-ztawvvmc/admap.git HEAD b4c1c9febb287c404f528949d400b48e3da4e4a1

### Classification report

PPCR: 0.265

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.401| 1.000| 0.572| 267| 666| 0.401 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 235| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 106| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.265| 1.000| 0.379| 267| 1007| 0.265 |
| `micro avg` | 1.000| 1.000| 0.265| 1.000| 0.419| 267| 1007| 0.265 |
| `macro avg` | 0.333| 0.333| 0.134| 0.333| 0.191| 267| 1007| 0.265 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|399 |267 |0 |0 |
|235 |0 |0 |0 |
|106 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 267}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 267}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 267}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 267}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"macro avg": {"f1-score": 0.19078242229367628, "precision": 0.3333333333333333, "recall": 0.13363363363363365, "support": 1007}, "micro avg": {"f1-score": 0.41915227629513346, "precision": 1.0, "recall": 0.26514399205561073, "support": 1007}, "weighted avg": {"f1-score": 0.37853354492826735, "precision": 0.6613704071499503, "recall": 0.26514399205561073, "support": 1007}, "\u2205": {"f1-score": 0.5723472668810289, "precision": 1.0, "recall": 0.4009009009009009, "support": 666}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 235}},
  "ppcr": 0.26514399205561073
}
```
</details>
