# Test report for javascript / file:///tmp/top-repos-quality-repos-ls1f1frh/prizey-web.git HEAD e8fa61071d69a295397656cd5e52645ba5782562

### Classification report

PPCR: 0.886

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.914| 0.954| 0.929| 0.934| 0.921| 3640| 3738| 0.974 |
| `␣` | 0.885| 0.887| 0.751| 0.886| 0.813| 1660| 1959| 0.847 |
| `'` | 0.970| 0.988| 0.988| 0.979| 0.979| 586| 586| 1.000 |
| `⏎␣⁻␣⁻` | 0.935| 0.717| 0.549| 0.811| 0.692| 180| 235| 0.766 |
| `⏎␣⁺␣⁺` | 0.846| 0.575| 0.358| 0.685| 0.503| 153| 246| 0.622 |
| `⏎` | 0.896| 0.527| 0.186| 0.663| 0.309| 131| 370| 0.354 |
| `⏎⏎` | 0.857| 0.600| 0.378| 0.706| 0.525| 80| 127| 0.630 |
| `"` | 1.000| 0.893| 0.893| 0.943| 0.943| 56| 56| 1.000 |
| `weighted avg` | 0.910| 0.911| 0.807| 0.908| 0.838| 6486| 7317| 0.886 |
| `micro avg` | 0.911| 0.911| 0.807| 0.911| 0.856| 6486| 7317| 0.886 |
| `macro avg` | 0.913| 0.768| 0.629| 0.826| 0.711| 6486| 7317| 0.886 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|98 |3473 |130 |12 |0 |16 |9 |0 |0 |
|299 |188 |1472 |0 |0 |0 |0 |0 |0 |
|0 |0 |7 |579 |0 |0 |0 |0 |0 |
|239 |40 |14 |0 |69 |0 |0 |8 |0 |
|93 |31 |34 |0 |0 |88 |0 |0 |0 |
|55 |46 |5 |0 |0 |0 |129 |0 |0 |
|47 |22 |2 |0 |8 |0 |0 |48 |0 |
|0 |0 |0 |6 |0 |0 |0 |0 |50 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9433962264150945, "precision": 1.0, "recall": 0.8928571428571429, "support": 56}, "\u0027": {"f1-score": 0.9788672865595943, "precision": 0.9698492462311558, "recall": 0.9880546075085325, "support": 586}, "macro avg": {"f1-score": 0.825879389510775, "precision": 0.9128244009204807, "recall": 0.7675409050062184, "support": 6486}, "micro avg": {"f1-score": 0.9108849830403947, "precision": 0.9108849830403947, "recall": 0.9108849830403947, "support": 6486}, "weighted avg": {"f1-score": 0.9079844098816282, "precision": 0.9101518755463185, "recall": 0.9108849830403947, "support": 6486}, "\u2205": {"f1-score": 0.9336021505376343, "precision": 0.9139473684210526, "recall": 0.9541208791208792, "support": 3640}, "\u23ce": {"f1-score": 0.6634615384615385, "precision": 0.8961038961038961, "recall": 0.5267175572519084, "support": 131}, "\u23ce\u23ce": {"f1-score": 0.7058823529411764, "precision": 0.8571428571428571, "recall": 0.6, "support": 80}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6848249027237354, "precision": 0.8461538461538461, "recall": 0.5751633986928104, "support": 153}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8113207547169811, "precision": 0.9347826086956522, "recall": 0.7166666666666667, "support": 180}, "\u2423": {"f1-score": 0.8856799037304453, "precision": 0.8846153846153846, "recall": 0.8867469879518072, "support": 1660}},
  "cl_report_full": {"\"": {"f1-score": 0.9433962264150945, "precision": 1.0, "recall": 0.8928571428571429, "support": 56}, "\u0027": {"f1-score": 0.9788672865595943, "precision": 0.9698492462311558, "recall": 0.9880546075085325, "support": 586}, "macro avg": {"f1-score": 0.71052193674665, "precision": 0.9128244009204807, "recall": 0.6290651239617469, "support": 7317}, "micro avg": {"f1-score": 0.8560457871477215, "precision": 0.9108849830403947, "recall": 0.8074347410140769, "support": 7317}, "weighted avg": {"f1-score": 0.8377529591292097, "precision": 0.9077315571304904, "recall": 0.8074347410140769, "support": 7317}, "\u2205": {"f1-score": 0.9214645794640488, "precision": 0.9139473684210526, "recall": 0.9291064740502942, "support": 3738}, "\u23ce": {"f1-score": 0.3087248322147651, "precision": 0.8961038961038961, "recall": 0.1864864864864865, "support": 370}, "\u23ce\u23ce": {"f1-score": 0.5245901639344263, "precision": 0.8571428571428571, "recall": 0.3779527559055118, "support": 127}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5028571428571429, "precision": 0.8461538461538461, "recall": 0.35772357723577236, "support": 246}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6916890080428953, "precision": 0.9347826086956522, "recall": 0.548936170212766, "support": 235}, "\u2423": {"f1-score": 0.8125862544852333, "precision": 0.8846153846153846, "recall": 0.7514037774374681, "support": 1959}},
  "ppcr": 0.8864288642886429
}
```
</details>
