# Train report for javascript / file:///tmp/top-repos-quality-repos-ot6yu6mo/veritaswrestlingsystems.git HEAD 393a6f217a98bf3581e298d269563499af7f4100

### Classification report

PPCR: 0.777

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 0.986| 0.870| 0.985| 0.923| 54775| 62108| 0.882 |
| `␣` | 0.962| 0.974| 0.853| 0.968| 0.904| 30027| 34263| 0.876 |
| `'` | 0.950| 1.000| 0.783| 0.974| 0.858| 8522| 10884| 0.783 |
| `⏎` | 0.941| 0.780| 0.139| 0.853| 0.242| 1040| 5843| 0.178 |
| `⏎⇥⁺` | 0.961| 0.929| 0.707| 0.945| 0.815| 479| 629| 0.762 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 450| 1052| 0.428 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.982| 0.867| 0.154| 0.921| 0.266| 309| 1745| 0.177 |
| `⏎␣⁻␣⁻` | 0.990| 0.820| 0.152| 0.897| 0.263| 244| 1319| 0.185 |
| `⏎⇥⁻` | 1.000| 0.949| 0.338| 0.974| 0.505| 215| 604| 0.356 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 134| 1784| 0.075 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 79| 1268| 0.062 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 2474| 0.011 |
| `macro avg` | 0.647| 0.609| 0.333| 0.626| 0.398| 96300| 123973| 0.777 |
| `micro avg` | 0.973| 0.973| 0.756| 0.973| 0.851| 96300| 123973| 0.777 |
| `weighted avg` | 0.966| 0.973| 0.756| 0.970| 0.812| 96300| 123973| 0.777 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|7333 |54029 |746 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4236 |739 |29243 |0 |42 |0 |0 |0 |0 |0 |0 |3 |0 |
|2362 |0 |0 |8522 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4803 |59 |167 |0 |811 |0 |0 |0 |0 |0 |0 |3 |0 |
|2448 |3 |12 |0 |9 |0 |0 |2 |0 |0 |0 |0 |0 |
|1650 |35 |87 |0 |0 |0 |0 |0 |0 |0 |0 |12 |0 |
|1436 |21 |18 |0 |0 |0 |0 |268 |0 |2 |0 |0 |0 |
|1189 |0 |79 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1075 |17 |24 |0 |0 |0 |0 |3 |0 |200 |0 |0 |0 |
|602 |0 |0 |450 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|150 |9 |25 |0 |0 |0 |0 |0 |0 |0 |0 |445 |0 |
|389 |9 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |204 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| staticfiles/vws_main/js/addons/datatables-select.js | 249 |
| vws_main/static/vws_main/js/addons/datatables-select.js | 249 |
| staticfiles/vws_main/js/addons/datatables-select.516bb1e036b4.js | 249 |
| staticfiles/vws_main/js/bootstrap.5e7d168ed320.js | 239 |
| staticfiles/vws_main/js/modules/jquery.easing.f75f52953e1f.js | 99 |
| vws_main/static/vws_main/js/modules/jquery.easing.js | 99 |
| staticfiles/vws_main/js/modules/jquery.easing.js | 99 |
| staticfiles/admin/js/urlify.js | 87 |
| staticfiles/admin/js/urlify.4087f3e18796.js | 87 |
| staticfiles/admin/js/inlines.js | 73 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 450}, "\u0027": {"f1-score": 0.9742768949354065, "precision": 0.9498439589835043, "recall": 1.0, "support": 8522}, "macro avg": {"f1-score": 0.6263608592513923, "precision": 0.6474325572901672, "recall": 0.6087433792819003, "support": 96300}, "micro avg": {"f1-score": 0.9732294911734164, "precision": 0.9732294911734164, "recall": 0.9732294911734164, "support": 96300}, "weighted avg": {"f1-score": 0.9696069470606278, "precision": 0.9663557517830783, "recall": 0.9732294911734164, "support": 96300}, "\u2205": {"f1-score": 0.9850678238039674, "precision": 0.9837584894666885, "recall": 0.9863806481058878, "support": 54775}, "\u23ce": {"f1-score": 0.8527865404837013, "precision": 0.9408352668213457, "recall": 0.7798076923076923, "support": 1040}, "\u23ce\u21e5\u207a": {"f1-score": 0.9447983014861996, "precision": 0.9611231101511879, "recall": 0.9290187891440501, "support": 479}, "\u23ce\u21e5\u207b": {"f1-score": 0.9737470167064439, "precision": 1.0, "recall": 0.9488372093023256, "support": 215}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 134}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8968609865470852, "precision": 0.9900990099009901, "recall": 0.819672131147541, "support": 244}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9209621993127147, "precision": 0.9816849816849816, "recall": 0.8673139158576052, "support": 309}, "\u2423": {"f1-score": 0.9678305477411882, "precision": 0.9618458704733086, "recall": 0.9738901655177007, "support": 30027}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1052}, "\u0027": {"f1-score": 0.8583803384367447, "precision": 0.9498439589835043, "recall": 0.7829841969864021, "support": 10884}, "macro avg": {"f1-score": 0.3980523629498555, "precision": 0.6474325572901672, "recall": 0.3329684477945019, "support": 123973}, "micro avg": {"f1-score": 0.8509622150694819, "precision": 0.9732294911734164, "recall": 0.7559871907592782, "support": 123973}, "weighted avg": {"f1-score": 0.8124323680769368, "precision": 0.9205062937964108, "recall": 0.7559871907592782, "support": 123973}, "\u2205": {"f1-score": 0.9233437865828128, "precision": 0.9837584894666885, "recall": 0.8699201391125136, "support": 62108}, "\u23ce": {"f1-score": 0.2419090231170768, "precision": 0.9408352668213457, "recall": 0.13879856238233784, "support": 5843}, "\u23ce\u21e5\u207a": {"f1-score": 0.815018315018315, "precision": 0.9611231101511879, "recall": 0.7074721780604134, "support": 629}, "\u23ce\u21e5\u207b": {"f1-score": 0.504950495049505, "precision": 1.0, "recall": 0.33774834437086093, "support": 604}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2474}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1268}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1784}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.26298487836949375, "precision": 0.9900990099009901, "recall": 0.1516300227445034, "support": 1319}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.265609514370664, "precision": 0.9816849816849816, "recall": 0.15358166189111747, "support": 1745}, "\u2423": {"f1-score": 0.9044320044536543, "precision": 0.9618458704733086, "recall": 0.853486267985874, "support": 34263}},
  "ppcr": 0.7767820412509175
}
```
</details>
