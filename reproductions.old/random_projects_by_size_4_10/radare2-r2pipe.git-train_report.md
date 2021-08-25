# Train report for javascript / file:///tmp/top-repos-quality-repos-5uy1ba_y/radare2-r2pipe.git HEAD 4e8baade815fdbbac359e3f8d185287e761d7f97

### Classification report

PPCR: 0.803

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.993| 0.944| 0.984| 0.959| 6363| 6697| 0.950 |
| `␣` | 0.977| 0.930| 0.602| 0.953| 0.745| 2316| 3582| 0.647 |
| `'` | 1.000| 1.000| 0.987| 1.000| 0.993| 1177| 1193| 0.987 |
| `⏎␣⁺␣⁺` | 0.963| 0.991| 0.919| 0.977| 0.940| 470| 507| 0.927 |
| `⏎␣⁻␣⁻` | 0.968| 0.985| 0.930| 0.976| 0.949| 462| 489| 0.945 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 776| 0.039 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 236| 0.008 |
| `macro avg` | 0.698| 0.700| 0.626| 0.699| 0.655| 10820| 13480| 0.803 |
| `weighted avg` | 0.974| 0.977| 0.784| 0.976| 0.832| 10820| 13480| 0.803 |
| `micro avg` | 0.977| 0.977| 0.784| 0.977| 0.870| 10820| 13480| 0.803 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|334 |6321 |37 |0 |0 |1 |4 |0 |
|1266 |139 |2155 |0 |0 |14 |8 |0 |
|16 |0 |0 |1177 |0 |0 |0 |0 |
|746 |16 |9 |0 |0 |2 |3 |0 |
|37 |1 |3 |0 |0 |466 |0 |0 |
|27 |7 |0 |0 |0 |0 |455 |0 |
|234 |0 |1 |0 |0 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| nodejs/r2pipe/index.js | 54 |
| nodejs/r2pipe/examples/tgbot/index.js | 37 |
| nodejs/r2pipe/examples/ircbot/index.js | 25 |
| r2core-js/tuto/index.js | 22 |
| nodejs/r2pipe-promise/index.js | 15 |
| nodejs/r2pipe-promise/test-timeout.js | 14 |
| nodejs/r2pipe/testsuite/index.js | 8 |
| nodejs/r2pipe/testsuite/node/async.js | 6 |
| nodejs/r2pipe-promise/test-promise.js | 6 |
| nodejs/r2pipe-promise/ts/test.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1177}, "macro avg": {"f1-score": 0.6986722757003854, "precision": 0.6975829262241433, "recall": 0.7000315398407567, "support": 10820}, "micro avg": {"f1-score": 0.9772643253234751, "precision": 0.9772643253234751, "recall": 0.9772643253234751, "support": 10820}, "weighted avg": {"f1-score": 0.9756590574065693, "precision": 0.9744270581045653, "recall": 0.9772643253234751, "support": 10820}, "\u2205": {"f1-score": 0.9840429672297034, "precision": 0.9748611967921036, "recall": 0.9933993399339934, "support": 6363}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9769392033542977, "precision": 0.9628099173553719, "recall": 0.9914893617021276, "support": 470}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9763948497854077, "precision": 0.9680851063829787, "recall": 0.9848484848484849, "support": 462}, "\u2423": {"f1-score": 0.9533289095332891, "precision": 0.9773242630385488, "recall": 0.9304835924006909, "support": 2316}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9932489451476794, "precision": 1.0, "recall": 0.9865884325230512, "support": 1193}, "macro avg": {"f1-score": 0.6552141150674673, "precision": 0.6975829262241433, "recall": 0.6259522278410216, "support": 13480}, "micro avg": {"f1-score": 0.8702880658436215, "precision": 0.9772643253234751, "recall": 0.7844213649851632, "support": 13480}, "weighted avg": {"f1-score": 0.8320991809586843, "precision": 0.9038545393354043, "recall": 0.7844213649851632, "support": 13480}, "\u2205": {"f1-score": 0.9591078066914498, "precision": 0.9748611967921036, "recall": 0.9438554576676124, "support": 6697}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 776}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 236}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9404641775983854, "precision": 0.9628099173553719, "recall": 0.9191321499013807, "support": 507}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9489051094890509, "precision": 0.9680851063829787, "recall": 0.9304703476482618, "support": 489}, "\u2423": {"f1-score": 0.744772766545706, "precision": 0.9773242630385488, "recall": 0.6016192071468454, "support": 3582}},
  "ppcr": 0.8026706231454006
}
```
</details>
