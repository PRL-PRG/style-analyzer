# Train report for javascript / file:///tmp/top-repos-quality-repos-mcjhqiwu/magrit.git HEAD 59cc52f1219053e7d3d3eab506ead231649145a6

### Classification report

PPCR: 0.827

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 0.997| 0.951| 0.997| 0.973| 95758| 100445| 0.953 |
| `␣` | 0.967| 0.999| 0.792| 0.983| 0.871| 37400| 47175| 0.793 |
| `'` | 0.995| 1.000| 0.993| 0.998| 0.994| 24795| 24964| 0.993 |
| `⏎` | 0.943| 0.760| 0.206| 0.842| 0.338| 3194| 11802| 0.271 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 427| 5143| 0.083 |
| `⏎␣⁻␣⁻` | 1.000| 0.667| 0.056| 0.800| 0.106| 345| 4120| 0.084 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 118| 120| 0.983 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 194| 0.278 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 31| 1122| 0.028 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 407| 0.025 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 562| 0.009 |
| `macro avg` | 0.446| 0.402| 0.272| 0.420| 0.298| 162137| 196054| 0.827 |
| `micro avg` | 0.989| 0.989| 0.818| 0.989| 0.895| 162137| 196054| 0.827 |
| `weighted avg` | 0.985| 0.989| 0.818| 0.986| 0.857| 162137| 196054| 0.827 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4687 |95485 |239 |0 |34 |0 |0 |0 |0 |0 |0 |0 |
|9775 |47 |37349 |0 |4 |0 |0 |0 |0 |0 |0 |0 |
|169 |0 |0 |24795 |0 |0 |0 |0 |0 |0 |0 |0 |
|8608 |0 |765 |0 |2429 |0 |0 |0 |0 |0 |0 |0 |
|4716 |166 |208 |0 |53 |0 |0 |0 |0 |0 |0 |0 |
|3775 |86 |8 |0 |21 |0 |230 |0 |0 |0 |0 |0 |
|1091 |0 |5 |0 |26 |0 |0 |0 |0 |0 |0 |0 |
|557 |0 |0 |0 |5 |0 |0 |0 |0 |0 |0 |0 |
|397 |1 |5 |0 |4 |0 |0 |0 |0 |0 |0 |0 |
|140 |15 |38 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |0 |118 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/js/function.js | 381 |
| client/js/legend.js | 259 |
| client/js/layers_style_popup.js | 148 |
| magrit_app/static/book/book.js | 141 |
| client/js/map_project.js | 114 |
| client/js/interface.js | 104 |
| client/js/classification/discretization_panel.js | 65 |
| client/js/helpers.js | 61 |
| client/js/map_ctrl.js | 51 |
| client/js/projections.js | 43 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u0027": {"f1-score": 0.9976261366379657, "precision": 0.9952635170392967, "recall": 1.0, "support": 24795}, "macro avg": {"f1-score": 0.4199091246475636, "precision": 0.4456099105065832, "recall": 0.40208550084966005, "support": 162137}, "micro avg": {"f1-score": 0.9885960638225698, "precision": 0.9885960638225698, "recall": 0.9885960638225698, "support": 162137}, "weighted avg": {"f1-score": 0.9863014404400772, "precision": 0.984650157205162, "recall": 0.9885960638225698, "support": 162137}, "\u2205": {"f1-score": 0.9969304336023553, "precision": 0.9967118997912318, "recall": 0.9971490632636437, "support": 95758}, "\u23ce": {"f1-score": 0.8417951828106047, "precision": 0.9425688785409391, "recall": 0.7604884157795867, "support": 3194}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 427}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 345}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.9826486180722733, "precision": 0.9671647202009478, "recall": 0.9986363636363637, "support": 37400}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 120}, "\u0027": {"f1-score": 0.9942458447781543, "precision": 0.9952635170392967, "recall": 0.9932302515622496, "support": 24964}, "macro avg": {"f1-score": 0.2983322148770477, "precision": 0.4456099105065832, "recall": 0.2724726838436208, "support": 196054}, "micro avg": {"f1-score": 0.8949861945163335, "precision": 0.9885960638225698, "recall": 0.8175706693053955, "support": 196054}, "weighted avg": {"f1-score": 0.8572287473927357, "precision": 0.9478545645277279, "recall": 0.8175706693053955, "support": 196054}, "\u2205": {"f1-score": 0.9731203342760325, "precision": 0.9967118997912318, "recall": 0.9506197421474438, "support": 100445}, "\u23ce": {"f1-score": 0.3378538145907226, "precision": 0.9425688785409391, "recall": 0.20581257413997628, "support": 11802}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1122}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 562}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5143}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.10574712643678161, "precision": 1.0, "recall": 0.055825242718446605, "support": 4120}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 407}, "\u2423": {"f1-score": 0.8706872435658336, "precision": 0.9671647202009478, "recall": 0.7917117117117117, "support": 47175}},
  "ppcr": 0.8270017444173544
}
```
</details>
