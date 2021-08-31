# Train report for javascript / file:///tmp/top-repos-quality-repos-20nfz900/real-time-person-removal.git HEAD bf5d194cf2579fd62e7ceb5dd7b5548d1f64cbf0

### Classification report

PPCR: 0.418

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 0.954| 0.468| 0.970| 0.635| 474| 965| 0.491 |
| `␣` | 0.935| 0.981| 0.418| 0.957| 0.577| 321| 754| 0.426 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 183| 0.000 |
| `macro avg` | 0.641| 0.645| 0.295| 0.642| 0.404| 795| 1902| 0.418 |
| `weighted avg` | 0.966| 0.965| 0.403| 0.965| 0.551| 795| 1902| 0.418 |
| `micro avg` | 0.965| 0.965| 0.403| 0.965| 0.569| 795| 1902| 0.418 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|491 |452 |22 |0 |
|433 |6 |315 |0 |
|183 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| script_original.js | 15 |
| script.js | 13 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.6424679633519009, "precision": 0.640539221402995, "recall": 0.6449649697017495, "support": 795}, "micro avg": {"f1-score": 0.9647798742138366, "precision": 0.9647798742138365, "recall": 0.9647798742138365, "support": 795}, "weighted avg": {"f1-score": 0.9649057637538055, "precision": 0.9658300671683736, "recall": 0.9647798742138365, "support": 795}, "\u2205": {"f1-score": 0.9699570815450644, "precision": 0.9868995633187773, "recall": 0.9535864978902954, "support": 474}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9574468085106382, "precision": 0.9347181008902077, "recall": 0.9813084112149533, "support": 321}},
  "cl_report_full": {"macro avg": {"f1-score": 0.4042431538607045, "precision": 0.640539221402995, "recall": 0.29538855522418145, "support": 1902}, "micro avg": {"f1-score": 0.5687801260659993, "precision": 0.9647798742138365, "recall": 0.4032597266035752, "support": 1902}, "weighted avg": {"f1-score": 0.5512311166959055, "precision": 0.8712594777464966, "recall": 0.4032597266035752, "support": 1902}, "\u2205": {"f1-score": 0.6352775825720309, "precision": 0.9868995633187773, "recall": 0.46839378238341967, "support": 965}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 183}, "\u2423": {"f1-score": 0.5774518790100825, "precision": 0.9347181008902077, "recall": 0.4177718832891247, "support": 754}},
  "ppcr": 0.41798107255520506
}
```
</details>
