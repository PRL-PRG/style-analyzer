# Train report for javascript / file:///tmp/top-repos-quality-repos-5k4caepc/archer.git HEAD a85408ae014b2a84f76e8ec0bcc7c5ee0a9af2c1

### Classification report

PPCR: 0.612

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.995| 0.995| 0.946| 0.995| 0.970| 22370| 23522| 0.951 |
| `␣` | 0.958| 0.989| 0.417| 0.974| 0.581| 4829| 11458| 0.421 |
| `⏎` | 0.932| 0.858| 0.154| 0.894| 0.265| 431| 2397| 0.180 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 970| 0.077 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 1200| 0.022 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 476| 0.046 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 140| 0.014 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3723| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1492| 0.000 |
| `weighted avg` | 0.983| 0.987| 0.604| 0.985| 0.664| 27756| 45378| 0.612 |
| `micro avg` | 0.987| 0.987| 0.604| 0.987| 0.750| 27756| 45378| 0.612 |
| `macro avg` | 0.321| 0.316| 0.169| 0.318| 0.202| 27756| 45378| 0.612 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1152 |22260 |110 |0 |0 |0 |0 |0 |0 |0 |
|6629 |52 |4777 |0 |0 |0 |0 |0 |0 |0 |
|3723 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1966 |25 |36 |0 |370 |0 |0 |0 |0 |0 |
|1492 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1173 |4 |17 |0 |6 |0 |0 |0 |0 |0 |
|895 |27 |44 |0 |4 |0 |0 |0 |0 |0 |
|454 |4 |1 |0 |17 |0 |0 |0 |0 |0 |
|138 |2 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| sql/static/ace/ext-language_tools.js | 104 |
| sql/static/daterangepicker/js/daterangepicker.js | 88 |
| sql/static/admin/js/inlines.js | 28 |
| sql/static/admin/js/core.js | 20 |
| sql/static/admin/js/admin/DateTimeShortcuts.js | 16 |
| sql/static/ace/ace_init.js | 13 |
| sql/static/user/js/charts.js | 12 |
| sql/static/admin/js/timeparse.js | 10 |
| sql/static/admin/js/actions.js | 9 |
| sql/static/admin/js/urlify.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.31802453150043075, "precision": 0.32057439434680046, "recall": 0.31586478917041394, "support": 27756}, "micro avg": {"f1-score": 0.9874261420954028, "precision": 0.9874261420954028, "recall": 0.9874261420954028, "support": 27756}, "weighted avg": {"f1-score": 0.9851660788634787, "precision": 0.9830385201618396, "recall": 0.9874261420954028, "support": 27756}, "\u2205": {"f1-score": 0.9949937421777222, "precision": 0.9949048002145348, "recall": 0.9950827000447027, "support": 22370}, "\u23ce": {"f1-score": 0.893719806763285, "precision": 0.9319899244332494, "recall": 0.8584686774941995, "support": 431}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u2423": {"f1-score": 0.9735072345628694, "precision": 0.9582748244734203, "recall": 0.9892317249948229, "support": 4829}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1492}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3723}, "macro avg": {"f1-score": 0.20176777270330093, "precision": 0.32057439434680046, "recall": 0.1686246291584253, "support": 45378}, "micro avg": {"f1-score": 0.7495009161265622, "precision": 0.9874261420954028, "recall": 0.6039710873110318, "support": 45378}, "weighted avg": {"f1-score": 0.6635191548273078, "precision": 0.8069117964284285, "recall": 0.6039710873110318, "support": 45378}, "\u2205": {"f1-score": 0.9700191737842079, "precision": 0.9949048002145348, "recall": 0.9463480996513902, "support": 23522}, "\u23ce": {"f1-score": 0.2648532569792412, "precision": 0.9319899244332494, "recall": 0.1543596161869003, "support": 2397}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 476}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1200}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 970}, "\u2423": {"f1-score": 0.5810375235662592, "precision": 0.9582748244734203, "recall": 0.4169139465875371, "support": 11458}},
  "ppcr": 0.6116620388734629
}
```
</details>
