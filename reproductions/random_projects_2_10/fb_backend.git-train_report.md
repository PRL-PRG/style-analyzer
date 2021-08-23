# Train report for javascript / file:///tmp/top-repos-quality-repos-eo3711xb/fb_backend.git HEAD 0851c3ba0eec5ed929bea9ba90f8d28a5d4f7707

### Classification report

PPCR: 0.637

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 0.991| 0.943| 0.991| 0.966| 8947| 9409| 0.951 |
| `␣` | 0.954| 0.995| 0.698| 0.974| 0.807| 3737| 5326| 0.702 |
| `⏎` | 0.945| 0.885| 0.217| 0.914| 0.353| 313| 1275| 0.245 |
| `'` | 1.000| 1.000| 0.054| 1.000| 0.103| 177| 3257| 0.054 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 477| 0.103 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 464| 0.088 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 105| 0.305 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 112| 0.107 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 188| 0.021 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 285| 0.000 |
| `macro avg` | 0.389| 0.387| 0.191| 0.388| 0.223| 13312| 20898| 0.637 |
| `weighted avg` | 0.970| 0.980| 0.624| 0.975| 0.678| 13312| 20898| 0.637 |
| `micro avg` | 0.980| 0.980| 0.624| 0.980| 0.762| 13312| 20898| 0.637 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|462 |8868 |79 |0 |0 |0 |0 |0 |0 |0 |0 |
|1589 |17 |3720 |0 |0 |0 |0 |0 |0 |0 |0 |
|3080 |0 |0 |177 |0 |0 |0 |0 |0 |0 |0 |
|962 |11 |25 |0 |277 |0 |0 |0 |0 |0 |0 |
|428 |7 |39 |0 |3 |0 |0 |0 |0 |0 |0 |
|423 |19 |14 |0 |8 |0 |0 |0 |0 |0 |0 |
|285 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|184 |0 |1 |0 |3 |0 |0 |0 |0 |0 |0 |
|100 |3 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|73 |19 |11 |0 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/rest_framework/docs/js/api.js | 42 |
| static/admin/js/SelectFilter2.js | 34 |
| static/admin/js/core.js | 32 |
| static/admin/js/inlines.js | 29 |
| static/admin/js/urlify.js | 28 |
| static/admin/js/actions.js | 25 |
| static/admin/js/admin/DateTimeShortcuts.js | 15 |
| static/rest_framework/js/ajax-form.js | 13 |
| static/admin/js/calendar.js | 11 |
| static/admin/js/admin/RelatedObjectLookups.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 177}, "macro avg": {"f1-score": 0.38799875702288444, "precision": 0.38912307315264666, "recall": 0.38716051466564105, "support": 13312}, "micro avg": {"f1-score": 0.9797175480769231, "precision": 0.9797175480769231, "recall": 0.9797175480769231, "support": 13312}, "weighted avg": {"f1-score": 0.9746232655477822, "precision": 0.9698192858512189, "recall": 0.9797175480769231, "support": 13312}, "\u2205": {"f1-score": 0.991336426136046, "precision": 0.9915026833631485, "recall": 0.9911702246563093, "support": 8947}, "\u23ce": {"f1-score": 0.9141914191419142, "precision": 0.9453924914675768, "recall": 0.8849840255591054, "support": 313}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u2423": {"f1-score": 0.9744597249508841, "precision": 0.9543355566957414, "recall": 0.9954508964409955, "support": 3737}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 285}, "\u0027": {"f1-score": 0.10308677926616192, "precision": 1.0, "recall": 0.05434448879336813, "support": 3257}, "macro avg": {"f1-score": 0.22293761242383808, "precision": 0.38912307315264666, "recall": 0.19125616337021661, "support": 20898}, "micro avg": {"f1-score": 0.7624671148786905, "precision": 0.9797175480769231, "recall": 0.624078859220978, "support": 20898}, "weighted avg": {"f1-score": 0.6782860556341705, "precision": 0.9031589314454276, "recall": 0.624078859220978, "support": 20898}, "\u2205": {"f1-score": 0.9663815180079552, "precision": 0.9915026833631485, "recall": 0.9425018599213519, "support": 9409}, "\u23ce": {"f1-score": 0.3533163265306123, "precision": 0.9453924914675768, "recall": 0.2172549019607843, "support": 1275}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 188}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 477}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 464}, "\u2423": {"f1-score": 0.8065915004336514, "precision": 0.9543355566957414, "recall": 0.6984603830266617, "support": 5326}},
  "ppcr": 0.636998755861805
}
```
</details>
