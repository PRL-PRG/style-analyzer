# Train report for javascript / file:///tmp/top-repos-quality-repos-6w8ecjru/new_for_react.git HEAD 35abaf04b2759c3c09b38c4e86f1b2004c44b468

### Classification report

PPCR: 0.611

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.954| 0.997| 0.887| 0.975| 0.920| 11987| 13474| 0.890 |
| `␣` | 0.967| 0.817| 0.367| 0.885| 0.532| 2126| 4729| 0.450 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 101| 1810| 0.056 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 67| 370| 0.181 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 418| 0.067 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 254| 0.063 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1338| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1050| 0.000 |
| `micro avg` | 0.956| 0.956| 0.584| 0.956| 0.725| 14325| 23443| 0.611 |
| `weighted avg` | 0.942| 0.956| 0.584| 0.947| 0.636| 14325| 23443| 0.611 |
| `macro avg` | 0.240| 0.227| 0.157| 0.233| 0.181| 14325| 23443| 0.611 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1487 |11955 |32 |0 |0 |0 |0 |0 |0 |
|2603 |390 |1736 |0 |0 |0 |0 |0 |0 |
|1709 |97 |4 |0 |0 |0 |0 |0 |0 |
|1338 |0 |0 |0 |0 |0 |0 |0 |0 |
|1050 |0 |0 |0 |0 |0 |0 |0 |0 |
|390 |6 |22 |0 |0 |0 |0 |0 |0 |
|303 |67 |0 |0 |0 |0 |0 |0 |0 |
|238 |14 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| matrix.js | 186 |
| sankey.js | 99 |
| js/force_gai2.js | 50 |
| js/Sankey.js | 33 |
| js/radar_xz.js | 33 |
| js/Radar_Chart.js | 32 |
| js/radar.js | 32 |
| js/Rader_gai.js | 29 |
| js/community.js | 26 |
| js/tsne_gai.js | 25 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.23256800874517605, "precision": 0.24009733942865025, "recall": 0.22673591946308091, "support": 14325}, "micro avg": {"f1-score": 0.9557417102966841, "precision": 0.9557417102966841, "recall": 0.9557417102966841, "support": 14325}, "weighted avg": {"f1-score": 0.947488101908911, "precision": 0.9419062152264804, "recall": 0.9557417102966841, "support": 14325}, "\u2205": {"f1-score": 0.9752814488497308, "precision": 0.9541862878122755, "recall": 0.9973304413114207, "support": 11987}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u2423": {"f1-score": 0.8852626211116776, "precision": 0.9665924276169265, "recall": 0.8165569143932268, "support": 2126}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1050}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1338}, "macro avg": {"f1-score": 0.18145207088542542, "precision": 0.24009733942865025, "recall": 0.15679512484481362, "support": 23443}, "micro avg": {"f1-score": 0.725005295488244, "precision": 0.9557417102966841, "recall": 0.5840122851170926, "support": 23443}, "weighted avg": {"f1-score": 0.6358317392809041, "precision": 0.7434083364835151, "recall": 0.5840122851170926, "support": 23443}, "\u2205": {"f1-score": 0.9195092873899167, "precision": 0.9541862878122755, "recall": 0.8872643609915393, "support": 13474}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1810}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 254}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 418}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 370}, "\u2423": {"f1-score": 0.5321072796934867, "precision": 0.9665924276169265, "recall": 0.36709663776696977, "support": 4729}},
  "ppcr": 0.6110566053832701
}
```
</details>
