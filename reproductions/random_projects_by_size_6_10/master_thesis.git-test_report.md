# Test report for javascript / file:///tmp/top-repos-quality-repos-joybpy87/master_thesis.git HEAD 80e4e61b65a1e67260cf5a1900606c7ccd6d9244

### Classification report

PPCR: 0.911

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.923| 0.938| 0.870| 0.930| 0.896| 64| 69| 0.928 |
| `␣` | 0.714| 0.667| 0.556| 0.690| 0.625| 15| 18| 0.833 |
| `⏎` | 1.000| 1.000| 1.000| 1.000| 1.000| 3| 3| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.888| 0.890| 0.811| 0.889| 0.845| 82| 90| 0.911 |
| `micro avg` | 0.890| 0.890| 0.811| 0.890| 0.849| 82| 90| 0.911 |
| `macro avg` | 0.527| 0.521| 0.485| 0.524| 0.504| 82| 90| 0.911 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|5 |60 |4 |0 |
|3 |5 |10 |0 |
|0 |0 |0 |3 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5239775461106656, "precision": 0.5274725274725275, "recall": 0.5208333333333333, "support": 82}, "micro avg": {"f1-score": 0.8902439024390244, "precision": 0.8902439024390244, "recall": 0.8902439024390244, "support": 82}, "weighted avg": {"f1-score": 0.8887769671602089, "precision": 0.887697668185473, "recall": 0.8902439024390244, "support": 82}, "\u2205": {"f1-score": 0.9302325581395349, "precision": 0.9230769230769231, "recall": 0.9375, "support": 64}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3}, "\u2423": {"f1-score": 0.689655172413793, "precision": 0.7142857142857143, "recall": 0.6666666666666666, "support": 15}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5041044776119403, "precision": 0.5274725274725275, "recall": 0.485024154589372, "support": 90}, "micro avg": {"f1-score": 0.8488372093023255, "precision": 0.8902439024390244, "recall": 0.8111111111111111, "support": 90}, "weighted avg": {"f1-score": 0.8449004975124379, "precision": 0.883882783882784, "recall": 0.8111111111111111, "support": 90}, "\u2205": {"f1-score": 0.8955223880597014, "precision": 0.9230769230769231, "recall": 0.8695652173913043, "support": 69}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3}, "\u2423": {"f1-score": 0.6250000000000001, "precision": 0.7142857142857143, "recall": 0.5555555555555556, "support": 18}},
  "ppcr": 0.9111111111111111
}
```
</details>
