# Train report for javascript / file:///tmp/top-repos-quality-repos-_ktkqrw7/webproject.git HEAD d6340814eee651441527c45efad1d83db0a5c8a3

### Classification report

PPCR: 0.464

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.997| 0.504| 0.993| 0.667| 1140| 2258| 0.505 |
| `␣` | 0.985| 0.983| 0.401| 0.984| 0.570| 401| 982| 0.408 |
| `'` | 1.000| 1.000| 0.803| 1.000| 0.891| 339| 422| 0.803 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 96| 0.031 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 93| 0.032 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 222| 0.009 |
| `weighted avg` | 0.986| 0.990| 0.459| 0.988| 0.600| 1888| 4073| 0.464 |
| `micro avg` | 0.990| 0.990| 0.459| 0.990| 0.627| 1888| 4073| 0.464 |
| `macro avg` | 0.496| 0.497| 0.285| 0.496| 0.355| 1888| 4073| 0.464 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1118 |1137 |3 |0 |0 |0 |0 |
|581 |7 |394 |0 |0 |0 |0 |
|83 |0 |0 |339 |0 |0 |0 |
|220 |2 |0 |0 |0 |0 |0 |
|93 |0 |3 |0 |0 |0 |0 |
|90 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| routes/upload.js | 7 |
| app.js | 5 |
| bin/www | 5 |
| lib/async-error.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 339}, "macro avg": {"f1-score": 0.49620286786558304, "precision": 0.4957593559617058, "recall": 0.4966520103250645, "support": 1888}, "micro avg": {"f1-score": 0.9904661016949152, "precision": 0.9904661016949152, "recall": 0.9904661016949152, "support": 1888}, "weighted avg": {"f1-score": 0.9883587786034611, "precision": 0.986270654013807, "recall": 0.9904661016949152, "support": 1888}, "\u2205": {"f1-score": 0.9934469200524246, "precision": 0.9895561357702349, "recall": 0.9973684210526316, "support": 1140}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9837702871410736, "precision": 0.985, "recall": 0.9825436408977556, "support": 401}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8909329829172142, "precision": 1.0, "recall": 0.8033175355450237, "support": 422}, "macro avg": {"f1-score": 0.3547617475006462, "precision": 0.4957593559617058, "recall": 0.2846804149736572, "support": 4073}, "micro avg": {"f1-score": 0.6274115081362188, "precision": 0.9904661016949152, "recall": 0.45912104100171863, "support": 4073}, "weighted avg": {"f1-score": 0.5998033735859637, "precision": 0.8896851840336829, "recall": 0.45912104100171863, "support": 4073}, "\u2205": {"f1-score": 0.667449368946287, "precision": 0.9895561357702349, "recall": 0.5035429583702391, "support": 2258}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 222}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 93}, "\u2423": {"f1-score": 0.5701881331403762, "precision": 0.985, "recall": 0.40122199592668023, "support": 982}},
  "ppcr": 0.46354038792045177
}
```
</details>
