# Test report for javascript / file:///tmp/top-repos-quality-repos-kr10su2p/redux-mock-store.git HEAD b943c3ba0abf6d3e7f9918bd470525e85d166cff

### Classification report

PPCR: 0.242

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 1.000| 0.464| 0.990| 0.630| 51| 110| 0.464 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 63| 0.016 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 42| 0.000 |
| `macro avg` | 0.327| 0.333| 0.155| 0.330| 0.210| 52| 215| 0.242 |
| `weighted avg` | 0.962| 0.981| 0.237| 0.971| 0.322| 52| 215| 0.242 |
| `micro avg` | 0.981| 0.981| 0.237| 0.981| 0.382| 52| 215| 0.242 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|59 |51 |0 |0 |
|62 |1 |0 |0 |
|42 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.33009708737864074, "precision": 0.3269230769230769, "recall": 0.3333333333333333, "support": 52}, "micro avg": {"f1-score": 0.9807692307692307, "precision": 0.9807692307692307, "recall": 0.9807692307692307, "support": 52}, "weighted avg": {"f1-score": 0.9712471994025391, "precision": 0.9619082840236686, "recall": 0.9807692307692307, "support": 52}, "\u2205": {"f1-score": 0.9902912621359222, "precision": 0.9807692307692307, "recall": 1.0, "support": 51}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "macro avg": {"f1-score": 0.20987654320987656, "precision": 0.3269230769230769, "recall": 0.15454545454545454, "support": 215}, "micro avg": {"f1-score": 0.3820224719101124, "precision": 0.9807692307692307, "recall": 0.2372093023255814, "support": 215}, "weighted avg": {"f1-score": 0.3221360895779501, "precision": 0.501788908765653, "recall": 0.2372093023255814, "support": 215}, "\u2205": {"f1-score": 0.6296296296296297, "precision": 0.9807692307692307, "recall": 0.4636363636363636, "support": 110}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}},
  "ppcr": 0.24186046511627907
}
```
</details>
