# Train report for javascript / file:///tmp/top-repos-quality-repos-bsd4vjjv/python-thermostat.git HEAD 8136fd5274bb770f164a1b03f7a49c2eb70f919e

### Classification report

PPCR: 0.603

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 0.998| 0.956| 0.970| 0.950| 2166| 2261| 0.958 |
| `␣` | 0.974| 0.680| 0.209| 0.801| 0.344| 281| 913| 0.308 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 356| 0.104 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 442| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 150| 0.000 |
| `macro avg` | 0.384| 0.335| 0.233| 0.354| 0.259| 2484| 4122| 0.603 |
| `micro avg` | 0.947| 0.947| 0.571| 0.947| 0.712| 2484| 4122| 0.603 |
| `weighted avg` | 0.934| 0.947| 0.571| 0.937| 0.597| 2484| 4122| 0.603 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|95 |2161 |5 |0 |0 |0 |
|632 |90 |191 |0 |0 |0 |
|442 |0 |0 |0 |0 |0 |
|319 |37 |0 |0 |0 |0 |
|150 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/temperature.js | 74 |
| static/schedule.js | 54 |
| static/playground.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.35424045848595337, "precision": 0.3837965605822749, "recall": 0.3354813799811385, "support": 2484}, "micro avg": {"f1-score": 0.9468599033816425, "precision": 0.9468599033816425, "recall": 0.9468599033816425, "support": 2484}, "weighted avg": {"f1-score": 0.9367324688464632, "precision": 0.9338178284218657, "recall": 0.9468599033816425, "support": 2484}, "\u2205": {"f1-score": 0.9703637180062865, "precision": 0.944493006993007, "recall": 0.9976915974145891, "support": 2166}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u2423": {"f1-score": 0.8008385744234802, "precision": 0.9744897959183674, "recall": 0.6797153024911032, "support": 281}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 150}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 442}, "macro avg": {"f1-score": 0.25891067726415956, "precision": 0.3837965605822749, "recall": 0.23299444410265405, "support": 4122}, "micro avg": {"f1-score": 0.7120799273387828, "precision": 0.9468599033816425, "recall": 0.5705967976710334, "support": 4122}, "weighted avg": {"f1-score": 0.5974431319018577, "precision": 0.7339174848337356, "recall": 0.5705967976710334, "support": 4122}, "\u2205": {"f1-score": 0.9500989228401847, "precision": 0.944493006993007, "recall": 0.9557717823971694, "support": 2261}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 356}, "\u2423": {"f1-score": 0.34445446348061315, "precision": 0.9744897959183674, "recall": 0.20920043811610076, "support": 913}},
  "ppcr": 0.6026200873362445
}
```
</details>
