# Test report for javascript / file:///tmp/top-repos-quality-repos-1gnwvhfj/apachecn-algo-zh.git HEAD 71862d95dd7e1e34c8e193b3ebf1063172a2cbf0

### Classification report

PPCR: 0.197

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 60| 412| 0.146 |
| `␣` | 0.379| 0.846| 0.180| 0.524| 0.244| 39| 183| 0.213 |
| `'` | 0.731| 0.905| 0.500| 0.809| 0.594| 21| 38| 0.553 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 42| 0.310 |
| `macro avg` | 0.278| 0.438| 0.170| 0.333| 0.210| 133| 675| 0.197 |
| `weighted avg` | 0.227| 0.391| 0.077| 0.281| 0.100| 133| 675| 0.197 |
| `micro avg` | 0.391| 0.391| 0.077| 0.391| 0.129| 133| 675| 0.197 |

### Confusion matrix

|refusal|  '| ␣| ∅| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|17 |19 |2 |0 |0 |
|144 |0 |33 |0 |6 |
|352 |6 |40 |0 |14 |
|29 |1 |12 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8085106382978723, "precision": 0.7307692307692307, "recall": 0.9047619047619048, "support": 21}, "macro avg": {"f1-score": 0.33308004052684903, "precision": 0.27751989389920423, "recall": 0.43772893772893773, "support": 133}, "micro avg": {"f1-score": 0.39097744360902253, "precision": 0.39097744360902253, "recall": 0.39097744360902253, "support": 133}, "weighted avg": {"f1-score": 0.2812578558859154, "precision": 0.22661095710097523, "recall": 0.39097744360902253, "support": 133}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 60}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.5238095238095238, "precision": 0.3793103448275862, "recall": 0.8461538461538461, "support": 39}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5937499999999999, "precision": 0.7307692307692307, "recall": 0.5, "support": 38}, "macro avg": {"f1-score": 0.2095486111111111, "precision": 0.27751989389920423, "recall": 0.17008196721311475, "support": 675}, "micro avg": {"f1-score": 0.12871287128712872, "precision": 0.39097744360902253, "recall": 0.07703703703703704, "support": 675}, "weighted avg": {"f1-score": 0.09969753086419753, "precision": 0.14397485018174672, "recall": 0.07703703703703704, "support": 675}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 412}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u2423": {"f1-score": 0.24444444444444446, "precision": 0.3793103448275862, "recall": 0.18032786885245902, "support": 183}},
  "ppcr": 0.19703703703703704
}
```
</details>
