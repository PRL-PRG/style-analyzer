# Train report for javascript / file:///tmp/top-repos-quality-repos-joybpy87/master_thesis.git HEAD 80e4e61b65a1e67260cf5a1900606c7ccd6d9244

### Classification report

PPCR: 0.164

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 1.000| 1.000| 0.367| 1.000| 0.536| 313| 854| 0.367 |
| `'` | 1.000| 1.000| 0.439| 1.000| 0.610| 302| 688| 0.439 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1796| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 269| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 133| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.164| 1.000| 0.235| 615| 3740| 0.164 |
| `micro avg` | 1.000| 1.000| 0.164| 1.000| 0.282| 615| 3740| 0.164 |
| `macro avg` | 0.400| 0.400| 0.161| 0.400| 0.229| 615| 3740| 0.164 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1796 |0 |0 |0 |0 |0 |
|541 |0 |313 |0 |0 |0 |
|386 |0 |0 |302 |0 |0 |
|269 |0 |0 |0 |0 |0 |
|133 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 302}, "macro avg": {"f1-score": 0.4, "precision": 0.4, "recall": 0.4, "support": 615}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 615}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 615}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 313}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 133}, "\u0027": {"f1-score": 0.61010101010101, "precision": 1.0, "recall": 0.438953488372093, "support": 688}, "macro avg": {"f1-score": 0.22930383526784553, "precision": 0.4, "recall": 0.16109280540275583, "support": 3740}, "micro avg": {"f1-score": 0.2824339839265212, "precision": 1.0, "recall": 0.16443850267379678, "support": 3740}, "weighted avg": {"f1-score": 0.2347194141489125, "precision": 0.4122994652406417, "recall": 0.16443850267379678, "support": 3740}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1796}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 269}, "\u2423": {"f1-score": 0.5364181662382177, "precision": 1.0, "recall": 0.3665105386416862, "support": 854}},
  "ppcr": 0.16443850267379678
}
```
</details>
