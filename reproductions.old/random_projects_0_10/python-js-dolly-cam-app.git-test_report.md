# Test report for javascript / file:///tmp/top-repos-quality-repos-v9ta4xr5/python-js-dolly-cam-app.git HEAD beeec04d8f47555a4ebd98f4b59d153dce57a8d2

### Classification report

PPCR: 0.837

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.865| 0.916| 0.887| 0.890| 0.876| 154| 159| 0.969 |
| `␣` | 0.724| 0.764| 0.646| 0.743| 0.683| 55| 65| 0.846 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 6| 0.833 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 7| 0.429 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 6| 0.500 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 5| 0.200 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 10| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.828| 0.828| 0.693| 0.828| 0.755| 221| 264| 0.837 |
| `weighted avg` | 0.783| 0.828| 0.693| 0.805| 0.696| 221| 264| 0.837 |
| `macro avg` | 0.159| 0.168| 0.153| 0.163| 0.156| 221| 264| 0.837 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5 |141 |13 |0 |0 |0 |0 |0 |0 |
|10 |13 |42 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |0 |0 |
|10 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |1 |2 |0 |0 |0 |0 |0 |0 |
|4 |1 |0 |0 |0 |0 |0 |0 |0 |
|1 |4 |1 |0 |0 |0 |0 |0 |0 |
|3 |3 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16329527372211833, "precision": 0.15891686058811086, "recall": 0.16792207792207794, "support": 221}, "micro avg": {"f1-score": 0.8280542986425339, "precision": 0.8280542986425339, "recall": 0.8280542986425339, "support": 221}, "weighted avg": {"f1-score": 0.8048950279551127, "precision": 0.7829968784311174, "recall": 0.8280542986425339, "support": 221}, "\u2205": {"f1-score": 0.889589905362776, "precision": 0.8650306748466258, "recall": 0.9155844155844156, "support": 154}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.7433628318584072, "precision": 0.7241379310344828, "recall": 0.7636363636363637, "support": 55}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "macro avg": {"f1-score": 0.15587032267838208, "precision": 0.15891686058811086, "recall": 0.15329462989840348, "support": 264}, "micro avg": {"f1-score": 0.754639175257732, "precision": 0.8280542986425339, "recall": 0.6931818181818182, "support": 264}, "weighted avg": {"f1-score": 0.6956011026795756, "precision": 0.6992759197646017, "recall": 0.6931818181818182, "support": 264}, "\u2205": {"f1-score": 0.875776397515528, "precision": 0.8650306748466258, "recall": 0.8867924528301887, "support": 159}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.6829268292682927, "precision": 0.7241379310344828, "recall": 0.6461538461538462, "support": 65}},
  "ppcr": 0.8371212121212122
}
```
</details>
