# Train report for javascript / file:///tmp/top-repos-quality-repos-oy2gbnvp/web-app-sample.git HEAD 4366654119c2beba3f06c28288fc0317fadd707d

### Classification report

PPCR: 0.730

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.989| 0.938| 0.983| 0.957| 6822| 7196| 0.948 |
| `␣` | 0.953| 0.934| 0.509| 0.943| 0.664| 1709| 3134| 0.545 |
| `'` | 1.000| 1.000| 0.915| 1.000| 0.956| 1074| 1174| 0.915 |
| `⏎` | 0.942| 0.801| 0.146| 0.866| 0.253| 161| 882| 0.183 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 344| 0.038 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 339| 0.021 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 332| 0.009 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 9| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `weighted avg` | 0.973| 0.975| 0.712| 0.974| 0.769| 9789| 13416| 0.730 |
| `micro avg` | 0.975| 0.975| 0.712| 0.975| 0.823| 9789| 13416| 0.730 |
| `macro avg` | 0.430| 0.414| 0.279| 0.421| 0.314| 9789| 13416| 0.730 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|374 |6749 |69 |0 |4 |0 |0 |0 |0 |0 |
|1425 |112 |1596 |0 |1 |0 |0 |0 |0 |0 |
|100 |0 |0 |1074 |0 |0 |0 |0 |0 |0 |
|721 |27 |5 |0 |129 |0 |0 |0 |0 |0 |
|329 |0 |0 |0 |3 |0 |0 |0 |0 |0 |
|9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|331 |8 |5 |0 |0 |0 |0 |0 |0 |0 |
|332 |7 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| nodejs/app4/myapp/public/javascripts/rtc.js | 47 |
| nodejs/app4/myapp/bin/www | 29 |
| python/web/app4/js/controller.js | 17 |
| python/app4/js/controller.js | 17 |
| python/web/app4/js/main.js | 12 |
| python/app4/js/main.js | 12 |
| nodejs/app1/myapp/bin/www | 11 |
| nodejs/app4/myapp/routes/timeline.js | 10 |
| python/app3/templates/script.js | 8 |
| python/app3/js/video.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1074}, "macro avg": {"f1-score": 0.4213883401461247, "precision": 0.43023694659537465, "recall": 0.4138245581565859, "support": 9789}, "micro avg": {"f1-score": 0.9753805291653898, "precision": 0.9753805291653898, "recall": 0.9753805291653898, "support": 9789}, "weighted avg": {"f1-score": 0.9740110898767441, "precision": 0.9729085706889001, "recall": 0.9753805291653898, "support": 9789}, "\u2205": {"f1-score": 0.9834608378870674, "precision": 0.9776908590467912, "recall": 0.9892993257109353, "support": 6822}, "\u23ce": {"f1-score": 0.8657718120805369, "precision": 0.9416058394160584, "recall": 0.8012422360248447, "support": 161}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9432624113475178, "precision": 0.9528358208955224, "recall": 0.9338794616734932, "support": 1709}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9555160142348754, "precision": 1.0, "recall": 0.9148211243611585, "support": 1174}, "macro avg": {"f1-score": 0.31442597082752166, "precision": 0.43023694659537465, "recall": 0.2786905705408067, "support": 13416}, "micro avg": {"f1-score": 0.8229260935143288, "precision": 0.9753805291653898, "recall": 0.7116875372689326, "support": 13416}, "weighted avg": {"f1-score": 0.768824805022908, "precision": 0.8964033418867203, "recall": 0.7116875372689326, "support": 13416}, "\u2205": {"f1-score": 0.9573728633236399, "precision": 0.9776908590467912, "recall": 0.9378821567537521, "support": 7196}, "\u23ce": {"f1-score": 0.253189401373896, "precision": 0.9416058394160584, "recall": 0.14625850340136054, "support": 882}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 332}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 344}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 339}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.6637554585152838, "precision": 0.9528358208955224, "recall": 0.5092533503509892, "support": 3134}},
  "ppcr": 0.7296511627906976
}
```
</details>
