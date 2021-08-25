# Test report for javascript / file:///tmp/top-repos-quality-repos-uu2rpnux/selected-chapters-linear-algebra.git HEAD 129997ded763f0752ec453478f067e61d7ba2463

### Classification report

PPCR: 0.185

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.866| 0.942| 0.180| 0.902| 0.298| 103| 539| 0.191 |
| `␣` | 0.793| 0.605| 0.130| 0.687| 0.223| 38| 177| 0.215 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 32| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 16| 0.000 |
| `micro avg` | 0.851| 0.851| 0.157| 0.851| 0.265| 141| 764| 0.185 |
| `weighted avg` | 0.846| 0.851| 0.157| 0.844| 0.262| 141| 764| 0.185 |
| `macro avg` | 0.415| 0.387| 0.077| 0.397| 0.130| 141| 764| 0.185 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|436 |97 |6 |0 |0 |
|139 |15 |23 |0 |0 |
|32 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3972231863936133, "precision": 0.4147937192118227, "recall": 0.3867526826775677, "support": 141}, "micro avg": {"f1-score": 0.8510638297872339, "precision": 0.851063829787234, "recall": 0.851063829787234, "support": 141}, "weighted avg": {"f1-score": 0.8441779228548008, "precision": 0.8464062991300703, "recall": 0.851063829787234, "support": 141}, "\u2205": {"f1-score": 0.9023255813953488, "precision": 0.8660714285714286, "recall": 0.941747572815534, "support": 103}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.6865671641791046, "precision": 0.7931034482758621, "recall": 0.6052631578947368, "support": 38}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "macro avg": {"f1-score": 0.13032601076760175, "precision": 0.4147937192118227, "recall": 0.07747659926836682, "support": 764}, "micro avg": {"f1-score": 0.2651933701657459, "precision": 0.851063829787234, "recall": 0.15706806282722513, "support": 764}, "weighted avg": {"f1-score": 0.2619737274327732, "precision": 0.7947536784618162, "recall": 0.15706806282722513, "support": 764}, "\u2205": {"f1-score": 0.29800307219662064, "precision": 0.8660714285714286, "recall": 0.17996289424860853, "support": 539}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.2233009708737864, "precision": 0.7931034482758621, "recall": 0.12994350282485875, "support": 177}},
  "ppcr": 0.18455497382198952
}
```
</details>
