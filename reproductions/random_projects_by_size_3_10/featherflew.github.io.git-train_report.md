# Train report for javascript / file:///tmp/top-repos-quality-repos-zhrgqjyr/featherflew.github.io.git HEAD 6c55e8fb2447c369eb92e28d939573551d4ed62f

### Classification report

PPCR: 0.291

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 1.000| 0.644| 0.990| 0.777| 1533| 2380| 0.644 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 1978| 0.010 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 321| 0.034 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 331| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 131| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 122| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 114| 0.000 |
| `weighted avg` | 0.961| 0.980| 0.285| 0.970| 0.344| 1564| 5377| 0.291 |
| `micro avg` | 0.980| 0.980| 0.285| 0.980| 0.442| 1564| 5377| 0.291 |
| `macro avg` | 0.140| 0.143| 0.092| 0.141| 0.111| 1564| 5377| 0.291 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|847 |1533 |0 |0 |0 |0 |0 |0 |
|1958 |20 |0 |0 |0 |0 |0 |0 |
|331 |0 |0 |0 |0 |0 |0 |0 |
|310 |11 |0 |0 |0 |0 |0 |0 |
|131 |0 |0 |0 |0 |0 |0 |0 |
|122 |0 |0 |0 |0 |0 |0 |0 |
|114 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/js/smoothscroll.js | 17 |
| gulpfile.js | 12 |
| src/js/classie.js | 1 |
| src/js/scrollanimation.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1414271876009041, "precision": 0.14002557544757033, "recall": 0.14285714285714285, "support": 1564}, "micro avg": {"f1-score": 0.9801790281329923, "precision": 0.9801790281329923, "recall": 0.9801790281329923, "support": 1564}, "weighted avg": {"f1-score": 0.9703677430596559, "precision": 0.9607509271917374, "recall": 0.9801790281329923, "support": 1564}, "\u2205": {"f1-score": 0.9899903132063287, "precision": 0.9801790281329923, "recall": 1.0, "support": 1533}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 331}, "macro avg": {"f1-score": 0.11105476673427993, "precision": 0.14002557544757033, "recall": 0.09201680672268908, "support": 5377}, "micro avg": {"f1-score": 0.4417230946549488, "precision": 0.9801790281329923, "recall": 0.2851032174074763, "support": 5377}, "weighted avg": {"f1-score": 0.3440900899745404, "precision": 0.4338527221418117, "recall": 0.2851032174074763, "support": 5377}, "\u2205": {"f1-score": 0.7773833671399595, "precision": 0.9801790281329923, "recall": 0.6441176470588236, "support": 2380}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 321}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 131}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1978}},
  "ppcr": 0.29086851404128694
}
```
</details>
