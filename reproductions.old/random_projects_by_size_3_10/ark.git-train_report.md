# Train report for javascript / file:///tmp/top-repos-quality-repos-crg004cm/ark.git HEAD 045166a89f29f2f5b3435692cddc14968117719a

### Classification report

PPCR: 0.621

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.996| 0.968| 0.979| 0.966| 7319| 7531| 0.972 |
| `␣` | 0.978| 0.894| 0.446| 0.934| 0.612| 2162| 4336| 0.499 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.986| 0.936| 0.908| 0.961| 0.946| 455| 469| 0.970 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.964| 0.964| 0.914| 0.964| 0.938| 450| 475| 0.947 |
| `⏎` | 0.966| 0.804| 0.208| 0.877| 0.342| 280| 1082| 0.259 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 126| 0.008 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2892| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 257| 0.000 |
| `weighted avg` | 0.967| 0.967| 0.601| 0.966| 0.652| 10667| 17168| 0.621 |
| `micro avg` | 0.967| 0.967| 0.601| 0.967| 0.741| 10667| 17168| 0.621 |
| `macro avg` | 0.607| 0.574| 0.430| 0.589| 0.475| 10667| 17168| 0.621 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|212 |7293 |19 |0 |0 |6 |1 |0 |0 |
|2174 |218 |1932 |0 |0 |7 |5 |0 |0 |
|2892 |0 |0 |0 |0 |0 |0 |0 |0 |
|802 |30 |22 |0 |225 |3 |0 |0 |0 |
|25 |11 |3 |0 |2 |434 |0 |0 |0 |
|14 |24 |0 |0 |5 |0 |426 |0 |0 |
|257 |0 |0 |0 |0 |0 |0 |0 |0 |
|125 |0 |0 |0 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/admin/js/admin/DateTimeShortcuts.js | 52 |
| static/admin/js/SelectFilter2.js | 47 |
| static/admin/js/core.js | 38 |
| static/admin/js/inlines.js | 34 |
| static/admin/js/SelectBox.js | 28 |
| static/admin/js/calendar.js | 26 |
| static/admin/js/actions.js | 26 |
| static/admin/js/admin/RelatedObjectLookups.js | 23 |
| static/admin/js/timeparse.js | 17 |
| static/admin/js/urlify.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5894022246568321, "precision": 0.6070748475603842, "recall": 0.5742930290859555, "support": 10667}, "micro avg": {"f1-score": 0.9665322958657542, "precision": 0.9665322958657542, "recall": 0.9665322958657542, "support": 10667}, "weighted avg": {"f1-score": 0.965844939652053, "precision": 0.9667690359088034, "recall": 0.9665322958657542, "support": 10667}, "\u2205": {"f1-score": 0.9792547834843908, "precision": 0.9626451953537487, "recall": 0.9964476021314387, "support": 7319}, "\u23ce": {"f1-score": 0.8771929824561404, "precision": 0.9656652360515021, "recall": 0.8035714285714286, "support": 280}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9644444444444444, "precision": 0.9644444444444444, "recall": 0.9644444444444444, "support": 450}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9605411499436303, "precision": 0.9861111111111112, "recall": 0.9362637362637363, "support": 455}, "\u2423": {"f1-score": 0.9337844369260512, "precision": 0.9777327935222672, "recall": 0.8936170212765957, "support": 2162}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 257}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2892}, "macro avg": {"f1-score": 0.4754849575546597, "precision": 0.6070748475603842, "recall": 0.43048965830835634, "support": 17168}, "micro avg": {"f1-score": 0.7407939644332676, "precision": 0.9665322958657542, "recall": 0.6005358807082946, "support": 17168}, "weighted avg": {"f1-score": 0.6515099699633171, "precision": 0.7837009183685684, "recall": 0.6005358807082946, "support": 17168}, "\u2205": {"f1-score": 0.9655126762428013, "precision": 0.9626451953537487, "recall": 0.9683972911963883, "support": 7531}, "\u23ce": {"f1-score": 0.34220532319391633, "precision": 0.9656652360515021, "recall": 0.20794824399260628, "support": 1082}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 126}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9383783783783785, "precision": 0.9644444444444444, "recall": 0.9136842105263158, "support": 475}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9456159822419534, "precision": 0.9861111111111112, "recall": 0.908315565031983, "support": 469}, "\u2423": {"f1-score": 0.612167300380228, "precision": 0.9777327935222672, "recall": 0.4455719557195572, "support": 4336}},
  "ppcr": 0.6213303821062441
}
```
</details>
