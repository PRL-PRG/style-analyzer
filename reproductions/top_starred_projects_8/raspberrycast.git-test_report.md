# Test report for javascript / file:///tmp/top-repos-quality-repos-ibuxsox4/raspberrycast.git HEAD 1976dae9cfb5f17fe72acc7bf3ec9a7fe9b8ecd8

### Classification report

PPCR: 0.606

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.930| 0.968| 0.649| 0.949| 0.764| 248| 370| 0.670 |
| `␣` | 0.852| 0.561| 0.217| 0.676| 0.346| 41| 106| 0.387 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 40| 80| 0.500 |
| `⏎⇥⁻` | 0.926| 1.000| 1.000| 0.962| 0.962| 25| 25| 1.000 |
| `⏎⇥⁺` | 0.923| 1.000| 1.000| 0.960| 0.960| 24| 24| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 19| 0.000 |
| `weighted avg` | 0.928| 0.931| 0.564| 0.926| 0.673| 378| 624| 0.606 |
| `macro avg` | 0.772| 0.755| 0.561| 0.758| 0.616| 378| 624| 0.606 |
| `micro avg` | 0.931| 0.931| 0.564| 0.931| 0.703| 378| 624| 0.606 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎⇥⁺| ⏎| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|122 |240 |4 |0 |2 |0 |2 |
|65 |18 |23 |0 |0 |0 |0 |
|40 |0 |0 |40 |0 |0 |0 |
|0 |0 |0 |0 |24 |0 |0 |
|19 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |25 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 40}, "macro avg": {"f1-score": 0.7577709417607116, "precision": 0.7718478764990393, "recall": 0.7547862575399948, "support": 378}, "micro avg": {"f1-score": 0.9312169312169312, "precision": 0.9312169312169312, "recall": 0.9312169312169312, "support": 378}, "weighted avg": {"f1-score": 0.9261128906141696, "precision": 0.9283745890119706, "recall": 0.9312169312169312, "support": 378}, "\u2205": {"f1-score": 0.9486166007905139, "precision": 0.9302325581395349, "recall": 0.967741935483871, "support": 248}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207a": {"f1-score": 0.9600000000000001, "precision": 0.9230769230769231, "recall": 1.0, "support": 24}, "\u23ce\u21e5\u207b": {"f1-score": 0.9615384615384615, "precision": 0.9259259259259259, "recall": 1.0, "support": 25}, "\u2423": {"f1-score": 0.6764705882352942, "precision": 0.8518518518518519, "recall": 0.5609756097560976, "support": 41}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 80}, "macro avg": {"f1-score": 0.6164001666750577, "precision": 0.7718478764990393, "recall": 0.5609382967873534, "support": 624}, "micro avg": {"f1-score": 0.7025948103792415, "precision": 0.9312169312169312, "recall": 0.5641025641025641, "support": 624}, "weighted avg": {"f1-score": 0.6728781999644775, "precision": 0.8970902838299976, "recall": 0.5641025641025641, "support": 624}, "\u2205": {"f1-score": 0.764331210191083, "precision": 0.9302325581395349, "recall": 0.6486486486486487, "support": 370}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u21e5\u207a": {"f1-score": 0.9600000000000001, "precision": 0.9230769230769231, "recall": 1.0, "support": 24}, "\u23ce\u21e5\u207b": {"f1-score": 0.9615384615384615, "precision": 0.9259259259259259, "recall": 1.0, "support": 25}, "\u2423": {"f1-score": 0.3458646616541354, "precision": 0.8518518518518519, "recall": 0.2169811320754717, "support": 106}},
  "ppcr": 0.6057692307692307
}
```
</details>
