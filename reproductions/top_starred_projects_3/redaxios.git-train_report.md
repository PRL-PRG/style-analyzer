# Train report for javascript / file:///tmp/top-repos-quality-repos-enqb55rb/redaxios.git HEAD f963b6f307031572cd591d4a39f32919b2d13354

### Classification report

PPCR: 0.229

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.338| 1.000| 0.505| 532| 1575| 0.338 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 752| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.229| 1.000| 0.342| 532| 2327| 0.229 |
| `micro avg` | 1.000| 1.000| 0.229| 1.000| 0.372| 532| 2327| 0.229 |
| `macro avg` | 0.500| 0.500| 0.169| 0.500| 0.252| 532| 2327| 0.229 |

### Confusion matrix

|refusal|  ∅| ␣| 
|:---|:---|:---|
|0 |0 |0 |
|1043 |532 |0 |
|752 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.5, "precision": 0.5, "recall": 0.5, "support": 532}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 532}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 532}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 532}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"macro avg": {"f1-score": 0.25249169435215946, "precision": 0.5, "recall": 0.1688888888888889, "support": 2327}, "micro avg": {"f1-score": 0.37215809723679605, "precision": 1.0, "recall": 0.22862054146970348, "support": 2327}, "weighted avg": {"f1-score": 0.3417915071806198, "precision": 0.6768371293510959, "recall": 0.22862054146970348, "support": 2327}, "\u2205": {"f1-score": 0.5049833887043189, "precision": 1.0, "recall": 0.3377777777777778, "support": 1575}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 752}},
  "ppcr": 0.22862054146970348
}
```
</details>
