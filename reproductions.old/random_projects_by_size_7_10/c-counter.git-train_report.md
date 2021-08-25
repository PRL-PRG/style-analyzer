# Train report for javascript / file:///tmp/top-repos-quality-repos-j1xctrug/c-counter.git HEAD fdeabc2c1686484dafdae95b5cb0f33f59dfe504

### Classification report

PPCR: 0.394

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.976| 1.000| 0.610| 0.988| 0.751| 558| 915| 0.610 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 420| 0.031 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 116| 0.009 |
| `weighted avg` | 0.952| 0.976| 0.385| 0.963| 0.473| 572| 1451| 0.394 |
| `micro avg` | 0.976| 0.976| 0.385| 0.976| 0.552| 572| 1451| 0.394 |
| `macro avg` | 0.325| 0.333| 0.203| 0.329| 0.250| 572| 1451| 0.394 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|357 |558 |0 |0 |
|407 |13 |0 |0 |
|115 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/pages/index.js | 5 |
| src/components/header.js | 4 |
| gatsby-node.js | 1 |
| src/components/layout.js | 1 |
| gatsby-config.js | 1 |
| src/hooks/useTimeDifference.js | 1 |
| src/pages/404.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3292035398230089, "precision": 0.32517482517482516, "recall": 0.3333333333333333, "support": 572}, "micro avg": {"f1-score": 0.9755244755244755, "precision": 0.9755244755244755, "recall": 0.9755244755244755, "support": 572}, "weighted avg": {"f1-score": 0.9634383315799245, "precision": 0.9516480023473032, "recall": 0.9755244755244755, "support": 572}, "\u2205": {"f1-score": 0.9876106194690266, "precision": 0.9755244755244755, "recall": 1.0, "support": 558}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}},
  "cl_report_full": {"macro avg": {"f1-score": 0.250168123739072, "precision": 0.32517482517482516, "recall": 0.20327868852459016, "support": 1451}, "micro avg": {"f1-score": 0.5516559565002472, "precision": 0.9755244755244755, "recall": 0.38456237077877325, "support": 1451}, "weighted avg": {"f1-score": 0.47326774615007067, "precision": 0.6151653308786321, "recall": 0.38456237077877325, "support": 1451}, "\u2205": {"f1-score": 0.7505043712172159, "precision": 0.9755244755244755, "recall": 0.6098360655737705, "support": 915}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 420}},
  "ppcr": 0.39421088904204
}
```
</details>
