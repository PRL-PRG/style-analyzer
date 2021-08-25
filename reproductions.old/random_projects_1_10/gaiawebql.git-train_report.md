# Train report for javascript / file:///tmp/top-repos-quality-repos-hcdgpqrv/gaiawebql.git HEAD 492e9b0c41ee95437d3fb6023be5169ee5755747

### Classification report

PPCR: 0.682

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 0.996| 0.960| 0.997| 0.978| 1896| 1968| 0.963 |
| `␣` | 0.963| 0.993| 0.862| 0.978| 0.909| 890| 1026| 0.867 |
| `⏎` | 0.923| 0.857| 0.120| 0.889| 0.212| 28| 200| 0.140 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 144| 0.125 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 96| 0.073 |
| `'` | 1.000| 1.000| 0.006| 1.000| 0.011| 1| 174| 0.006 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 416| 0.000 |
| `⏎␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 19| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 124| 0.000 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.985| 0.985| 0.671| 0.985| 0.799| 2840| 4167| 0.682 |
| `weighted avg` | 0.977| 0.985| 0.671| 0.981| 0.696| 2840| 4167| 0.682 |
| `macro avg` | 0.243| 0.240| 0.122| 0.241| 0.132| 2840| 4167| 0.682 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⏎| '| ⏎␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ␣␣| ␣␣␣| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|72 |1889 |7 |0 |0 |0 |0 |0 |0 |0 |
|136 |6 |884 |0 |0 |0 |0 |0 |0 |0 |
|416 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|172 |0 |4 |0 |24 |0 |0 |0 |0 |0 |
|89 |0 |5 |0 |2 |0 |0 |0 |0 |0 |
|173 |0 |0 |0 |0 |0 |1 |0 |0 |0 |
|126 |0 |18 |0 |0 |0 |0 |0 |0 |0 |
|19 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|124 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| htdocs/index.js | 18 |
| htdocs/gaiawebql/gaiawebql.js | 17 |
| htdocs/gaiawebql/reconnecting-websocket.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1}, "macro avg": {"f1-score": 0.24145848879520326, "precision": 0.24267960369543484, "recall": 0.24041933131167414, "support": 2840}, "micro avg": {"f1-score": 0.9852112676056339, "precision": 0.9852112676056338, "recall": 0.9852112676056338, "support": 2840}, "weighted avg": {"f1-score": 0.9808791932447122, "precision": 0.9767183185409429, "recall": 0.9852112676056338, "support": 2840}, "\u2205": {"f1-score": 0.9965708256396729, "precision": 0.9968337730870712, "recall": 0.9963080168776371, "support": 1896}, "\u23ce": {"f1-score": 0.888888888888889, "precision": 0.9230769230769231, "recall": 0.8571428571428571, "support": 28}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9778761061946903, "precision": 0.9629629629629629, "recall": 0.9932584269662922, "support": 890}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 416}, "\u0027": {"f1-score": 0.011428571428571429, "precision": 1.0, "recall": 0.005747126436781609, "support": 174}, "macro avg": {"f1-score": 0.13195495927558437, "precision": 0.24267960369543484, "recall": 0.12170020565998914, "support": 4167}, "micro avg": {"f1-score": 0.7986299414870843, "precision": 0.9852112676056338, "recall": 0.6714662826973842, "support": 4167}, "weighted avg": {"f1-score": 0.6964901431162147, "precision": 0.7939487041158485, "recall": 0.6714662826973842, "support": 4167}, "\u2205": {"f1-score": 0.9779963758736734, "precision": 0.9968337730870712, "recall": 0.9598577235772358, "support": 1968}, "\u23ce": {"f1-score": 0.21238938053097348, "precision": 0.9230769230769231, "recall": 0.12, "support": 200}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 144}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9094650205761317, "precision": 0.9629629629629629, "recall": 0.8615984405458089, "support": 1026}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "ppcr": 0.681545476361891
}
```
</details>
