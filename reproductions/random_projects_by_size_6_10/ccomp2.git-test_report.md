# Test report for javascript / file:///tmp/top-repos-quality-repos-bqn647s0/ccomp2.git HEAD 46b4acc705d33433cc8797c2e19844827b2f34e1

### Classification report

PPCR: 0.830

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.957| 1.000| 0.936| 0.978| 0.946| 88| 94| 0.936 |
| `'` | 1.000| 0.550| 0.550| 0.710| 0.710| 40| 40| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 22| 0.455 |
| `␣` | 0.263| 1.000| 0.833| 0.417| 0.400| 10| 12| 0.833 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 6| 1.000 |
| `⏎␣⁻␣⁻` | 0.500| 1.000| 0.333| 0.667| 0.400| 2| 6| 0.333 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 8| 0.000 |
| `weighted avg` | 0.819| 0.782| 0.649| 0.769| 0.662| 156| 188| 0.830 |
| `micro avg` | 0.782| 0.782| 0.649| 0.782| 0.709| 156| 188| 0.830 |
| `macro avg` | 0.340| 0.444| 0.332| 0.346| 0.307| 156| 188| 0.830 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|6 |88 |0 |0 |0 |0 |0 |0 |
|2 |0 |10 |0 |0 |0 |0 |0 |
|12 |0 |10 |0 |0 |0 |0 |0 |
|0 |2 |14 |0 |22 |0 |2 |0 |
|0 |2 |4 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |2 |0 |
|8 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.7096774193548387, "precision": 1.0, "recall": 0.55, "support": 40}, "macro avg": {"f1-score": 0.3463485663082437, "precision": 0.33995995423340963, "recall": 0.44375, "support": 156}, "micro avg": {"f1-score": 0.782051282051282, "precision": 0.782051282051282, "recall": 0.782051282051282, "support": 156}, "weighted avg": {"f1-score": 0.768791930888705, "precision": 0.8192659743002993, "recall": 0.782051282051282, "support": 156}, "\u2205": {"f1-score": 0.9777777777777777, "precision": 0.9565217391304348, "recall": 1.0, "support": 88}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6666666666666666, "precision": 0.5, "recall": 1.0, "support": 2}, "\u2423": {"f1-score": 0.4166666666666667, "precision": 0.2631578947368421, "recall": 1.0, "support": 10}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.7096774193548387, "precision": 1.0, "recall": 0.55, "support": 40}, "macro avg": {"f1-score": 0.3069892473118279, "precision": 0.33995995423340963, "recall": 0.3316046099290781, "support": 188}, "micro avg": {"f1-score": 0.7093023255813954, "precision": 0.782051282051282, "recall": 0.648936170212766, "support": 188}, "weighted avg": {"f1-score": 0.6624113475177305, "precision": 0.7237815862505478, "recall": 0.648936170212766, "support": 188}, "\u2205": {"f1-score": 0.9462365591397849, "precision": 0.9565217391304348, "recall": 0.9361702127659575, "support": 94}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.4, "precision": 0.5, "recall": 0.3333333333333333, "support": 6}, "\u2423": {"f1-score": 0.39999999999999997, "precision": 0.2631578947368421, "recall": 0.8333333333333334, "support": 12}},
  "ppcr": 0.8297872340425532
}
```
</details>
