# Test report for javascript / file:///tmp/top-repos-quality-repos-h1ss_7yc/rov-gui.git HEAD a725c430eed689736c456696e2a69f1be8b8121c

### Classification report

PPCR: 0.912

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.752| 0.890| 0.869| 0.815| 0.806| 698| 715| 0.976 |
| `␣` | 0.640| 0.596| 0.555| 0.617| 0.594| 280| 301| 0.930 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 51| 77| 0.662 |
| `'` | 0.812| 1.000| 0.848| 0.897| 0.830| 39| 46| 0.848 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 36| 0.778 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 31| 0.742 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 28| 0.357 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 10| 0.600 |
| `macro avg` | 0.276| 0.311| 0.284| 0.291| 0.279| 1135| 1244| 0.912 |
| `micro avg` | 0.729| 0.729| 0.665| 0.729| 0.695| 1135| 1244| 0.912 |
| `weighted avg` | 0.648| 0.729| 0.665| 0.684| 0.638| 1135| 1244| 0.912 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|17 |621 |77 |0 |0 |0 |0 |0 |0 |
|21 |110 |167 |3 |0 |0 |0 |0 |0 |
|7 |0 |0 |39 |0 |0 |0 |0 |0 |
|26 |42 |9 |0 |0 |0 |0 |0 |0 |
|8 |23 |0 |0 |0 |0 |0 |0 |0 |
|8 |20 |8 |0 |0 |0 |0 |0 |0 |
|18 |10 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |6 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u0027": {"f1-score": 0.896551724137931, "precision": 0.8125, "recall": 1.0, "support": 39}, "macro avg": {"f1-score": 0.29111094813909943, "precision": 0.2755203404905699, "recall": 0.31076417314776916, "support": 1135}, "micro avg": {"f1-score": 0.7286343612334801, "precision": 0.7286343612334801, "recall": 0.7286343612334801, "support": 1135}, "weighted avg": {"f1-score": 0.6842934816047822, "precision": 0.6481164252000214, "recall": 0.7286343612334801, "support": 1135}, "\u2205": {"f1-score": 0.8149606299212597, "precision": 0.7518159806295399, "recall": 0.8896848137535817, "support": 698}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.6173752310536045, "precision": 0.6398467432950191, "recall": 0.5964285714285714, "support": 280}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.8297872340425533, "precision": 0.8125, "recall": 0.8478260869565217, "support": 46}, "macro avg": {"f1-score": 0.2787579291397936, "precision": 0.2755203404905699, "recall": 0.2838968539044373, "support": 1244}, "micro avg": {"f1-score": 0.6952501050861707, "precision": 0.7286343612334801, "recall": 0.6647909967845659, "support": 1244}, "weighted avg": {"f1-score": 0.6377210534395631, "precision": 0.6169753182330561, "recall": 0.6647909967845659, "support": 1244}, "\u2205": {"f1-score": 0.8059701492537312, "precision": 0.7518159806295399, "recall": 0.8685314685314686, "support": 715}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u2423": {"f1-score": 0.5943060498220639, "precision": 0.6398467432950191, "recall": 0.5548172757475083, "support": 301}},
  "ppcr": 0.912379421221865
}
```
</details>
