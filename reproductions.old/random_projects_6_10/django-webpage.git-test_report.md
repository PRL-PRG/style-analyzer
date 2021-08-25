# Test report for javascript / file:///tmp/top-repos-quality-repos-1n4rdwaq/django-webpage.git HEAD bfeebd3080481690b8736fbd5fe5e8003912a5b2

### Classification report

PPCR: 0.892

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.883| 0.980| 0.947| 0.929| 0.914| 147| 152| 0.967 |
| `␣` | 0.882| 0.698| 0.545| 0.779| 0.674| 43| 55| 0.782 |
| `"` | 1.000| 0.846| 0.500| 0.917| 0.667| 13| 22| 0.591 |
| `⏎` | 1.000| 0.500| 0.417| 0.667| 0.588| 10| 12| 0.833 |
| `⏎␣⁺␣⁺` | 1.000| 1.000| 1.000| 1.000| 1.000| 10| 10| 1.000 |
| `⏎␣⁻␣⁻` | 1.000| 1.000| 1.000| 1.000| 1.000| 9| 9| 1.000 |
| `micro avg` | 0.901| 0.901| 0.804| 0.901| 0.850| 232| 260| 0.892 |
| `weighted avg` | 0.904| 0.901| 0.804| 0.895| 0.834| 232| 260| 0.892 |
| `macro avg` | 0.961| 0.837| 0.735| 0.882| 0.807| 232| 260| 0.892 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|5 |144 |3 |0 |0 |0 |0 |
|12 |13 |30 |0 |0 |0 |0 |
|9 |2 |0 |11 |0 |0 |0 |
|2 |4 |1 |0 |5 |0 |0 |
|0 |0 |0 |0 |0 |10 |0 |
|0 |0 |0 |0 |0 |0 |9 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9166666666666666, "precision": 1.0, "recall": 0.8461538461538461, "support": 13}, "macro avg": {"f1-score": 0.8819310617697713, "precision": 0.9609647539997593, "recall": 0.8372366835821986, "support": 232}, "micro avg": {"f1-score": 0.9008620689655172, "precision": 0.9008620689655172, "recall": 0.9008620689655172, "support": 232}, "weighted avg": {"f1-score": 0.8950757274797875, "precision": 0.9043370997648054, "recall": 0.9008620689655172, "support": 232}, "\u2205": {"f1-score": 0.9290322580645161, "precision": 0.8834355828220859, "recall": 0.9795918367346939, "support": 147}, "\u23ce": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9}, "\u2423": {"f1-score": 0.7792207792207793, "precision": 0.8823529411764706, "recall": 0.6976744186046512, "support": 43}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 22}, "macro avg": {"f1-score": 0.8072241630734691, "precision": 0.9609647539997593, "recall": 0.7349149388623072, "support": 260}, "micro avg": {"f1-score": 0.8495934959349594, "precision": 0.9008620689655172, "recall": 0.8038461538461539, "support": 260}, "weighted avg": {"f1-score": 0.8337521940496163, "precision": 0.9069677705910113, "recall": 0.8038461538461539, "support": 260}, "\u2205": {"f1-score": 0.9142857142857141, "precision": 0.8834355828220859, "recall": 0.9473684210526315, "support": 152}, "\u23ce": {"f1-score": 0.5882352941176471, "precision": 1.0, "recall": 0.4166666666666667, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9}, "\u2423": {"f1-score": 0.6741573033707865, "precision": 0.8823529411764706, "recall": 0.5454545454545454, "support": 55}},
  "ppcr": 0.8923076923076924
}
```
</details>
