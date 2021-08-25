# Test report for javascript / file:///tmp/top-repos-quality-repos-vfco_hjt/random-code.git HEAD 605a2e5345fb948b662aaa7925958dc53f50aa52

### Classification report

PPCR: 0.219

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.404| 1.000| 0.576| 256| 633| 0.404 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 116| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 82| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 336| 0.000 |
| `macro avg` | 0.250| 0.250| 0.101| 0.250| 0.144| 256| 1167| 0.219 |
| `micro avg` | 1.000| 1.000| 0.219| 1.000| 0.360| 256| 1167| 0.219 |
| `weighted avg` | 1.000| 1.000| 0.219| 1.000| 0.312| 256| 1167| 0.219 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|377 |256 |0 |0 |0 |
|336 |0 |0 |0 |0 |
|116 |0 |0 |0 |0 |
|82 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 256}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 256}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 256}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 256}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "macro avg": {"f1-score": 0.1439820022497188, "precision": 0.25, "recall": 0.10110584518167456, "support": 1167}, "micro avg": {"f1-score": 0.35980323260716796, "precision": 1.0, "recall": 0.21936589545844046, "support": 1167}, "weighted avg": {"f1-score": 0.31239282750324593, "precision": 0.5424164524421594, "recall": 0.21936589545844046, "support": 1167}, "\u2205": {"f1-score": 0.5759280089988752, "precision": 1.0, "recall": 0.40442338072669826, "support": 633}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 336}},
  "ppcr": 0.21936589545844046
}
```
</details>
