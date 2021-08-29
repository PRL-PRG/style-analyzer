# Train report for javascript / file:///tmp/top-repos-quality-repos-7pnoijn4/meteor-files.git HEAD 7f67a0885e120ad77085b191e3399add575a2d89

### Classification report

PPCR: 0.857

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.995| 0.970| 0.984| 0.971| 14374| 14746| 0.975 |
| `␣` | 0.970| 0.922| 0.791| 0.945| 0.871| 4040| 4708| 0.858 |
| `'` | 1.000| 1.000| 0.931| 1.000| 0.964| 1220| 1311| 0.931 |
| `⏎␣⁻␣⁻` | 0.983| 0.838| 0.695| 0.905| 0.814| 605| 729| 0.830 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 1039| 0.021 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 729| 0.012 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 385| 0.005 |
| `weighted avg` | 0.973| 0.974| 0.835| 0.973| 0.858| 20272| 23647| 0.857 |
| `macro avg` | 0.561| 0.536| 0.484| 0.548| 0.517| 20272| 23647| 0.857 |
| `micro avg` | 0.974| 0.974| 0.835| 0.974| 0.899| 20272| 23647| 0.857 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|372 |14301 |73 |0 |0 |0 |0 |0 |
|668 |308 |3723 |0 |0 |0 |9 |0 |
|91 |0 |0 |1220 |0 |0 |0 |0 |
|1017 |1 |21 |0 |0 |0 |0 |0 |
|720 |7 |2 |0 |0 |0 |0 |0 |
|124 |79 |19 |0 |0 |0 |507 |0 |
|383 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| server.js | 235 |
| upload.js | 120 |
| tests/helpers.js | 50 |
| core.js | 45 |
| lib.js | 42 |
| worker.js | 11 |
| write-stream.js | 8 |
| cursor.js | 7 |
| package.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1220}, "macro avg": {"f1-score": 0.5476494997310896, "precision": 0.5607977341786893, "recall": 0.5363532240323575, "support": 20272}, "micro avg": {"f1-score": 0.9742995264404104, "precision": 0.9742995264404104, "recall": 0.9742995264404104, "support": 20272}, "weighted avg": {"f1-score": 0.9731320543847678, "precision": 0.972728218113025, "recall": 0.9742995264404104, "support": 20272}, "\u2205": {"f1-score": 0.9838332416070447, "precision": 0.9729895223839978, "recall": 0.9949213858355364, "support": 14374}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9045495093666369, "precision": 0.9825581395348837, "recall": 0.8380165289256198, "support": 605}, "\u2423": {"f1-score": 0.9451637471439452, "precision": 0.9700364773319438, "recall": 0.9215346534653466, "support": 4040}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9640458316870801, "precision": 1.0, "recall": 0.9305873379099924, "support": 1311}, "macro avg": {"f1-score": 0.5173131166510554, "precision": 0.5607977341786893, "recall": 0.48380922312790087, "support": 23647}, "micro avg": {"f1-score": 0.8994284933627815, "precision": 0.9742995264404104, "recall": 0.8352433712521673, "support": 23647}, "weighted avg": {"f1-score": 0.8577801166166439, "precision": 0.8856057899976384, "recall": 0.8352433712521673, "support": 23647}, "\u2205": {"f1-score": 0.9714033419372368, "precision": 0.9729895223839978, "recall": 0.9698223246982233, "support": 14746}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1039}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 385}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 729}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8144578313253011, "precision": 0.9825581395348837, "recall": 0.6954732510288066, "support": 729}, "\u2423": {"f1-score": 0.8712848116077697, "precision": 0.9700364773319438, "recall": 0.7907816482582838, "support": 4708}},
  "ppcr": 0.857275764367573
}
```
</details>
