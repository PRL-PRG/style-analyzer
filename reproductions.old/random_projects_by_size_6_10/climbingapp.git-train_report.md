# Train report for javascript / file:///tmp/top-repos-quality-repos-94t7vbdo/climbingapp.git HEAD 100d7c4bf2ac434ff63fba567ef74f36f7c89654

### Classification report

PPCR: 0.359

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 0.988| 0.437| 0.978| 0.602| 2257| 5106| 0.442 |
| `␣` | 0.951| 0.995| 0.301| 0.973| 0.458| 611| 2017| 0.303 |
| `'` | 1.000| 1.000| 0.444| 1.000| 0.615| 273| 615| 0.444 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 312| 0.151 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 343| 0.044 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 314| 0.025 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 239| 0.021 |
| `weighted avg` | 0.945| 0.967| 0.348| 0.956| 0.489| 3216| 8946| 0.359 |
| `micro avg` | 0.967| 0.967| 0.348| 0.967| 0.512| 3216| 8946| 0.359 |
| `macro avg` | 0.417| 0.426| 0.169| 0.422| 0.239| 3216| 8946| 0.359 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2849 |2230 |27 |0 |0 |0 |0 |0 |
|1406 |3 |608 |0 |0 |0 |0 |0 |
|342 |0 |0 |273 |0 |0 |0 |0 |
|328 |15 |0 |0 |0 |0 |0 |0 |
|265 |47 |0 |0 |0 |0 |0 |0 |
|306 |4 |4 |0 |0 |0 |0 |0 |
|234 |5 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| climbingApp/static/ClimbingApp/javascript/main.js | 25 |
| climbingApp/static/ClimbingApp/javascript/components/fullGym/fullGym.js | 15 |
| climbingApp/static/ClimbingApp/javascript/components/listGyms/listGyms.js | 9 |
| climbingApp/static/ClimbingApp/javascript/views/listRoutes.js | 8 |
| climbingApp/static/ClimbingApp/javascript/services/authService.js | 8 |
| climbingApp/static/ClimbingApp/javascript/components/listAscents/listAscents.js | 7 |
| climbingApp/static/ClimbingApp/javascript/views/editAscent.js | 6 |
| climbingApp/static/ClimbingApp/javascript/services/wallService.js | 5 |
| climbingApp/static/ClimbingApp/javascript/services/baseService.js | 4 |
| climbingApp/static/ClimbingApp/javascript/views/editRoute.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 273}, "macro avg": {"f1-score": 0.4215222476274, "precision": 0.41705266320143075, "recall": 0.42616103341600375, "support": 3216}, "micro avg": {"f1-score": 0.9673507462686566, "precision": 0.9673507462686567, "recall": 0.9673507462686567, "support": 3216}, "weighted avg": {"f1-score": 0.9559705193610986, "precision": 0.9449216172475357, "recall": 0.9673507462686567, "support": 3216}, "\u2205": {"f1-score": 0.9778557333918001, "precision": 0.9678819444444444, "recall": 0.9880372175454143, "support": 2257}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u2423": {"f1-score": 0.9728, "precision": 0.9514866979655712, "recall": 0.9950900163666121, "support": 611}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6148648648648649, "precision": 1.0, "recall": 0.44390243902439025, "support": 615}, "macro avg": {"f1-score": 0.23922650412821625, "precision": 0.41705266320143075, "recall": 0.16886875811698804, "support": 8946}, "micro avg": {"f1-score": 0.5115934879131722, "precision": 0.9673507462686567, "recall": 0.3477531857813548, "support": 8946}, "weighted avg": {"f1-score": 0.4890268990148866, "precision": 0.8356979519483445, "recall": 0.3477531857813548, "support": 8946}, "\u2205": {"f1-score": 0.6018893387314441, "precision": 0.9678819444444444, "recall": 0.43674108891500196, "support": 5106}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 343}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 239}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 314}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 312}, "\u2423": {"f1-score": 0.45783132530120485, "precision": 0.9514866979655712, "recall": 0.30143777887952405, "support": 2017}},
  "ppcr": 0.3594902749832327
}
```
</details>
