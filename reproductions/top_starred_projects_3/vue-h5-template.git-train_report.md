# Train report for javascript / file:///tmp/top-repos-quality-repos-kgdr_bld/vue-h5-template.git HEAD c2109f0e7848b965a53c7076cd87706029ed7daa

### Classification report

PPCR: 0.299

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.753| 1.000| 0.859| 506| 672| 0.753 |
| `∅` | 0.923| 1.000| 0.270| 0.960| 0.418| 373| 1381| 0.270 |
| `␣` | 1.000| 0.983| 0.239| 0.991| 0.385| 234| 964| 0.243 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 480| 0.029 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 154| 0.078 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 159| 0.006 |
| `weighted avg` | 0.951| 0.973| 0.291| 0.961| 0.400| 1140| 3810| 0.299 |
| `micro avg` | 0.973| 0.973| 0.291| 0.973| 0.448| 1140| 3810| 0.299 |
| `macro avg` | 0.487| 0.497| 0.210| 0.492| 0.277| 1140| 3810| 0.299 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1008 |373 |0 |0 |0 |0 |0 |
|730 |4 |230 |0 |0 |0 |0 |
|166 |0 |0 |506 |0 |0 |0 |
|466 |14 |0 |0 |0 |0 |0 |
|158 |1 |0 |0 |0 |0 |0 |
|142 |12 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| vue.config.js | 15 |
| src/utils/index.js | 5 |
| src/api/user.js | 3 |
| src/utils/request.js | 2 |
| src/store/modules/app.js | 2 |
| babel.config.js | 1 |
| src/store/index.js | 1 |
| src/router/index.js | 1 |
| src/router/router.config.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 506}, "macro avg": {"f1-score": 0.49191371174129794, "precision": 0.48721122112211224, "recall": 0.49715099715099714, "support": 1140}, "micro avg": {"f1-score": 0.9728070175438597, "precision": 0.9728070175438597, "recall": 0.9728070175438597, "support": 1140}, "weighted avg": {"f1-score": 0.9614922480167489, "precision": 0.9512093972555149, "recall": 0.9728070175438597, "support": 1140}, "\u2205": {"f1-score": 0.96010296010296, "precision": 0.9232673267326733, "recall": 1.0, "support": 373}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.9913793103448275, "precision": 1.0, "recall": 0.9829059829059829, "support": 234}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8590831918505942, "precision": 1.0, "recall": 0.7529761904761905, "support": 672}, "macro avg": {"f1-score": 0.27704499903495483, "precision": 0.48721122112211224, "recall": 0.2102765894632431, "support": 3810}, "micro avg": {"f1-score": 0.448080808080808, "precision": 0.9728070175438597, "recall": 0.2910761154855643, "support": 3810}, "weighted avg": {"f1-score": 0.4004859875721538, "precision": 0.7640504404771186, "recall": 0.2910761154855643, "support": 3810}, "\u2205": {"f1-score": 0.41792717086834735, "precision": 0.9232673267326733, "recall": 0.27009413468501087, "support": 1381}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 480}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 159}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u2423": {"f1-score": 0.3852596314907873, "precision": 1.0, "recall": 0.23858921161825727, "support": 964}},
  "ppcr": 0.2992125984251969
}
```
</details>
