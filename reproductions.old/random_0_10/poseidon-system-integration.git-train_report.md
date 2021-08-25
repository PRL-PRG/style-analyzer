# Train report for javascript / file:///tmp/top-repos-quality-repos-6sg6tfui/poseidon-system-integration.git HEAD a500d187f7c57e618c8a4d2551b0c941485c4a60

### Classification report

PPCR: 0.563

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.990| 0.944| 0.992| 0.968| 10560| 11083| 0.953 |
| `␣` | 0.959| 0.988| 0.381| 0.973| 0.545| 2912| 7553| 0.386 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 606| 0.038 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 1241| 0.017 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 675| 0.006 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1210| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1155| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 165| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 182| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 160| 0.000 |
| `macro avg` | 0.195| 0.198| 0.132| 0.197| 0.151| 13520| 24030| 0.563 |
| `weighted avg` | 0.983| 0.986| 0.555| 0.985| 0.618| 13520| 24030| 0.563 |
| `micro avg` | 0.986| 0.986| 0.555| 0.986| 0.710| 13520| 24030| 0.563 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|523 |10459 |101 |0 |0 |0 |0 |0 |0 |0 |0 |
|4641 |35 |2877 |0 |0 |0 |0 |0 |0 |0 |0 |
|1210 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1220 |4 |17 |0 |0 |0 |0 |0 |0 |0 |0 |
|1155 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|671 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|583 |23 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|165 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|182 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|160 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/psi/web-server/plugins/input-mask/jquery.inputmask.js | 99 |
| src/psi/web-server/plugins/input-mask/jquery.inputmask.date.extensions.js | 30 |
| src/psi/web-server/Gruntfile.js | 20 |
| src/psi/web-server/plugins/iCheck/icheck.js | 14 |
| src/psi/web-server/plugins/input-mask/jquery.inputmask.numeric.extensions.js | 12 |
| src/psi/web-server/plugins/input-mask/jquery.inputmask.extensions.js | 7 |
| src/psi/web-server/plugins/input-mask/jquery.inputmask.regex.extensions.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19657072693429045, "precision": 0.19534267973048963, "recall": 0.19784163752913755, "support": 13520}, "micro avg": {"f1-score": 0.9863905325443787, "precision": 0.9863905325443787, "recall": 0.9863905325443787, "support": 13520}, "weighted avg": {"f1-score": 0.9846896902869284, "precision": 0.9830850113657112, "recall": 0.9863905325443787, "support": 13520}, "\u2205": {"f1-score": 0.9922679189791757, "precision": 0.9941070240471438, "recall": 0.9904356060606061, "support": 10560}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.9734393503637288, "precision": 0.9593197732577526, "recall": 0.9879807692307693, "support": 2912}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1155}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1210}, "macro avg": {"f1-score": 0.15135460902910322, "precision": 0.19534267973048963, "recall": 0.13246058031918065, "support": 24030}, "micro avg": {"f1-score": 0.710306258322237, "precision": 0.9863905325443787, "recall": 0.5549729504785684, "support": 24030}, "weighted avg": {"f1-score": 0.6179660504404678, "precision": 0.76002623366335, "recall": 0.5549729504785684, "support": 24030}, "\u2205": {"f1-score": 0.9682466209961118, "precision": 0.9941070240471438, "recall": 0.9436975548136786, "support": 11083}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1241}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 675}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 606}, "\u2423": {"f1-score": 0.5452994692949205, "precision": 0.9593197732577526, "recall": 0.3809082483781279, "support": 7553}},
  "ppcr": 0.5626300457761132
}
```
</details>
