# Test report for javascript / file:///tmp/top-repos-quality-repos-kkyut8m1/enquire.js.git HEAD 2f339f1e29b2b6676f541e64b770635075af494d

### Classification report

PPCR: 0.709

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.925| 1.000| 0.839| 0.961| 0.880| 99| 118| 0.839 |
| `␣` | 1.000| 0.759| 0.468| 0.863| 0.638| 29| 47| 0.617 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 10| 0.100 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 7| 0.000 |
| `micro avg` | 0.938| 0.938| 0.665| 0.938| 0.778| 129| 182| 0.709 |
| `macro avg` | 0.385| 0.352| 0.261| 0.365| 0.304| 129| 182| 0.709 |
| `weighted avg` | 0.935| 0.938| 0.665| 0.932| 0.735| 129| 182| 0.709 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|19 |99 |0 |0 |0 |
|18 |7 |22 |0 |0 |
|9 |1 |0 |0 |0 |
|7 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.364782029316581, "precision": 0.3850467289719626, "recall": 0.35172413793103446, "support": 129}, "micro avg": {"f1-score": 0.9379844961240311, "precision": 0.937984496124031, "recall": 0.937984496124031, "support": 129}, "weighted avg": {"f1-score": 0.9315887414648256, "precision": 0.9348692313265232, "recall": 0.937984496124031, "support": 129}, "\u2205": {"f1-score": 0.9611650485436893, "precision": 0.9252336448598131, "recall": 1.0, "support": 99}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8627450980392156, "precision": 1.0, "recall": 0.7586206896551724, "support": 29}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.303536231884058, "precision": 0.3850467289719626, "recall": 0.26141363144608726, "support": 182}, "micro avg": {"f1-score": 0.7781350482315113, "precision": 0.937984496124031, "recall": 0.6648351648351648, "support": 182}, "weighted avg": {"f1-score": 0.7352253543557892, "precision": 0.8581185169970217, "recall": 0.6648351648351648, "support": 182}, "\u2205": {"f1-score": 0.88, "precision": 0.9252336448598131, "recall": 0.8389830508474576, "support": 118}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.6376811594202899, "precision": 1.0, "recall": 0.46808510638297873, "support": 47}},
  "ppcr": 0.7087912087912088
}
```
</details>
