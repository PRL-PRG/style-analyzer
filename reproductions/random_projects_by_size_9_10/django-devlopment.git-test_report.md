# Test report for javascript / file:///tmp/top-repos-quality-repos-v9cvlv5z/django-devlopment.git HEAD 445e6b65825fe03be34a13b30817adbb160bb608

### Classification report

PPCR: 0.798

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.878| 0.996| 0.927| 0.933| 0.902| 462| 496| 0.931 |
| `␣` | 0.981| 0.850| 0.543| 0.911| 0.699| 120| 188| 0.638 |
| `⏎` | 0.824| 0.700| 0.560| 0.757| 0.667| 40| 50| 0.800 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 24| 1.000 |
| `'` | 0.917| 1.000| 0.458| 0.957| 0.611| 22| 48| 0.458 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 30| 0.333 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 24| 0.333 |
| `micro avg` | 0.892| 0.892| 0.712| 0.892| 0.792| 686| 860| 0.798 |
| `weighted avg` | 0.840| 0.892| 0.712| 0.862| 0.746| 686| 860| 0.798 |
| `macro avg` | 0.514| 0.507| 0.355| 0.508| 0.411| 686| 860| 0.798 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|34 |460 |2 |0 |0 |0 |0 |0 |
|68 |16 |102 |0 |2 |0 |0 |0 |
|10 |12 |0 |28 |0 |0 |0 |0 |
|26 |0 |0 |0 |22 |0 |0 |0 |
|20 |8 |0 |2 |0 |0 |0 |0 |
|0 |22 |0 |2 |0 |0 |0 |0 |
|16 |6 |0 |2 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9565217391304348, "precision": 0.9166666666666666, "recall": 1.0, "support": 22}, "macro avg": {"f1-score": 0.5081508088465744, "precision": 0.5141182720886357, "recall": 0.5065244279529993, "support": 686}, "micro avg": {"f1-score": 0.892128279883382, "precision": 0.892128279883382, "recall": 0.892128279883382, "support": 686}, "weighted avg": {"f1-score": 0.8624992908553839, "precision": 0.8401933963754113, "recall": 0.892128279883382, "support": 686}, "\u2205": {"f1-score": 0.9330628803245437, "precision": 0.8778625954198473, "recall": 0.9956709956709957, "support": 462}, "\u23ce": {"f1-score": 0.7567567567567567, "precision": 0.8235294117647058, "recall": 0.7, "support": 40}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.9107142857142857, "precision": 0.9807692307692307, "recall": 0.85, "support": 120}},
  "cl_report_full": {"\u0027": {"f1-score": 0.611111111111111, "precision": 0.9166666666666666, "recall": 0.4583333333333333, "support": 48}, "macro avg": {"f1-score": 0.4111955284396864, "precision": 0.5141182720886357, "recall": 0.35547226852305785, "support": 860}, "micro avg": {"f1-score": 0.7917205692108668, "precision": 0.892128279883382, "recall": 0.7116279069767442, "support": 860}, "weighted avg": {"f1-score": 0.7457926528368595, "precision": 0.8197452712803429, "recall": 0.7116279069767442, "support": 860}, "\u2205": {"f1-score": 0.9019607843137254, "precision": 0.8778625954198473, "recall": 0.9274193548387096, "support": 496}, "\u23ce": {"f1-score": 0.6666666666666666, "precision": 0.8235294117647058, "recall": 0.56, "support": 50}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u2423": {"f1-score": 0.6986301369863014, "precision": 0.9807692307692307, "recall": 0.5425531914893617, "support": 188}},
  "ppcr": 0.7976744186046512
}
```
</details>
