# Train report for javascript / file:///tmp/top-repos-quality-repos-ymn6j1l9/takana.git HEAD 3e274d5990392590ce44b00f1c0acd680f98cf7b

### Classification report

PPCR: 0.335

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 1.000| 0.401| 0.977| 0.565| 172| 429| 0.401 |
| `␣` | 0.932| 0.986| 0.667| 0.958| 0.777| 69| 102| 0.676 |
| `⏎⏎` | 1.000| 0.556| 0.238| 0.714| 0.385| 9| 21| 0.429 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 57| 0.105 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 18| 0.111 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 101| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 21| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 21| 0.000 |
| `weighted avg` | 0.921| 0.950| 0.318| 0.933| 0.428| 258| 770| 0.335 |
| `macro avg` | 0.361| 0.318| 0.163| 0.331| 0.216| 258| 770| 0.335 |
| `micro avg` | 0.950| 0.950| 0.318| 0.950| 0.477| 258| 770| 0.335 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|257 |172 |0 |0 |0 |0 |0 |0 |0 |
|33 |1 |68 |0 |0 |0 |0 |0 |0 |
|101 |0 |0 |0 |0 |0 |0 |0 |0 |
|51 |5 |1 |0 |0 |0 |0 |0 |0 |
|16 |2 |0 |0 |0 |0 |0 |0 |0 |
|21 |0 |0 |0 |0 |0 |0 |0 |0 |
|12 |0 |4 |0 |0 |0 |0 |5 |0 |
|21 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| bin/takana | 9 |
| gulpfile.js | 2 |
| index.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3311631150539601, "precision": 0.360882800608828, "recall": 0.31763285024154586, "support": 258}, "micro avg": {"f1-score": 0.9496124031007752, "precision": 0.9496124031007752, "recall": 0.9496124031007752, "support": 258}, "weighted avg": {"f1-score": 0.9325735952005195, "precision": 0.9210446827840902, "recall": 0.9496124031007752, "support": 258}, "\u2205": {"f1-score": 0.9772727272727273, "precision": 0.9555555555555556, "recall": 1.0, "support": 172}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.7142857142857143, "precision": 1.0, "recall": 0.5555555555555556, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9577464788732394, "precision": 0.9315068493150684, "recall": 0.9855072463768116, "support": 69}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "macro avg": {"f1-score": 0.21582733358595427, "precision": 0.360882800608828, "recall": 0.16321178821178822, "support": 770}, "micro avg": {"f1-score": 0.47665369649805445, "precision": 0.9496124031007752, "recall": 0.3181818181818182, "support": 770}, "weighted avg": {"f1-score": 0.42814365929636866, "precision": 0.6830480934590524, "recall": 0.3181818181818182, "support": 770}, "\u2205": {"f1-score": 0.5648604269293924, "precision": 0.9555555555555556, "recall": 0.40093240093240096, "support": 429}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u23ce\u23ce": {"f1-score": 0.3846153846153846, "precision": 1.0, "recall": 0.23809523809523808, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.7771428571428571, "precision": 0.9315068493150684, "recall": 0.6666666666666666, "support": 102}},
  "ppcr": 0.33506493506493507
}
```
</details>
