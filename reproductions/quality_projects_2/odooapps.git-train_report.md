# Train report for javascript / file:///tmp/top-repos-quality-repos-ghkylp30/odooapps.git HEAD 9ef6c85d8b47e8ca6a1d4527e41a7f134445cd2a

### Classification report

PPCR: 0.808

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.989| 0.952| 0.985| 0.966| 9063| 9421| 0.962 |
| `␣` | 0.967| 0.976| 0.883| 0.972| 0.923| 4805| 5308| 0.905 |
| `'` | 0.991| 1.000| 0.825| 0.995| 0.900| 1097| 1330| 0.825 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 362| 0.110 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 1057| 0.024 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 130| 0.108 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 120| 0.108 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 400| 0.030 |
| `"` | 1.000| 0.167| 0.043| 0.286| 0.082| 12| 47| 0.255 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 64| 0.109 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 63| 0.095 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 347| 0.000 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 19| 0.000 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 17| 0.000 |
| `micro avg` | 0.977| 0.977| 0.789| 0.977| 0.873| 15094| 18688| 0.808 |
| `macro avg` | 0.263| 0.209| 0.180| 0.216| 0.191| 15094| 18688| 0.808 |
| `weighted avg` | 0.970| 0.977| 0.789| 0.973| 0.814| 15094| 18688| 0.808 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| "| ⏎⏎␣⁻␣⁻| ⏎⇥⁺| ⏎⏎⏎| ⏎⇥⁻| ⏎⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|358 |8965 |98 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|503 |116 |4689 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|233 |0 |0 |1097 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1032 |4 |21 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|388 |2 |10 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|322 |28 |12 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|347 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|107 |10 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|116 |6 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|35 |0 |0 |10 |0 |0 |0 |0 |0 |0 |2 |0 |0 |0 |0 |0 |
|19 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|57 |1 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|57 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|17 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/apps/www/lib/underscore/underscore.js | 67 |
| static/apps/www/app/detail/detail.controller.js | 57 |
| static/apps/www/lib/odoo/src/components/odoo/jsonRpc-service.js | 43 |
| static/apps/www/lib/buche/src/components/register/register.directive.js | 39 |
| static/apps/www/lib/buche/src/components/login/login.directive.js | 26 |
| static/apps/www/app/list/list.controller.js | 19 |
| static/apps/www/app/print/print.controller.js | 15 |
| static/apps/www/lib/buche/gulp/build.js | 12 |
| static/apps/www/lib/buche/src/app/main/main.controller.js | 7 |
| static/apps/www/app/index.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.2857142857142857, "precision": 1.0, "recall": 0.16666666666666666, "support": 12}, "\u0027": {"f1-score": 0.9954627949183303, "precision": 0.99096657633243, "recall": 1.0, "support": 1097}, "macro avg": {"f1-score": 0.21585999266362677, "precision": 0.2626291440509386, "recall": 0.20878079672683933, "support": 15094}, "micro avg": {"f1-score": 0.9774082416854379, "precision": 0.9774082416854379, "recall": 0.9774082416854379, "support": 15094}, "weighted avg": {"f1-score": 0.9733739069678597, "precision": 0.9698475767500987, "recall": 0.9774082416854379, "support": 15094}, "\u2205": {"f1-score": 0.985110708202846, "precision": 0.981068067410812, "recall": 0.9891868034867042, "support": 9063}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.9716121011189391, "precision": 0.9674025170208377, "recall": 0.9758584807492195, "support": 4805}},
  "cl_report_full": {"\"": {"f1-score": 0.08163265306122448, "precision": 1.0, "recall": 0.0425531914893617, "support": 47}, "\u0027": {"f1-score": 0.9002872384078786, "precision": 0.99096657633243, "recall": 0.824812030075188, "support": 1330}, "macro avg": {"f1-score": 0.19143426311286932, "precision": 0.2626291440509386, "recall": 0.18015641923263095, "support": 18688}, "micro avg": {"f1-score": 0.8734237167722456, "precision": 0.9774082416854379, "recall": 0.7894370719178082, "support": 18688}, "weighted avg": {"f1-score": 0.8136122951465311, "precision": 0.8423908588370076, "recall": 0.7894370719178082, "support": 18688}, "\u2205": {"f1-score": 0.9661080877202436, "precision": 0.981068067410812, "recall": 0.9515974949580724, "support": 9421}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1057}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 347}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 400}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 362}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 120}, "\u2423": {"f1-score": 0.9234859675036928, "precision": 0.9674025170208377, "recall": 0.8833835719668425, "support": 5308}},
  "ppcr": 0.8076840753424658
}
```
</details>
