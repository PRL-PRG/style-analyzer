# Test report for javascript / file:///tmp/top-repos-quality-repos-b9nhoqam/mailor.git HEAD 069eece82d49974ed848e26daca641b2fec24868

### Classification report

PPCR: 0.868

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.969| 0.921| 0.971| 0.946| 783| 824| 0.950 |
| `␣` | 0.882| 0.949| 0.859| 0.914| 0.870| 276| 305| 0.905 |
| `'` | 1.000| 0.909| 0.769| 0.952| 0.870| 44| 52| 0.846 |
| `⏎␣⁺␣⁺` | 1.000| 0.912| 0.775| 0.954| 0.873| 34| 40| 0.850 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 43| 0.279 |
| `⏎⏎` | 0.786| 1.000| 0.289| 0.880| 0.423| 11| 38| 0.289 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 36| 0.056 |
| `micro avg` | 0.949| 0.949| 0.824| 0.949| 0.882| 1162| 1338| 0.868 |
| `macro avg` | 0.663| 0.677| 0.516| 0.667| 0.569| 1162| 1338| 0.868 |
| `weighted avg` | 0.940| 0.949| 0.824| 0.944| 0.853| 1162| 1338| 0.868 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|41 |759 |24 |0 |0 |0 |0 |0 |
|29 |13 |262 |0 |0 |0 |0 |1 |
|8 |2 |2 |40 |0 |0 |0 |0 |
|31 |2 |8 |0 |0 |0 |0 |2 |
|6 |2 |1 |0 |0 |31 |0 |0 |
|34 |2 |0 |0 |0 |0 |0 |0 |
|27 |0 |0 |0 |0 |0 |0 |11 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9523809523809523, "precision": 1.0, "recall": 0.9090909090909091, "support": 44}, "macro avg": {"f1-score": 0.667417355010428, "precision": 0.6629922987065845, "recall": 0.6770685194708478, "support": 1162}, "micro avg": {"f1-score": 0.9492254733218589, "precision": 0.9492254733218589, "recall": 0.9492254733218589, "support": 1162}, "weighted avg": {"f1-score": 0.9439503018195107, "precision": 0.9397907361332491, "recall": 0.9492254733218589, "support": 1162}, "\u2205": {"f1-score": 0.9712092130518234, "precision": 0.9730769230769231, "recall": 0.9693486590038314, "support": 783}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.88, "precision": 0.7857142857142857, "recall": 1.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9538461538461539, "precision": 1.0, "recall": 0.9117647058823529, "support": 34}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9144851657940662, "precision": 0.8821548821548821, "recall": 0.9492753623188406, "support": 276}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8695652173913044, "precision": 1.0, "recall": 0.7692307692307693, "support": 52}, "macro avg": {"f1-score": 0.5689567872394147, "precision": 0.6629922987065845, "recall": 0.5162624788197554, "support": 1338}, "micro avg": {"f1-score": 0.8824, "precision": 0.9492254733218589, "recall": 0.8243647234678625, "support": 1338}, "weighted avg": {"f1-score": 0.85315849648626, "precision": 0.8914273292449675, "recall": 0.8243647234678625, "support": 1338}, "\u2205": {"f1-score": 0.9463840399002493, "precision": 0.9730769230769231, "recall": 0.9211165048543689, "support": 824}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u23ce": {"f1-score": 0.423076923076923, "precision": 0.7857142857142857, "recall": 0.2894736842105263, "support": 38}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8732394366197184, "precision": 1.0, "recall": 0.775, "support": 40}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.8704318936877077, "precision": 0.8821548821548821, "recall": 0.8590163934426229, "support": 305}},
  "ppcr": 0.8684603886397608
}
```
</details>
