# Train report for javascript / file:///tmp/top-repos-quality-repos-t3sqsuxa/citgm.git HEAD 5fd10b4af59f91c2640dc9982811385fc991e2cc

### Classification report

PPCR: 0.971

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.960| 0.979| 0.978| 0.970| 0.969| 14384| 14394| 0.999 |
| `␣` | 0.908| 0.967| 0.966| 0.937| 0.936| 6796| 6805| 0.999 |
| `'` | 1.000| 1.000| 0.988| 1.000| 0.994| 3730| 3775| 0.988 |
| `⏎` | 0.880| 0.809| 0.660| 0.843| 0.754| 1693| 2076| 0.816 |
| `⏎␣⁻␣⁻` | 0.935| 0.716| 0.716| 0.811| 0.811| 1030| 1030| 1.000 |
| `⏎␣⁺␣⁺` | 0.963| 0.800| 0.800| 0.874| 0.874| 1005| 1005| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 176| 594| 0.296 |
| `weighted avg` | 0.942| 0.947| 0.920| 0.943| 0.922| 28814| 29679| 0.971 |
| `macro avg` | 0.807| 0.753| 0.730| 0.776| 0.763| 28814| 29679| 0.971 |
| `micro avg` | 0.947| 0.947| 0.920| 0.947| 0.933| 28814| 29679| 0.971 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|10 |14084 |274 |0 |0 |0 |26 |0 |
|9 |115 |6572 |0 |53 |31 |25 |0 |
|45 |0 |0 |3730 |0 |0 |0 |0 |
|383 |54 |269 |0 |1370 |0 |0 |0 |
|0 |137 |64 |0 |0 |804 |0 |0 |
|0 |278 |1 |0 |14 |0 |737 |0 |
|418 |1 |55 |0 |120 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/test-lookup.js | 156 |
| lib/lookup.js | 80 |
| bin/citgm-all.js | 68 |
| test/test-check-tags.js | 59 |
| lib/package-manager/test.js | 57 |
| test/reporter/test-reporter-util.js | 53 |
| test/bin/test-citgm-all.js | 50 |
| test/npm/test-npm-test.js | 45 |
| test/yarn/test-yarn-test.js | 44 |
| test/test-match-conditions.js | 43 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3730}, "macro avg": {"f1-score": 0.776298901775836, "precision": 0.8066475410067063, "recall": 0.7529901886571276, "support": 28814}, "micro avg": {"f1-score": 0.9473519816755743, "precision": 0.9473519816755743, "recall": 0.9473519816755743, "support": 28814}, "weighted avg": {"f1-score": 0.9433932564285724, "precision": 0.9417049712508628, "recall": 0.9473519816755743, "support": 28814}, "\u2205": {"f1-score": 0.9695384297662891, "precision": 0.9601199809121276, "recall": 0.9791434927697441, "support": 14384}, "\u23ce": {"f1-score": 0.8430769230769231, "precision": 0.8798972382787412, "recall": 0.8092144122858831, "support": 1693}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 176}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8739130434782609, "precision": 0.962874251497006, "recall": 0.8, "support": 1005}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8107810781078109, "precision": 0.9352791878172588, "recall": 0.7155339805825243, "support": 1030}, "\u2423": {"f1-score": 0.9367828380015679, "precision": 0.9083621285418106, "recall": 0.9670394349617422, "support": 6796}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9940039973351099, "precision": 1.0, "recall": 0.9880794701986755, "support": 3775}, "macro avg": {"f1-score": 0.7626118455426424, "precision": 0.8066475410067063, "recall": 0.7296800140440743, "support": 29679}, "micro avg": {"f1-score": 0.9333424512334809, "precision": 0.9473519816755743, "recall": 0.9197412311735571, "support": 29679}, "weighted avg": {"f1-score": 0.9216254465539669, "precision": 0.9277288366470945, "recall": 0.9197412311735571, "support": 29679}, "\u2205": {"f1-score": 0.9692048308846299, "precision": 0.9601199809121276, "recall": 0.9784632485757955, "support": 14394}, "\u23ce": {"f1-score": 0.7541976328103496, "precision": 0.8798972382787412, "recall": 0.6599229287090559, "support": 2076}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 594}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8739130434782609, "precision": 0.962874251497006, "recall": 0.8, "support": 1005}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8107810781078109, "precision": 0.9352791878172588, "recall": 0.7155339805825243, "support": 1030}, "\u2423": {"f1-score": 0.9361823361823363, "precision": 0.9083621285418106, "recall": 0.9657604702424688, "support": 6805}},
  "ppcr": 0.9708548131675595
}
```
</details>
