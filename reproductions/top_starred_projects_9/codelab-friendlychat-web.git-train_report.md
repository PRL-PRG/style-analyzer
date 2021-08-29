# Train report for javascript / file:///tmp/top-repos-quality-repos-ajcn6pfw/codelab-friendlychat-web.git HEAD 1bb39a63bbbafccbe22b647e9660c88b305347d7

### Classification report

PPCR: 0.708

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.994| 0.956| 0.984| 0.966| 7200| 7485| 0.962 |
| `'` | 1.000| 1.000| 0.918| 1.000| 0.958| 1104| 1202| 0.918 |
| `␣` | 0.948| 0.915| 0.301| 0.931| 0.457| 856| 2604| 0.329 |
| `⏎` | 0.945| 0.918| 0.308| 0.931| 0.465| 340| 1012| 0.336 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 438| 0.135 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 451| 0.049 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 373| 0.051 |
| `weighted avg` | 0.964| 0.975| 0.690| 0.969| 0.740| 9600| 13565| 0.708 |
| `micro avg` | 0.975| 0.975| 0.690| 0.975| 0.808| 9600| 13565| 0.708 |
| `macro avg` | 0.553| 0.547| 0.355| 0.550| 0.406| 9600| 13565| 0.708 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|285 |7157 |43 |0 |0 |0 |0 |0 |
|1748 |73 |783 |0 |0 |0 |0 |0 |
|98 |0 |0 |1104 |0 |0 |0 |0 |
|672 |28 |0 |0 |312 |0 |0 |0 |
|429 |22 |0 |0 |0 |0 |0 |0 |
|379 |59 |0 |0 |0 |0 |0 |0 |
|354 |1 |0 |0 |18 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| cloud-functions-angular/functions/index.js | 41 |
| cloud-functions-start/public/scripts/main.js | 24 |
| performance-monitoring/public/scripts/main.js | 24 |
| cloud-functions/public/scripts/main.js | 24 |
| web/public/scripts/main.js | 24 |
| performance-monitoring-start/public/scripts/main.js | 24 |
| cloud-functions/functions/index.js | 20 |
| cloud-functions-angular-start/karma.conf.js | 11 |
| cloud-functions-angular/karma.conf.js | 11 |
| web-start/public/scripts/main.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1104}, "macro avg": {"f1-score": 0.5495477767989185, "precision": 0.5526377934236297, "recall": 0.5466277803956473, "support": 9600}, "micro avg": {"f1-score": 0.9745833333333332, "precision": 0.9745833333333334, "recall": 0.9745833333333334, "support": 9600}, "weighted avg": {"f1-score": 0.9693448194449658, "precision": 0.9643107568050415, "recall": 0.9745833333333334, "support": 9600}, "\u2205": {"f1-score": 0.9844566712517194, "precision": 0.9750681198910082, "recall": 0.9940277777777777, "support": 7200}, "\u23ce": {"f1-score": 0.9313432835820895, "precision": 0.9454545454545454, "recall": 0.9176470588235294, "support": 340}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u2423": {"f1-score": 0.9310344827586208, "precision": 0.9479418886198547, "recall": 0.9147196261682243, "support": 856}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9575021682567215, "precision": 1.0, "recall": 0.9184692179700499, "support": 1202}, "macro avg": {"f1-score": 0.4063672539468484, "precision": 0.5526377934236297, "recall": 0.35480569745467105, "support": 13565}, "micro avg": {"f1-score": 0.8077703431901577, "precision": 0.9745833333333334, "recall": 0.6897161813490601, "support": 13565}, "weighted avg": {"f1-score": 0.7399452729635978, "precision": 0.8791467420088682, "recall": 0.6897161813490601, "support": 13565}, "\u2205": {"f1-score": 0.9655311973018549, "precision": 0.9750681198910082, "recall": 0.9561790247160988, "support": 7485}, "\u23ce": {"f1-score": 0.4649776453055142, "precision": 0.9454545454545454, "recall": 0.308300395256917, "support": 1012}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 373}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 451}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 438}, "\u2423": {"f1-score": 0.45655976676384846, "precision": 0.9479418886198547, "recall": 0.30069124423963134, "support": 2604}},
  "ppcr": 0.7077036490969406
}
```
</details>
