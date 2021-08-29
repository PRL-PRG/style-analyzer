# Test report for javascript / file:///tmp/top-repos-quality-repos-evym80ti/karma-webpack.git HEAD f734b109c7c970bf775c71553343ed8069899ed2

### Classification report

PPCR: 0.744

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 1.000| 0.938| 0.985| 0.954| 228| 243| 0.938 |
| `␣` | 1.000| 0.846| 0.493| 0.917| 0.660| 39| 67| 0.582 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 24| 48| 0.500 |
| `⏎␣⁺␣⁺` | 0.875| 0.933| 0.933| 0.903| 0.903| 15| 15| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 19| 0.053 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 15| 0.067 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 7| 0.000 |
| `weighted avg` | 0.965| 0.971| 0.722| 0.967| 0.777| 308| 414| 0.744 |
| `micro avg` | 0.971| 0.971| 0.722| 0.971| 0.828| 308| 414| 0.744 |
| `macro avg` | 0.549| 0.540| 0.409| 0.544| 0.455| 308| 414| 0.744 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|15 |228 |0 |0 |0 |0 |0 |0 |
|28 |4 |33 |0 |0 |2 |0 |0 |
|24 |0 |0 |24 |0 |0 |0 |0 |
|18 |1 |0 |0 |0 |0 |0 |0 |
|0 |1 |0 |0 |0 |14 |0 |0 |
|14 |1 |0 |0 |0 |0 |0 |0 |
|7 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 24}, "macro avg": {"f1-score": 0.5435390975173598, "precision": 0.5493161094224924, "recall": 0.5399267399267399, "support": 308}, "micro avg": {"f1-score": 0.9707792207792207, "precision": 0.9707792207792207, "recall": 0.9707792207792207, "support": 308}, "weighted avg": {"f1-score": 0.967049684621795, "precision": 0.9653685410334346, "recall": 0.9707792207792207, "support": 308}, "\u2205": {"f1-score": 0.9848812095032397, "precision": 0.9702127659574468, "recall": 1.0, "support": 228}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9032258064516129, "precision": 0.875, "recall": 0.9333333333333333, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9166666666666666, "precision": 1.0, "recall": 0.8461538461538461, "support": 39}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 48}, "macro avg": {"f1-score": 0.45483819550225274, "precision": 0.5493161094224924, "recall": 0.4091631788149201, "support": 414}, "micro avg": {"f1-score": 0.8282548476454293, "precision": 0.9707792207792207, "recall": 0.7222222222222222, "support": 414}, "weighted avg": {"f1-score": 0.7767736393197202, "precision": 0.8789533867817864, "recall": 0.7222222222222222, "support": 414}, "\u2205": {"f1-score": 0.9539748953974895, "precision": 0.9702127659574468, "recall": 0.9382716049382716, "support": 243}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9032258064516129, "precision": 0.875, "recall": 0.9333333333333333, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.66, "precision": 1.0, "recall": 0.4925373134328358, "support": 67}},
  "ppcr": 0.7439613526570048
}
```
</details>
