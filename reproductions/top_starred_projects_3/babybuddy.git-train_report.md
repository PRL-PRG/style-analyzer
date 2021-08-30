# Train report for javascript / file:///tmp/top-repos-quality-repos-pqm27xi6/babybuddy.git HEAD 4de006783ef9462d06d1af3f50d8a3244c1df5fa

### Classification report

PPCR: 0.759

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.996| 0.978| 0.989| 0.980| 19710| 20082| 0.981 |
| `␣` | 0.966| 0.978| 0.607| 0.972| 0.746| 6938| 11171| 0.621 |
| `'` | 0.997| 1.000| 0.765| 0.999| 0.866| 5637| 7368| 0.765 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.910| 0.872| 0.654| 0.891| 0.761| 815| 1087| 0.750 |
| `⏎` | 0.965| 0.760| 0.201| 0.850| 0.333| 721| 2726| 0.264 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 106| 227| 0.467 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 68| 1093| 0.062 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 297| 0.054 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 241| 0.058 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 535| 0.015 |
| `weighted avg` | 0.973| 0.979| 0.743| 0.976| 0.806| 34033| 44827| 0.759 |
| `micro avg` | 0.979| 0.979| 0.743| 0.979| 0.845| 34033| 44827| 0.759 |
| `macro avg` | 0.482| 0.461| 0.321| 0.470| 0.369| 34033| 44827| 0.759 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|372 |19641 |69 |0 |0 |0 |0 |0 |0 |0 |0 |
|4233 |146 |6784 |0 |0 |0 |8 |0 |0 |0 |0 |
|1731 |0 |0 |5637 |0 |0 |0 |0 |0 |0 |0 |
|2005 |90 |83 |0 |548 |0 |0 |0 |0 |0 |0 |
|1025 |16 |48 |0 |4 |0 |0 |0 |0 |0 |0 |
|272 |69 |25 |0 |10 |0 |711 |0 |0 |0 |0 |
|281 |0 |0 |16 |0 |0 |0 |0 |0 |0 |0 |
|527 |0 |6 |0 |2 |0 |0 |0 |0 |0 |0 |
|227 |6 |8 |0 |0 |0 |0 |0 |0 |0 |0 |
|121 |38 |2 |0 |4 |0 |62 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| gulpfile.js | 53 |
| static/rest_framework/docs/js/api.c9743eab7a4f.js | 49 |
| static/rest_framework/docs/js/api.js | 49 |
| static/admin/js/inlines.7596b7fd289e.js | 45 |
| static/admin/js/inlines.js | 45 |
| static/admin/js/admin/DateTimeShortcuts.js | 37 |
| static/admin/js/admin/DateTimeShortcuts.5548f99471bf.js | 37 |
| static/admin/js/SelectFilter2.d250dcb52a9a.js | 29 |
| static/admin/js/SelectFilter2.js | 29 |
| static/admin/js/core.ccd84108ec57.js | 24 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u0027": {"f1-score": 0.9985828166519043, "precision": 0.9971696444365823, "recall": 1.0, "support": 5637}, "macro avg": {"f1-score": 0.470061515308668, "precision": 0.48197791191889, "recall": 0.4606750757060527, "support": 34033}, "micro avg": {"f1-score": 0.9790791290805982, "precision": 0.9790791290805982, "recall": 0.9790791290805982, "support": 34033}, "weighted avg": {"f1-score": 0.9756574966147719, "precision": 0.9728497515001867, "recall": 0.9790791290805982, "support": 34033}, "\u2205": {"f1-score": 0.9890724141403968, "precision": 0.9817554733579926, "recall": 0.9964992389649924, "support": 19710}, "\u23ce": {"f1-score": 0.8502715283165244, "precision": 0.9647887323943662, "recall": 0.7600554785020804, "support": 721}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8909774436090226, "precision": 0.910371318822023, "recall": 0.8723926380368098, "support": 815}, "\u2423": {"f1-score": 0.9717109503688319, "precision": 0.9656939501779359, "recall": 0.9778034015566446, "support": 6938}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 297}, "\u0027": {"f1-score": 0.8658321173488979, "precision": 0.9971696444365823, "recall": 0.7650651465798045, "support": 7368}, "macro avg": {"f1-score": 0.36853528753087017, "precision": 0.48197791191889, "recall": 0.3205512889235475, "support": 44827}, "micro avg": {"f1-score": 0.8450672077098657, "precision": 0.9790791290805982, "recall": 0.7433243357797756, "support": 44827}, "weighted avg": {"f1-score": 0.8058071790246278, "precision": 0.9251146447830157, "recall": 0.7433243357797756, "support": 44827}, "\u2205": {"f1-score": 0.9798942326880862, "precision": 0.9817554733579926, "recall": 0.9780400358530027, "support": 20082}, "\u23ce": {"f1-score": 0.33272616879174255, "precision": 0.9647887323943662, "recall": 0.20102714600146734, "support": 2726}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 535}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 241}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1093}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 227}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7612419700214133, "precision": 0.910371318822023, "recall": 0.6540938362465502, "support": 1087}, "\u2423": {"f1-score": 0.7456583864585622, "precision": 0.9656939501779359, "recall": 0.6072867245546504, "support": 11171}},
  "ppcr": 0.7592076204073438
}
```
</details>
