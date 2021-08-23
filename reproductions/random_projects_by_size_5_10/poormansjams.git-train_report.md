# Train report for javascript / file:///tmp/top-repos-quality-repos-kvy31774/poormansjams.git HEAD ef46a2c4789383ebe02890b691614279a7336561

### Classification report

PPCR: 0.475

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 0.998| 0.808| 0.993| 0.889| 7260| 8971| 0.809 |
| `␣` | 0.959| 0.934| 0.133| 0.947| 0.234| 625| 4389| 0.142 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 155| 0.181 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 286| 0.059 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 952| 0.007 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 303| 0.017 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 92| 0.011 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1065| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 196| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 163| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 164| 0.000 |
| `macro avg` | 0.177| 0.176| 0.086| 0.176| 0.102| 7943| 16736| 0.475 |
| `weighted avg` | 0.979| 0.986| 0.468| 0.982| 0.538| 7943| 16736| 0.475 |
| `micro avg` | 0.986| 0.986| 0.468| 0.986| 0.635| 7943| 16736| 0.475 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1711 |7246 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3764 |41 |584 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1065 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|945 |1 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|298 |0 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|269 |17 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|196 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|163 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|164 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|127 |28 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|91 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| PoorMansJamsApp/static/jscolor.js | 82 |
| PoorMansJamsApp/static/script.js | 30 |
| PoorMansJamsApp/static/auth.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.17632056576815075, "precision": 0.17699547160818707, "recall": 0.1756792386676684, "support": 7943}, "micro avg": {"f1-score": 0.9857736371647992, "precision": 0.9857736371647992, "recall": 0.9857736371647992, "support": 7943}, "weighted avg": {"f1-score": 0.9821013121743777, "precision": 0.978500705631392, "recall": 0.9857736371647992, "support": 7943}, "\u2205": {"f1-score": 0.9930108263670001, "precision": 0.9880010908099264, "recall": 0.9980716253443526, "support": 7260}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u2423": {"f1-score": 0.9465153970826581, "precision": 0.9589490968801314, "recall": 0.9344, "support": 625}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 196}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1065}, "macro avg": {"f1-score": 0.10204550834300431, "precision": 0.17699547160818707, "recall": 0.08552487880188675, "support": 16736}, "micro avg": {"f1-score": 0.6345475910693302, "precision": 0.9857736371647992, "recall": 0.46785372848948376, "support": 16736}, "weighted avg": {"f1-score": 0.5377132705180836, "precision": 0.7810818219325255, "recall": 0.46785372848948376, "support": 16736}, "\u2205": {"f1-score": 0.8888071143820913, "precision": 0.9880010908099264, "recall": 0.8077137442871475, "support": 8971}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 952}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 303}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 286}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u2423": {"f1-score": 0.23369347739095636, "precision": 0.9589490968801314, "recall": 0.13305992253360674, "support": 4389}},
  "ppcr": 0.47460564053537285
}
```
</details>
