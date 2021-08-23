# Test report for javascript / file:///tmp/top-repos-quality-repos-d5nw4zup/kidprojects.git HEAD ecb7937431803842f4097c7782bcf87c69a66f82

### Classification report

PPCR: 0.833

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.976| 0.253| 0.212| 0.402| 0.349| 162| 193| 0.839 |
| `␣` | 0.258| 0.930| 0.870| 0.404| 0.398| 43| 46| 0.935 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 18| 0.944 |
| `"` | 0.150| 0.500| 0.500| 0.231| 0.231| 6| 6| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 8| 0.125 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.743| 0.367| 0.305| 0.366| 0.317| 229| 275| 0.833 |
| `micro avg` | 0.367| 0.367| 0.305| 0.367| 0.333| 229| 275| 0.833 |
| `macro avg` | 0.115| 0.140| 0.132| 0.086| 0.081| 229| 275| 0.833 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎⏎⇥⁻| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|31 |41 |112 |0 |0 |4 |0 |0 |5 |0 |
|3 |1 |40 |0 |0 |0 |0 |1 |1 |0 |
|7 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |2 |0 |3 |0 |0 |0 |1 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |17 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.23076923076923075, "precision": 0.15, "recall": 0.5, "support": 6}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "macro avg": {"f1-score": 0.0863975349269467, "precision": 0.11535458269329235, "recall": 0.1402765814910518, "support": 229}, "micro avg": {"f1-score": 0.3668122270742358, "precision": 0.36681222707423583, "recall": 0.36681222707423583, "support": 229}, "weighted avg": {"f1-score": 0.3662707415597218, "precision": 0.7429678224297185, "recall": 0.36681222707423583, "support": 229}, "\u2205": {"f1-score": 0.40196078431372556, "precision": 0.9761904761904762, "recall": 0.25308641975308643, "support": 162}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.404040404040404, "precision": 0.25806451612903225, "recall": 0.9302325581395349, "support": 43}},
  "cl_report_full": {"\"": {"f1-score": 0.23076923076923075, "precision": 0.15, "recall": 0.5, "support": 6}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "macro avg": {"f1-score": 0.08147627926922942, "precision": 0.11535458269329235, "recall": 0.13183337087932717, "support": 275}, "micro avg": {"f1-score": 0.33333333333333337, "precision": 0.36681222707423583, "recall": 0.3054545454545455, "support": 275}, "weighted avg": {"f1-score": 0.31650092344408, "precision": 0.7315481078061723, "recall": 0.3054545454545455, "support": 275}, "\u2205": {"f1-score": 0.34893617021276596, "precision": 0.9761904761904762, "recall": 0.21243523316062177, "support": 193}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.3980099502487562, "precision": 0.25806451612903225, "recall": 0.8695652173913043, "support": 46}},
  "ppcr": 0.8327272727272728
}
```
</details>
