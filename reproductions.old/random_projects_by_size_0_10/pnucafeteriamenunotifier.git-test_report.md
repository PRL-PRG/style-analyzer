# Test report for javascript / file:///tmp/top-repos-quality-repos-z3g93427/pnucafeteriamenunotifier.git HEAD d458e89371634fe5f48c31c414dcacd64b782c73

### Classification report

PPCR: 0.193

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.338| 1.000| 0.506| 68| 201| 0.338 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 34| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 21| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 97| 0.000 |
| `micro avg` | 1.000| 1.000| 0.193| 1.000| 0.323| 68| 353| 0.193 |
| `weighted avg` | 1.000| 1.000| 0.193| 1.000| 0.288| 68| 353| 0.193 |
| `macro avg` | 0.250| 0.250| 0.085| 0.250| 0.126| 68| 353| 0.193 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|133 |68 |0 |0 |0 |
|97 |0 |0 |0 |0 |
|34 |0 |0 |0 |0 |
|21 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 68}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 68}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 68}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 68}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "macro avg": {"f1-score": 0.12639405204460966, "precision": 0.25, "recall": 0.0845771144278607, "support": 353}, "micro avg": {"f1-score": 0.3230403800475059, "precision": 1.0, "recall": 0.19263456090651557, "support": 353}, "weighted avg": {"f1-score": 0.28787767094579647, "precision": 0.5694050991501416, "recall": 0.19263456090651557, "support": 353}, "\u2205": {"f1-score": 0.5055762081784386, "precision": 1.0, "recall": 0.3383084577114428, "support": 201}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}},
  "ppcr": 0.19263456090651557
}
```
</details>
