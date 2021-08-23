# Test report for javascript / file:///tmp/top-repos-quality-repos-mhwba1tl/guys-night-out-big2.git HEAD dcc31501fa91e7e7c1159559399f18f8dd5389a6

### Classification report

PPCR: 0.760

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.900| 0.857| 0.783| 0.878| 0.837| 21| 23| 0.913 |
| `'` | 1.000| 0.714| 0.500| 0.833| 0.667| 7| 10| 0.700 |
| `␣` | 0.636| 1.000| 0.636| 0.778| 0.636| 7| 11| 0.636 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 1.000| 1.000| 1.000| 1.000| 2| 2| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 2| 0.500 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `micro avg` | 0.842| 0.842| 0.640| 0.842| 0.727| 38| 50| 0.760 |
| `weighted avg` | 0.851| 0.842| 0.640| 0.835| 0.698| 38| 50| 0.760 |
| `macro avg` | 0.505| 0.510| 0.417| 0.498| 0.449| 38| 50| 0.760 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2 |18 |3 |0 |0 |0 |0 |0 |
|4 |0 |7 |0 |0 |0 |0 |0 |
|1 |0 |1 |0 |0 |0 |0 |0 |
|3 |2 |0 |0 |5 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |2 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8333333333333333, "precision": 1.0, "recall": 0.7142857142857143, "support": 7}, "macro avg": {"f1-score": 0.49845141308555935, "precision": 0.5051948051948052, "recall": 0.5102040816326531, "support": 38}, "micro avg": {"f1-score": 0.8421052631578947, "precision": 0.8421052631578947, "recall": 0.8421052631578947, "support": 38}, "weighted avg": {"f1-score": 0.8346526886321494, "precision": 0.8514354066985647, "recall": 0.8421052631578947, "support": 38}, "\u2205": {"f1-score": 0.8780487804878048, "precision": 0.9, "recall": 0.8571428571428571, "support": 21}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2}, "\u2423": {"f1-score": 0.7777777777777778, "precision": 0.6363636363636364, "recall": 1.0, "support": 7}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 10}, "macro avg": {"f1-score": 0.44860565790798346, "precision": 0.5051948051948052, "recall": 0.4169960474308301, "support": 50}, "micro avg": {"f1-score": 0.7272727272727272, "precision": 0.8421052631578947, "recall": 0.64, "support": 50}, "weighted avg": {"f1-score": 0.6984496124031009, "precision": 0.794, "recall": 0.64, "support": 50}, "\u2205": {"f1-score": 0.8372093023255814, "precision": 0.9, "recall": 0.782608695652174, "support": 23}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2}, "\u2423": {"f1-score": 0.6363636363636364, "precision": 0.6363636363636364, "recall": 0.6363636363636364, "support": 11}},
  "ppcr": 0.76
}
```
</details>
