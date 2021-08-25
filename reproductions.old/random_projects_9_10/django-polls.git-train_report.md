# Train report for javascript / file:///tmp/top-repos-quality-repos-kfvbti2n/django-polls.git HEAD c99f95871aee715298d1e7c7538734cd6f8b3c6d

### Classification report

PPCR: 0.765

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.992| 0.934| 0.987| 0.958| 7542| 8005| 0.942 |
| `␣` | 0.950| 0.980| 0.758| 0.965| 0.843| 3464| 4480| 0.773 |
| `'` | 1.000| 1.000| 0.495| 1.000| 0.663| 1473| 2973| 0.495 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.974| 0.906| 0.834| 0.939| 0.898| 448| 487| 0.920 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.981| 0.873| 0.676| 0.924| 0.801| 363| 469| 0.774 |
| `⏎` | 0.940| 0.652| 0.199| 0.770| 0.329| 339| 1108| 0.306 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 122| 0.041 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 168| 0.000 |
| `micro avg` | 0.975| 0.975| 0.746| 0.975| 0.845| 13634| 17812| 0.765 |
| `macro avg` | 0.729| 0.675| 0.487| 0.698| 0.561| 13634| 17812| 0.765 |
| `weighted avg` | 0.974| 0.975| 0.746| 0.974| 0.819| 13634| 17812| 0.765 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|463 |7479 |58 |0 |0 |5 |0 |0 |0 |
|1016 |58 |3396 |0 |0 |4 |6 |0 |0 |
|1500 |0 |0 |1473 |0 |0 |0 |0 |0 |
|769 |24 |92 |0 |221 |2 |0 |0 |0 |
|39 |12 |27 |0 |3 |406 |0 |0 |0 |
|106 |38 |0 |0 |8 |0 |317 |0 |0 |
|168 |0 |0 |0 |0 |0 |0 |0 |0 |
|117 |2 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/admin/js/urlify.js | 78 |
| static/admin/js/core.js | 48 |
| static/admin/js/inlines.js | 48 |
| static/admin/js/actions.js | 39 |
| static/admin/js/SelectFilter2.js | 36 |
| static/admin/js/admin/DateTimeShortcuts.js | 32 |
| static/admin/js/calendar.js | 14 |
| static/admin/js/SelectBox.js | 9 |
| static/admin/js/timeparse.js | 9 |
| static/admin/js/popup_response.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1473}, "macro avg": {"f1-score": 0.6981433043363208, "precision": 0.7285413886601094, "recall": 0.6754327417623627, "support": 13634}, "micro avg": {"f1-score": 0.9749156520463547, "precision": 0.9749156520463547, "recall": 0.9749156520463547, "support": 13634}, "weighted avg": {"f1-score": 0.9738473669689519, "precision": 0.9744677163019474, "recall": 0.9749156520463547, "support": 13634}, "\u2205": {"f1-score": 0.9870009897723524, "precision": 0.9823985288322606, "recall": 0.9916467780429594, "support": 7542}, "\u23ce": {"f1-score": 0.7700348432055749, "precision": 0.9404255319148936, "recall": 0.6519174041297935, "support": 339}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9387283236994219, "precision": 0.973621103117506, "recall": 0.90625, "support": 448}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.924198250728863, "precision": 0.9814241486068112, "recall": 0.8732782369146006, "support": 363}, "\u2423": {"f1-score": 0.9651840272843542, "precision": 0.9504617968094039, "recall": 0.9803695150115473, "support": 3464}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 168}, "\u0027": {"f1-score": 0.6626180836707152, "precision": 1.0, "recall": 0.4954591321897074, "support": 2973}, "macro avg": {"f1-score": 0.5614525759521868, "precision": 0.7285413886601094, "recall": 0.4871032682953722, "support": 17812}, "micro avg": {"f1-score": 0.8453857406347388, "precision": 0.9749156520463547, "recall": 0.7462384909050078, "support": 17812}, "weighted avg": {"f1-score": 0.8192623782782319, "precision": 0.9584326277388783, "recall": 0.7462384909050078, "support": 17812}, "\u2205": {"f1-score": 0.9577410679984634, "precision": 0.9823985288322606, "recall": 0.9342910680824484, "support": 8005}, "\u23ce": {"f1-score": 0.3291139240506329, "precision": 0.9404255319148936, "recall": 0.19945848375451264, "support": 1108}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8982300884955753, "precision": 0.973621103117506, "recall": 0.8336755646817249, "support": 487}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8005050505050505, "precision": 0.9814241486068112, "recall": 0.67590618336887, "support": 469}, "\u2423": {"f1-score": 0.843412392897057, "precision": 0.9504617968094039, "recall": 0.7580357142857143, "support": 4480}},
  "ppcr": 0.765439029867505
}
```
</details>
