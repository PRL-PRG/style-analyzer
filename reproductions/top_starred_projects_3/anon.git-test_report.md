# Test report for javascript / file:///tmp/top-repos-quality-repos-1sampocj/anon.git HEAD 95d886e1b068cc1b16e6605b35a1814263724280

### Classification report

PPCR: 0.790

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.659| 0.990| 0.833| 0.791| 0.736| 700| 832| 0.841 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 359| 483| 0.743 |
| `'` | 0.833| 1.000| 0.500| 0.909| 0.625| 35| 70| 0.500 |
| `weighted avg` | 0.448| 0.665| 0.526| 0.535| 0.474| 1094| 1385| 0.790 |
| `micro avg` | 0.665| 0.665| 0.526| 0.665| 0.587| 1094| 1385| 0.790 |
| `macro avg` | 0.497| 0.663| 0.444| 0.567| 0.454| 1094| 1385| 0.790 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|132 |693 |0 |7 |
|124 |359 |0 |0 |
|35 |0 |0 |35 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9090909090909091, "precision": 0.8333333333333334, "recall": 1.0, "support": 35}, "macro avg": {"f1-score": 0.5667289331672893, "precision": 0.49735952682720747, "recall": 0.6633333333333333, "support": 1094}, "micro avg": {"f1-score": 0.6654478976234004, "precision": 0.6654478976234004, "recall": 0.6654478976234004, "support": 1094}, "weighted avg": {"f1-score": 0.5352699315409992, "precision": 0.44816118799860055, "recall": 0.6654478976234004, "support": 1094}, "\u2205": {"f1-score": 0.791095890410959, "precision": 0.658745247148289, "recall": 0.99, "support": 700}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 359}},
  "cl_report_full": {"\u0027": {"f1-score": 0.625, "precision": 0.8333333333333334, "recall": 0.5, "support": 70}, "macro avg": {"f1-score": 0.4535562632696391, "precision": 0.49735952682720747, "recall": 0.4443108974358974, "support": 1385}, "micro avg": {"f1-score": 0.5873336022589756, "precision": 0.6654478976234004, "recall": 0.5256317689530686, "support": 1385}, "weighted avg": {"f1-score": 0.4735208903400861, "precision": 0.43784070683083737, "recall": 0.5256317689530686, "support": 1385}, "\u2205": {"f1-score": 0.7356687898089174, "precision": 0.658745247148289, "recall": 0.8329326923076923, "support": 832}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 483}},
  "ppcr": 0.7898916967509025
}
```
</details>
