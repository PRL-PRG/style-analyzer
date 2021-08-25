# Train report for javascript / file:///tmp/top-repos-quality-repos-1xc8eazm/xchange.git HEAD 845264b7f9de5d3be122c06c4bf2e6e5aae69222

### Classification report

PPCR: 0.591

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.919| 0.995| 0.952| 0.956| 0.935| 7814| 8172| 0.956 |
| `␣` | 0.965| 0.780| 0.280| 0.863| 0.434| 1623| 4519| 0.359 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.963| 0.685| 0.678| 0.801| 0.795| 454| 459| 0.989 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.969| 0.860| 0.735| 0.911| 0.836| 406| 475| 0.855 |
| `⏎` | 0.940| 0.634| 0.196| 0.757| 0.324| 347| 1122| 0.309 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 130| 0.338 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2792| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 408| 0.000 |
| `macro avg` | 0.595| 0.494| 0.355| 0.536| 0.416| 10688| 18077| 0.591 |
| `micro avg` | 0.928| 0.928| 0.549| 0.928| 0.690| 10688| 18077| 0.591 |
| `weighted avg` | 0.927| 0.928| 0.549| 0.923| 0.594| 10688| 18077| 0.591 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|358 |7777 |35 |0 |2 |0 |0 |0 |0 |
|2896 |336 |1266 |0 |0 |0 |9 |12 |0 |
|2792 |0 |0 |0 |0 |0 |0 |0 |0 |
|775 |125 |0 |0 |220 |0 |2 |0 |0 |
|408 |0 |0 |0 |0 |0 |0 |0 |0 |
|69 |43 |11 |0 |3 |0 |349 |0 |0 |
|5 |137 |0 |0 |6 |0 |0 |311 |0 |
|86 |41 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| XChange/static/admin/js/core.js | 120 |
| XChange/static/admin/js/inlines.js | 98 |
| XChange/static/admin/js/SelectFilter2.js | 86 |
| XChange/static/admin/js/actions.js | 66 |
| XChange/static/admin/js/SelectBox.js | 64 |
| XChange/static/admin/js/calendar.js | 61 |
| XChange/static/admin/js/admin/DateTimeShortcuts.js | 61 |
| XChange/static/admin/js/admin/RelatedObjectLookups.js | 53 |
| XChange/static/admin/js/timeparse.js | 48 |
| XChange/static/admin/js/urlify.js | 42 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5359456205225097, "precision": 0.5945973148703843, "recall": 0.49424194739558547, "support": 10688}, "micro avg": {"f1-score": 0.9284244011976048, "precision": 0.9284244011976048, "recall": 0.9284244011976048, "support": 10688}, "weighted avg": {"f1-score": 0.9230050531877618, "precision": 0.9269335259154515, "recall": 0.9284244011976048, "support": 10688}, "\u2205": {"f1-score": 0.9558163829656485, "precision": 0.9193758127438232, "recall": 0.9952649091374456, "support": 7814}, "\u23ce": {"f1-score": 0.7573149741824441, "precision": 0.9401709401709402, "recall": 0.6340057636887608, "support": 347}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9112271540469974, "precision": 0.9694444444444444, "recall": 0.8596059113300493, "support": 406}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8005148005148004, "precision": 0.9628482972136223, "recall": 0.6850220264317181, "support": 454}, "\u2423": {"f1-score": 0.8626916524701874, "precision": 0.9649390243902439, "recall": 0.7800369685767098, "support": 1623}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 408}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2792}, "macro avg": {"f1-score": 0.4156600738479919, "precision": 0.5945973148703843, "recall": 0.3550237351732729, "support": 18077}, "micro avg": {"f1-score": 0.6899356857291847, "precision": 0.9284244011976048, "recall": 0.5489295790230679, "support": 18077}, "weighted avg": {"f1-score": 0.5936441678479768, "precision": 0.7651161070623441, "recall": 0.5489295790230679, "support": 18077}, "\u2205": {"f1-score": 0.9352414166315915, "precision": 0.9193758127438232, "recall": 0.9516642192853647, "support": 8172}, "\u23ce": {"f1-score": 0.32448377581120946, "precision": 0.9401709401709402, "recall": 0.19607843137254902, "support": 1122}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8359281437125747, "precision": 0.9694444444444444, "recall": 0.7347368421052631, "support": 475}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7953964194373401, "precision": 0.9628482972136223, "recall": 0.6775599128540305, "support": 459}, "\u2423": {"f1-score": 0.4342308351912193, "precision": 0.9649390243902439, "recall": 0.28015047576897545, "support": 4519}},
  "ppcr": 0.5912485478785197
}
```
</details>
