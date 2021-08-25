# Test report for javascript / file:///tmp/top-repos-quality-repos-h48wcfa1/coding-challenge-solutions.git HEAD 9d17e58b043d1d5443fa03ad935232494da1234c

### Classification report

PPCR: 0.947

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 0.976| 0.976| 0.958| 0.958| 1595| 1595| 1.000 |
| `␣` | 0.896| 0.949| 0.920| 0.922| 0.908| 824| 850| 0.969 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 104| 104| 1.000 |
| `⏎␣⁺␣⁺` | 0.983| 0.795| 0.795| 0.879| 0.879| 73| 73| 1.000 |
| `⏎␣⁻␣⁻` | 1.000| 0.588| 0.588| 0.741| 0.741| 68| 68| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 116| 0.422 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 76| 0.197 |
| `weighted avg` | 0.911| 0.931| 0.881| 0.919| 0.874| 2728| 2882| 0.947 |
| `micro avg` | 0.931| 0.931| 0.881| 0.931| 0.906| 2728| 2882| 0.947 |
| `macro avg` | 0.689| 0.615| 0.611| 0.643| 0.641| 2728| 2882| 0.947 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |1556 |39 |0 |0 |0 |0 |0 |
|26 |41 |782 |0 |0 |1 |0 |0 |
|0 |0 |0 |104 |0 |0 |0 |0 |
|67 |8 |41 |0 |0 |0 |0 |0 |
|0 |12 |3 |0 |0 |58 |0 |0 |
|0 |24 |4 |0 |0 |0 |40 |0 |
|61 |11 |4 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 104}, "macro avg": {"f1-score": 0.6427968826995788, "precision": 0.6886716012049877, "recall": 0.6153333653740196, "support": 2728}, "micro avg": {"f1-score": 0.9310850439882697, "precision": 0.9310850439882697, "recall": 0.9310850439882697, "support": 2728}, "weighted avg": {"f1-score": 0.9188515317734602, "precision": 0.9106241698012031, "recall": 0.9310850439882697, "support": 2728}, "\u2205": {"f1-score": 0.958423159839852, "precision": 0.9418886198547215, "recall": 0.9755485893416928, "support": 1595}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8787878787878787, "precision": 0.9830508474576272, "recall": 0.7945205479452054, "support": 73}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7407407407407407, "precision": 1.0, "recall": 0.5882352941176471, "support": 68}, "\u2423": {"f1-score": 0.9216263995285798, "precision": 0.8957617411225659, "recall": 0.9490291262135923, "support": 824}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 104}, "macro avg": {"f1-score": 0.6408101248529869, "precision": 0.6886716012049877, "recall": 0.6111863473435065, "support": 2882}, "micro avg": {"f1-score": 0.9055258467023173, "precision": 0.9310850439882697, "recall": 0.8813324080499653, "support": 2882}, "weighted avg": {"f1-score": 0.8739653212624987, "precision": 0.8700459890655339, "recall": 0.8813324080499653, "support": 2882}, "\u2205": {"f1-score": 0.958423159839852, "precision": 0.9418886198547215, "recall": 0.9755485893416928, "support": 1595}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8787878787878787, "precision": 0.9830508474576272, "recall": 0.7945205479452054, "support": 73}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7407407407407407, "precision": 1.0, "recall": 0.5882352941176471, "support": 68}, "\u2423": {"f1-score": 0.9077190946024377, "precision": 0.8957617411225659, "recall": 0.92, "support": 850}},
  "ppcr": 0.9465648854961832
}
```
</details>
