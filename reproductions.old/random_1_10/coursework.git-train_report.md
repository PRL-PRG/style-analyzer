# Train report for javascript / file:///tmp/top-repos-quality-repos-_65sehye/coursework.git HEAD 35eca505bda907082a222fc60e2af98890d145da

### Classification report

PPCR: 0.417

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.998| 0.816| 0.990| 0.891| 11241| 13744| 0.818 |
| `␣` | 0.934| 0.741| 0.055| 0.826| 0.104| 796| 10737| 0.074 |
| `⏎⏎⇥⁻` | 0.963| 0.963| 0.453| 0.963| 0.616| 161| 342| 0.471 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 123| 0.122 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 1044| 0.007 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 1463| 0.001 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 240| 0.004 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 580| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 339| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 209| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 201| 0.000 |
| `⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 140| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 150| 0.000 |
| `weighted avg` | 0.976| 0.979| 0.408| 0.977| 0.463| 12222| 29312| 0.417 |
| `micro avg` | 0.979| 0.979| 0.408| 0.979| 0.576| 12222| 29312| 0.417 |
| `macro avg` | 0.221| 0.208| 0.102| 0.214| 0.124| 12222| 29312| 0.417 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| '| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⇥| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2503 |11217 |24 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|9941 |203 |590 |0 |0 |0 |3 |0 |0 |0 |0 |0 |0 |0 |
|1462 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1037 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|580 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|181 |0 |6 |0 |0 |0 |155 |0 |0 |0 |0 |0 |0 |0 |
|339 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|239 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|209 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|201 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|140 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|150 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|108 |0 |12 |0 |0 |0 |3 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| cs314/A4/js/OBJLoader.js | 37 |
| cs314/A3/js/OBJLoader.js | 37 |
| cs314/A2/js/OBJLoader.js | 37 |
| cs314/A2/js/OrbitControls.js | 32 |
| cs314/A4/js/OrbitControls.js | 32 |
| cs314/A3/js/OrbitControls.js | 32 |
| cs314/A4/webvr/VREffect.js | 17 |
| cs314/A3/js/SourceLoader.js | 4 |
| cs314/A4/js/SourceLoader.js | 4 |
| cs314/A2/js/SourceLoader.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2137425552060304, "precision": 0.22136368605008017, "recall": 0.20783106984915276, "support": 12222}, "micro avg": {"f1-score": 0.9787268859433808, "precision": 0.9787268859433808, "recall": 0.9787268859433808, "support": 12222}, "weighted avg": {"f1-score": 0.9766599300947174, "precision": 0.9761568102642929, "recall": 0.9787268859433808, "support": 12222}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2205": {"f1-score": 0.9895897662108514, "precision": 0.9814506955989151, "recall": 0.9978649586335735, "support": 11241}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.9627329192546584, "precision": 0.9627329192546584, "recall": 0.9627329192546584, "support": 161}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8263305322128851, "precision": 0.9335443037974683, "recall": 0.7412060301507538, "support": 796}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 201}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 580}, "macro avg": {"f1-score": 0.12394508787257978, "precision": 0.22136368605008017, "recall": 0.10186957674433314, "support": 29312}, "micro avg": {"f1-score": 0.5760100158905956, "precision": 0.9787268859433808, "recall": 0.40809224890829693, "support": 29312}, "weighted avg": {"f1-score": 0.46307775869328044, "precision": 0.8133794421591839, "recall": 0.40809224890829693, "support": 29312}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u2205": {"f1-score": 0.891192944821833, "precision": 0.9814506955989151, "recall": 0.8161379511059371, "support": 13744}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1463}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 150}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1044}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 339}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.6163021868787276, "precision": 0.9627329192546584, "recall": 0.45321637426900585, "support": 342}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 240}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 209}, "\u2423": {"f1-score": 0.10379101064297651, "precision": 0.9335443037974683, "recall": 0.054950172301387726, "support": 10737}},
  "ppcr": 0.4169623362445415
}
```
</details>
