# Train report for javascript / file:///tmp/top-repos-quality-repos-v71wo3xd/startbootstrap-clean-blog.git HEAD 55a51512302795e49d5a3f8b6c8efc95431448fb

### Classification report

PPCR: 0.044

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.973| 1.000| 0.140| 0.987| 0.244| 256| 1835| 0.140 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 3827| 0.002 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 291| 0.000 |
| `weighted avg` | 0.947| 0.973| 0.043| 0.960| 0.075| 263| 5953| 0.044 |
| `micro avg` | 0.973| 0.973| 0.043| 0.973| 0.082| 263| 5953| 0.044 |
| `macro avg` | 0.324| 0.333| 0.047| 0.329| 0.081| 263| 5953| 0.044 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|3820 |0 |7 |0 |
|1579 |0 |256 |0 |
|291 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/assets/mail/jqBootstrapValidation.js | 5 |
| scripts/render-scss.js | 1 |
| scripts/render-scripts.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3288375080282595, "precision": 0.3244613434727503, "recall": 0.3333333333333333, "support": 263}, "micro avg": {"f1-score": 0.973384030418251, "precision": 0.973384030418251, "recall": 0.973384030418251, "support": 263}, "weighted avg": {"f1-score": 0.9602555367517236, "precision": 0.9474764706732784, "recall": 0.973384030418251, "support": 263}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.9865125240847785, "precision": 0.973384030418251, "recall": 1.0, "support": 256}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 291}, "macro avg": {"f1-score": 0.08134731490308232, "precision": 0.3244613434727503, "recall": 0.04650317892824705, "support": 5953}, "micro avg": {"f1-score": 0.08236808236808238, "precision": 0.973384030418251, "recall": 0.043003527633126155, "support": 5953}, "weighted avg": {"f1-score": 0.07522542727053051, "precision": 0.30004362436040494, "recall": 0.043003527633126155, "support": 5953}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3827}, "\u2423": {"f1-score": 0.24404194470924695, "precision": 0.973384030418251, "recall": 0.13950953678474115, "support": 1835}},
  "ppcr": 0.04417940534184445
}
```
</details>
