# Train report for javascript / file:///tmp/top-repos-quality-repos-ak4ld70i/weatherstation-project.git HEAD 4df3967867dacf430f636b03b56f4b5c4b3d21a9

### Classification report

PPCR: 0.275

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 1.000| 0.344| 0.976| 0.505| 745| 2168| 0.344 |
| `'` | 1.000| 1.000| 0.568| 1.000| 0.724| 222| 391| 0.568 |
| `⏎` | 0.980| 0.919| 0.412| 0.948| 0.580| 160| 357| 0.448 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 135| 0.089 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 153| 0.052 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 994| 0.007 |
| `macro avg` | 0.489| 0.486| 0.221| 0.487| 0.302| 1154| 4198| 0.275 |
| `weighted avg` | 0.943| 0.965| 0.265| 0.954| 0.378| 1154| 4198| 0.275 |
| `micro avg` | 0.965| 0.965| 0.265| 0.965| 0.416| 1154| 4198| 0.275 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1423 |745 |0 |0 |0 |0 |0 |
|987 |4 |0 |3 |0 |0 |0 |
|197 |13 |0 |147 |0 |0 |0 |
|169 |0 |0 |0 |222 |0 |0 |
|145 |8 |0 |0 |0 |0 |0 |
|123 |12 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| now-application/api/index.js | 20 |
| now-application/src/serviceWorker.js | 13 |
| now-application/src/App.js | 5 |
| now-application/api/models/data.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 222}, "macro avg": {"f1-score": 0.4873594299033173, "precision": 0.48878090366581417, "recall": 0.4864583333333334, "support": 1154}, "micro avg": {"f1-score": 0.9653379549393414, "precision": 0.9653379549393414, "recall": 0.9653379549393414, "support": 1154}, "weighted avg": {"f1-score": 0.9538043328032282, "precision": 0.9432847828303199, "recall": 0.9653379549393414, "support": 1154}, "\u2205": {"f1-score": 0.9757694826457105, "precision": 0.9526854219948849, "recall": 1.0, "support": 745}, "\u23ce": {"f1-score": 0.9483870967741935, "precision": 0.98, "recall": 0.91875, "support": 160}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7243066884176182, "precision": 1.0, "recall": 0.5677749360613811, "support": 391}, "macro avg": {"f1-score": 0.3015455151641773, "precision": 0.48878090366581417, "recall": 0.22052905471509957, "support": 4198}, "micro avg": {"f1-score": 0.4162929745889387, "precision": 0.9653379549393414, "recall": 0.2653644592663173, "support": 4198}, "weighted avg": {"f1-score": 0.37761919853838455, "precision": 0.6684807038792069, "recall": 0.2653644592663173, "support": 4198}, "\u2205": {"f1-score": 0.5050847457627119, "precision": 0.9526854219948849, "recall": 0.34363468634686345, "support": 2168}, "\u23ce": {"f1-score": 0.5798816568047338, "precision": 0.98, "recall": 0.4117647058823529, "support": 357}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 153}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 135}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 994}},
  "ppcr": 0.274892806098142
}
```
</details>
