# Train report for javascript / file:///tmp/top-repos-quality-repos-aes85ail/campus-navigator.git HEAD 42004fbd9b618b68d7430d91d10190ecac7dc7d4

### Classification report

PPCR: 0.592

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.952| 0.998| 0.857| 0.974| 0.902| 6266| 7298| 0.859 |
| `␣` | 0.974| 0.869| 0.322| 0.919| 0.485| 1054| 2841| 0.371 |
| `⏎` | 0.978| 0.942| 0.396| 0.959| 0.564| 565| 1343| 0.421 |
| `'` | 1.000| 1.000| 0.096| 1.000| 0.175| 91| 951| 0.096 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 68| 391| 0.174 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 354| 0.133 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 94| 0.351 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 226| 0.044 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 88| 0.114 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 175| 0.000 |
| `micro avg` | 0.957| 0.957| 0.566| 0.957| 0.711| 8144| 13761| 0.592 |
| `macro avg` | 0.390| 0.381| 0.167| 0.385| 0.212| 8144| 13761| 0.592 |
| `weighted avg` | 0.937| 0.957| 0.566| 0.946| 0.645| 8144| 13761| 0.592 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1032 |6251 |15 |0 |0 |0 |0 |0 |0 |0 |0 |
|1787 |138 |916 |0 |0 |0 |0 |0 |0 |0 |0 |
|778 |32 |1 |532 |0 |0 |0 |0 |0 |0 |0 |
|860 |0 |0 |0 |91 |0 |0 |0 |0 |0 |0 |
|323 |67 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|216 |2 |0 |8 |0 |0 |0 |0 |0 |0 |0 |
|307 |43 |0 |4 |0 |0 |0 |0 |0 |0 |0 |
|78 |3 |7 |0 |0 |0 |0 |0 |0 |0 |0 |
|61 |33 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|175 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/registerServiceWorker.js | 38 |
| config/webpack.config.prod.js | 35 |
| config/webpack.config.dev.js | 28 |
| scripts/build.js | 27 |
| src/serverattempt.js | 24 |
| src/components/UserForm/UserForm.js | 20 |
| src/components/Blog/Blog.js | 16 |
| scripts/start.js | 16 |
| src/components/CampusMap/CampusMap.js | 14 |
| config/env.js | 14 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 91}, "macro avg": {"f1-score": 0.38522344898143357, "precision": 0.3904000066874581, "recall": 0.3808269257394158, "support": 8144}, "micro avg": {"f1-score": 0.956532416502947, "precision": 0.956532416502947, "recall": 0.956532416502947, "support": 8144}, "weighted avg": {"f1-score": 0.9460797724961898, "precision": 0.9372905344306034, "recall": 0.956532416502947, "support": 8144}, "\u2205": {"f1-score": 0.974055317491235, "precision": 0.95159080529761, "recall": 0.9976061283115225, "support": 6266}, "\u23ce": {"f1-score": 0.9594229035166817, "precision": 0.9779411764705882, "recall": 0.9415929203539823, "support": 565}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u2423": {"f1-score": 0.9187562688064193, "precision": 0.9744680851063829, "recall": 0.8690702087286527, "support": 1054}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 175}, "\u0027": {"f1-score": 0.1746641074856046, "precision": 1.0, "recall": 0.09568874868559411, "support": 951}, "macro avg": {"f1-score": 0.2124614852008714, "precision": 0.3904000066874581, "recall": 0.16707745399439963, "support": 13761}, "micro avg": {"f1-score": 0.7112531385528419, "precision": 0.956532416502947, "recall": 0.5660925804810697, "support": 13761}, "weighted avg": {"f1-score": 0.6452678579107355, "precision": 0.8703981198204486, "recall": 0.5660925804810697, "support": 13761}, "\u2205": {"f1-score": 0.9015648662291772, "precision": 0.95159080529761, "recall": 0.8565360372704851, "support": 7298}, "\u23ce": {"f1-score": 0.5638579756226815, "precision": 0.9779411764705882, "recall": 0.39612807148175727, "support": 1343}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 226}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 391}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 354}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u2423": {"f1-score": 0.48452790267125095, "precision": 0.9744680851063829, "recall": 0.3224216825061598, "support": 2841}},
  "ppcr": 0.5918174551268076
}
```
</details>
