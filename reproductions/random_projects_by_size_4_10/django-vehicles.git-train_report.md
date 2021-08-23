# Train report for javascript / file:///tmp/top-repos-quality-repos-tcm12m1c/django-vehicles.git HEAD 1b189885569c3be96d09b3bb55744ae070138178

### Classification report

PPCR: 0.612

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 0.998| 0.912| 0.983| 0.940| 8587| 9397| 0.914 |
| `␣` | 0.963| 0.934| 0.503| 0.948| 0.661| 2826| 5251| 0.538 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.969| 0.886| 0.802| 0.926| 0.878| 421| 465| 0.905 |
| `⏎` | 0.957| 0.702| 0.224| 0.810| 0.362| 382| 1199| 0.319 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.962| 0.865| 0.679| 0.911| 0.796| 355| 452| 0.785 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 178| 0.039 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3182| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 414| 0.000 |
| `macro avg` | 0.602| 0.548| 0.390| 0.572| 0.455| 12578| 20538| 0.612 |
| `weighted avg` | 0.966| 0.967| 0.592| 0.966| 0.657| 12578| 20538| 0.612 |
| `micro avg` | 0.967| 0.967| 0.592| 0.967| 0.734| 12578| 20538| 0.612 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|810 |8573 |14 |0 |0 |0 |0 |0 |0 |
|2425 |164 |2640 |0 |0 |10 |12 |0 |0 |
|3182 |0 |0 |0 |0 |0 |0 |0 |0 |
|817 |27 |85 |0 |268 |2 |0 |0 |0 |
|44 |42 |3 |0 |3 |373 |0 |0 |0 |
|97 |42 |0 |0 |6 |0 |307 |0 |0 |
|414 |0 |0 |0 |0 |0 |0 |0 |0 |
|171 |4 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| staticfiles/admin/js/urlify.js | 74 |
| staticfiles/admin/js/core.js | 57 |
| staticfiles/admin/js/SelectFilter2.js | 55 |
| staticfiles/rest_framework/docs/js/api.js | 51 |
| staticfiles/admin/js/inlines.js | 38 |
| staticfiles/admin/js/actions.js | 36 |
| staticfiles/admin/js/admin/DateTimeShortcuts.js | 24 |
| staticfiles/admin/js/SelectBox.js | 16 |
| staticfiles/admin/js/admin/RelatedObjectLookups.js | 16 |
| staticfiles/admin/js/calendar.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5722099568533525, "precision": 0.6024548806799596, "recall": 0.5481121724978651, "support": 12578}, "micro avg": {"f1-score": 0.9668468754968993, "precision": 0.9668468754968993, "recall": 0.9668468754968993, "support": 12578}, "weighted avg": {"f1-score": 0.9655672206817618, "precision": 0.9661615427576734, "recall": 0.9668468754968993, "support": 12578}, "\u2205": {"f1-score": 0.983198577900109, "precision": 0.968481699051062, "recall": 0.9983696285082101, "support": 8587}, "\u23ce": {"f1-score": 0.809667673716012, "precision": 0.9571428571428572, "recall": 0.7015706806282722, "support": 382}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9255583126550868, "precision": 0.9688311688311688, "recall": 0.8859857482185273, "support": 421}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9109792284866468, "precision": 0.9623824451410659, "recall": 0.8647887323943662, "support": 355}, "\u2423": {"f1-score": 0.9482758620689655, "precision": 0.962800875273523, "recall": 0.9341825902335457, "support": 2826}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 414}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3182}, "macro avg": {"f1-score": 0.45456984763859687, "precision": 0.6024548806799596, "recall": 0.38999343700615874, "support": 20538}, "micro avg": {"f1-score": 0.7344486049039739, "precision": 0.9668468754968993, "recall": 0.5921219203427792, "support": 20538}, "weighted avg": {"f1-score": 0.6573334873208295, "precision": 0.7882757603694925, "recall": 0.5921219203427792, "support": 20538}, "\u2205": {"f1-score": 0.9395583319633953, "precision": 0.968481699051062, "recall": 0.9123124401404704, "support": 9397}, "\u23ce": {"f1-score": 0.36240703177822853, "precision": 0.9571428571428572, "recall": 0.22351959966638865, "support": 1199}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 178}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8776470588235293, "precision": 0.9688311688311688, "recall": 0.8021505376344086, "support": 465}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7963683527885862, "precision": 0.9623824451410659, "recall": 0.6792035398230089, "support": 452}, "\u2423": {"f1-score": 0.6605780057550358, "precision": 0.962800875273523, "recall": 0.5027613787849934, "support": 5251}},
  "ppcr": 0.6124257473950725
}
```
</details>
