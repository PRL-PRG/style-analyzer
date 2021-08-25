# Train report for javascript / file:///tmp/top-repos-quality-repos-sjpkm6xa/pmmt-py.git HEAD 300d596c4c2458b61ecf4226507b23c502eadbe1

### Classification report

PPCR: 0.823

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.954| 0.971| 0.962| 0.963| 0.958| 3600| 3633| 0.991 |
| `␣` | 0.928| 0.956| 0.840| 0.942| 0.882| 1668| 1898| 0.879 |
| `'` | 0.968| 1.000| 0.840| 0.984| 0.899| 483| 575| 0.840 |
| `⏎` | 0.875| 0.365| 0.079| 0.515| 0.145| 96| 444| 0.216 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 175| 0.120 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 180| 0.100 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 92| 0.174 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 121| 0.091 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 39| 0.128 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 39| 0.103 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.310| 0.274| 0.227| 0.284| 0.240| 5922| 7196| 0.823 |
| `micro avg` | 0.947| 0.947| 0.779| 0.947| 0.855| 5922| 7196| 0.823 |
| `weighted avg` | 0.935| 0.947| 0.779| 0.939| 0.797| 5922| 7196| 0.823 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|33 |3496 |104 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|230 |68 |1595 |0 |5 |0 |0 |0 |0 |0 |0 |0 |
|92 |0 |0 |483 |0 |0 |0 |0 |0 |0 |0 |0 |
|348 |54 |7 |0 |35 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|76 |0 |0 |16 |0 |0 |0 |0 |0 |0 |0 |0 |
|110 |11 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|162 |13 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|154 |18 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|35 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|34 |3 |0 |0 |0 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| analise_criminal/static/analise_criminal/styledMarkers.js | 116 |
| analise_criminal/static/analise_criminal/map.js | 72 |
| analise_criminal/static/analise_criminal/draggable.js | 54 |
| static/js/setup-lat-lng.js | 34 |
| static/js/setup_lat_lng.js | 26 |
| static/js/headerScript.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u0027": {"f1-score": 0.9837067209775967, "precision": 0.9679358717434869, "recall": 1.0, "support": 483}, "macro avg": {"f1-score": 0.2836012322895427, "precision": 0.3104791623656332, "recall": 0.27432745470290437, "support": 5922}, "micro avg": {"f1-score": 0.9471462343802769, "precision": 0.9471462343802769, "recall": 0.9471462343802769, "support": 5922}, "weighted avg": {"f1-score": 0.9391530879718316, "precision": 0.9348134086140801, "recall": 0.9471462343802769, "support": 5922}, "\u2205": {"f1-score": 0.962687594657855, "precision": 0.9544089544089545, "recall": 0.9711111111111111, "support": 3600}, "\u23ce": {"f1-score": 0.5147058823529411, "precision": 0.875, "recall": 0.3645833333333333, "support": 96}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9421145894861193, "precision": 0.9284051222351571, "recall": 0.9562350119904077, "support": 1668}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u0027": {"f1-score": 0.8994413407821229, "precision": 0.9679358717434869, "recall": 0.84, "support": 575}, "macro avg": {"f1-score": 0.24038275323131128, "precision": 0.3104791623656332, "recall": 0.2267897682544527, "support": 7196}, "micro avg": {"f1-score": 0.8551608476901966, "precision": 0.9471462343802769, "recall": 0.7794608115619789, "support": 7196}, "weighted avg": {"f1-score": 0.7973062494269849, "precision": 0.8580522206257039, "recall": 0.7794608115619789, "support": 7196}, "\u2205": {"f1-score": 0.9583333333333333, "precision": 0.9544089544089545, "recall": 0.9622901183594825, "support": 3633}, "\u23ce": {"f1-score": 0.14462809917355374, "precision": 0.875, "recall": 0.07882882882882883, "support": 444}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 175}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8821902654867255, "precision": 0.9284051222351571, "recall": 0.8403582718651211, "support": 1898}},
  "ppcr": 0.8229571984435797
}
```
</details>
