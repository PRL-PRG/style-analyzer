# Train report for javascript / file:///tmp/top-repos-quality-repos-d8svls6c/vue-cnodejs.git HEAD b23c152b13839da2bafa7388780f17e433d4a4ce

### Classification report

PPCR: 0.594

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 1.000| 0.811| 0.993| 0.890| 1620| 1998| 0.811 |
| `'` | 1.000| 1.000| 0.762| 1.000| 0.865| 443| 581| 0.762 |
| `␣` | 1.000| 0.810| 0.163| 0.895| 0.280| 248| 1236| 0.201 |
| `⏎` | 0.929| 0.992| 0.312| 0.959| 0.467| 118| 375| 0.315 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.920| 0.989| 0.893| 0.953| 0.906| 93| 103| 0.903 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.911| 1.000| 0.852| 0.953| 0.880| 92| 108| 0.852 |
| `weighted avg` | 0.982| 0.981| 0.583| 0.981| 0.679| 2614| 4401| 0.594 |
| `micro avg` | 0.981| 0.981| 0.583| 0.981| 0.731| 2614| 4401| 0.594 |
| `macro avg` | 0.958| 0.965| 0.632| 0.959| 0.715| 2614| 4401| 0.594 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|378 |1620 |0 |0 |0 |0 |0 |
|988 |21 |201 |0 |9 |9 |8 |
|138 |0 |0 |443 |0 |0 |0 |
|257 |1 |0 |0 |117 |0 |0 |
|16 |0 |0 |0 |0 |92 |0 |
|10 |1 |0 |0 |0 |0 |92 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/libs/utils.js | 11 |
| build/utils.js | 7 |
| build/webpack.prod.conf.js | 7 |
| build/dev-server.js | 6 |
| build/webpack.dev.conf.js | 4 |
| src/routers.js | 4 |
| build/check-versions.js | 3 |
| src/main.js | 3 |
| src/vuex/user.js | 2 |
| build/webpack.base.conf.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 443}, "macro avg": {"f1-score": 0.9590043927400105, "precision": 0.9575772891609655, "recall": 0.965209434420752, "support": 2614}, "micro avg": {"f1-score": 0.9812547819433818, "precision": 0.9812547819433818, "recall": 0.9812547819433818, "support": 2614}, "weighted avg": {"f1-score": 0.9805501686617433, "precision": 0.9821175901958236, "recall": 0.9812547819433818, "support": 2614}, "\u2205": {"f1-score": 0.992951271835734, "precision": 0.9860012172854534, "recall": 1.0, "support": 1620}, "\u23ce": {"f1-score": 0.9590163934426229, "precision": 0.9285714285714286, "recall": 0.9915254237288136, "support": 118}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9533678756476685, "precision": 0.9108910891089109, "recall": 1.0, "support": 92}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9533678756476685, "precision": 0.92, "recall": 0.989247311827957, "support": 93}, "\u2423": {"f1-score": 0.8953229398663696, "precision": 1.0, "recall": 0.8104838709677419, "support": 248}},
  "cl_report_full": {"\u0027": {"f1-score": 0.865234375, "precision": 1.0, "recall": 0.7624784853700516, "support": 581}, "macro avg": {"f1-score": 0.7147836431560997, "precision": 0.9575772891609655, "recall": 0.6321610651251934, "support": 4401}, "micro avg": {"f1-score": 0.7312900926585888, "precision": 0.9812547819433818, "recall": 0.5828220858895705, "support": 4401}, "weighted avg": {"f1-score": 0.6793940661976556, "precision": 0.9834994218301261, "recall": 0.5828220858895705, "support": 4401}, "\u2205": {"f1-score": 0.8898654215874761, "precision": 0.9860012172854534, "recall": 0.8108108108108109, "support": 1998}, "\u23ce": {"f1-score": 0.467065868263473, "precision": 0.9285714285714286, "recall": 0.312, "support": 375}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8803827751196173, "precision": 0.9108910891089109, "recall": 0.8518518518518519, "support": 108}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9064039408866995, "precision": 0.92, "recall": 0.8932038834951457, "support": 103}, "\u2423": {"f1-score": 0.2797494780793319, "precision": 1.0, "recall": 0.16262135922330098, "support": 1236}},
  "ppcr": 0.5939559191092934
}
```
</details>
