# Test report for javascript / file:///tmp/top-repos-quality-repos-x13ev0qx/open-energy-view.git HEAD e4265dc4c240ec35d8fa4d3d3fc94c98e18276bf

### Classification report

PPCR: 0.844

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.878| 0.953| 0.881| 0.914| 0.879| 761| 823| 0.925 |
| `␣` | 0.859| 0.936| 0.874| 0.896| 0.867| 562| 602| 0.934 |
| `"` | 1.000| 0.842| 0.815| 0.914| 0.898| 120| 124| 0.968 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 57| 142| 0.401 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 49| 0.429 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 51| 0.216 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 33| 0.212 |
| `macro avg` | 0.391| 0.390| 0.367| 0.389| 0.378| 1539| 1824| 0.844 |
| `weighted avg` | 0.826| 0.878| 0.741| 0.850| 0.744| 1539| 1824| 0.844 |
| `micro avg` | 0.878| 0.878| 0.741| 0.878| 0.804| 1539| 1824| 0.844 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|62 |725 |36 |0 |0 |0 |0 |0 |
|40 |36 |526 |0 |0 |0 |0 |0 |
|85 |22 |35 |0 |0 |0 |0 |0 |
|4 |17 |2 |0 |101 |0 |0 |0 |
|40 |7 |4 |0 |0 |0 |0 |0 |
|28 |19 |2 |0 |0 |0 |0 |0 |
|26 |0 |7 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9140271493212669, "precision": 1.0, "recall": 0.8416666666666667, "support": 120}, "macro avg": {"f1-score": 0.3891117884322139, "precision": 0.39102872787533094, "recall": 0.3900433644401125, "support": 1539}, "micro avg": {"f1-score": 0.8784925276153346, "precision": 0.8784925276153346, "recall": 0.8784925276153346, "support": 1539}, "weighted avg": {"f1-score": 0.8502838347570301, "precision": 0.8258441102530666, "recall": 0.8784925276153346, "support": 1539}, "\u2205": {"f1-score": 0.9136735979836169, "precision": 0.8777239709443099, "recall": 0.9526938239159002, "support": 761}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.8960817717206132, "precision": 0.8594771241830066, "recall": 0.9359430604982206, "support": 562}},
  "cl_report_full": {"\"": {"f1-score": 0.8977777777777779, "precision": 1.0, "recall": 0.8145161290322581, "support": 124}, "macro avg": {"f1-score": 0.37766505930938876, "precision": 0.39102872787533094, "recall": 0.36702767609228165, "support": 1824}, "micro avg": {"f1-score": 0.8040440083258995, "precision": 0.8784925276153346, "recall": 0.7412280701754386, "support": 1824}, "weighted avg": {"f1-score": 0.7437898459754402, "precision": 0.7476820487090663, "recall": 0.7412280701754386, "support": 1824}, "\u2205": {"f1-score": 0.8793208004851425, "precision": 0.8777239709443099, "recall": 0.8809234507897934, "support": 823}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u2423": {"f1-score": 0.8665568369028007, "precision": 0.8594771241830066, "recall": 0.8737541528239202, "support": 602}},
  "ppcr": 0.84375
}
```
</details>
