# Train report for javascript / file:///tmp/top-repos-quality-repos-9wjv15g2/github-buttons.git HEAD 4a0bfa75e18619804dfec24770e8add807130c7b

### Classification report

PPCR: 0.811

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.905| 0.998| 0.981| 0.949| 0.942| 5984| 6090| 0.983 |
| `'` | 1.000| 1.000| 0.943| 1.000| 0.971| 1465| 1554| 0.943 |
| `␣` | 0.993| 0.675| 0.343| 0.803| 0.509| 1423| 2802| 0.508 |
| `⏎␣⁺␣⁺` | 0.921| 0.723| 0.698| 0.810| 0.794| 642| 665| 0.965 |
| `⏎␣⁻␣⁻` | 0.894| 0.921| 0.767| 0.908| 0.826| 432| 519| 0.832 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 475| 0.078 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 198| 0.000 |
| `macro avg` | 0.673| 0.617| 0.533| 0.639| 0.577| 9983| 12303| 0.811 |
| `weighted avg` | 0.929| 0.928| 0.753| 0.922| 0.782| 9983| 12303| 0.811 |
| `micro avg` | 0.928| 0.928| 0.753| 0.928| 0.831| 9983| 12303| 0.811 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|106 |5975 |1 |0 |0 |8 |0 |0 |
|1379 |385 |960 |0 |39 |39 |0 |0 |
|89 |0 |0 |1465 |0 |0 |0 |0 |
|23 |175 |3 |0 |464 |0 |0 |0 |
|87 |34 |0 |0 |0 |398 |0 |0 |
|438 |33 |3 |0 |1 |0 |0 |0 |
|198 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/content.js | 70 |
| rollup.config.js | 65 |
| test/unit/content.spec.js | 61 |
| test/unit/fetch.spec.js | 50 |
| src/container.js | 38 |
| test/unit/util.spec.js | 35 |
| src/main.js | 33 |
| src/util.js | 31 |
| src/querystring.js | 30 |
| src/config.js | 29 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1465}, "macro avg": {"f1-score": 0.6386039639222659, "precision": 0.6732581198744169, "recall": 0.6167378256801938, "support": 9983}, "micro avg": {"f1-score": 0.9277772212761695, "precision": 0.9277772212761695, "recall": 0.9277772212761695, "support": 9983}, "weighted avg": {"f1-score": 0.921742197719557, "precision": 0.9286599155086042, "recall": 0.9277772212761695, "support": 9983}, "\u2205": {"f1-score": 0.9494676624821231, "precision": 0.905028779157831, "recall": 0.9984959893048129, "support": 5984}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8097731239092496, "precision": 0.9206349206349206, "recall": 0.7227414330218068, "support": 642}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9076396807297605, "precision": 0.8943820224719101, "recall": 0.9212962962962963, "support": 432}, "\u2423": {"f1-score": 0.803347280334728, "precision": 0.9927611168562565, "recall": 0.67463106113844, "support": 1423}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9705200397482611, "precision": 1.0, "recall": 0.9427284427284427, "support": 1554}, "macro avg": {"f1-score": 0.5772919987387964, "precision": 0.6732581198744169, "recall": 0.5330087361128285, "support": 12303}, "micro avg": {"f1-score": 0.8311944718657452, "precision": 0.9277772212761695, "recall": 0.7528245143460944, "support": 12303}, "weighted avg": {"f1-score": 0.7824109859149342, "precision": 0.8878930672508789, "recall": 0.7528245143460944, "support": 12303}, "\u2205": {"f1-score": 0.9415379766782225, "precision": 0.905028779157831, "recall": 0.9811165845648604, "support": 6090}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 475}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 198}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7938408896492729, "precision": 0.9206349206349206, "recall": 0.6977443609022557, "support": 665}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8257261410788381, "precision": 0.8943820224719101, "recall": 0.766859344894027, "support": 519}, "\u2423": {"f1-score": 0.5094189440169806, "precision": 0.9927611168562565, "recall": 0.3426124197002141, "support": 2802}},
  "ppcr": 0.8114281069657807
}
```
</details>
