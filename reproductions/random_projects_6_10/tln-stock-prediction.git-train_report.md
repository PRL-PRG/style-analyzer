# Train report for javascript / file:///tmp/top-repos-quality-repos-wlippkoz/tln-stock-prediction.git HEAD 3f694d35b68aa8fd8e8a6fb9b6d0025c16fdb6d1

### Classification report

PPCR: 0.661

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 0.999| 0.900| 0.976| 0.926| 2368| 2629| 0.901 |
| `␣` | 0.988| 0.931| 0.578| 0.959| 0.729| 903| 1456| 0.620 |
| `'` | 1.000| 1.000| 0.473| 1.000| 0.642| 351| 742| 0.473 |
| `⏎` | 0.944| 0.957| 0.447| 0.950| 0.606| 280| 600| 0.467 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 220| 0.200 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 126| 0.127 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 230| 0.026 |
| `micro avg` | 0.964| 0.964| 0.637| 0.964| 0.767| 3968| 6003| 0.661 |
| `weighted avg` | 0.949| 0.964| 0.637| 0.956| 0.722| 3968| 6003| 0.661 |
| `macro avg` | 0.555| 0.555| 0.342| 0.555| 0.415| 3968| 6003| 0.661 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|261 |2366 |2 |0 |0 |0 |0 |0 |
|553 |54 |841 |0 |8 |0 |0 |0 |
|391 |0 |0 |351 |0 |0 |0 |0 |
|320 |12 |0 |0 |268 |0 |0 |0 |
|224 |6 |0 |0 |0 |0 |0 |0 |
|176 |42 |0 |0 |2 |0 |0 |0 |
|110 |2 |8 |0 |6 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Source Code/Front End/build/utils.js | 16 |
| build/utils.js | 16 |
| build/webpack.dev.conf.js | 14 |
| Source Code/Front End/build/webpack.prod.conf.js | 14 |
| build/webpack.prod.conf.js | 14 |
| Source Code/Front End/build/webpack.dev.conf.js | 14 |
| Source Code/Front End/build/check-versions.js | 9 |
| build/check-versions.js | 9 |
| Source Code/Front End/build/webpack.base.conf.js | 6 |
| build/webpack.base.conf.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 351}, "macro avg": {"f1-score": 0.5549965260335841, "precision": 0.5550249410992258, "recall": 0.5553768914856955, "support": 3968}, "micro avg": {"f1-score": 0.9642137096774194, "precision": 0.9642137096774194, "recall": 0.9642137096774194, "support": 3968}, "weighted avg": {"f1-score": 0.9560027268405737, "precision": 0.9488261763119339, "recall": 0.9642137096774194, "support": 3968}, "\u2205": {"f1-score": 0.9756701030927835, "precision": 0.9532634971796938, "recall": 0.9991554054054054, "support": 2368}, "\u23ce": {"f1-score": 0.9503546099290779, "precision": 0.9436619718309859, "recall": 0.9571428571428572, "support": 280}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u2423": {"f1-score": 0.958950969213227, "precision": 0.9882491186839013, "recall": 0.9313399778516057, "support": 903}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6422689844464775, "precision": 1.0, "recall": 0.47304582210242585, "support": 742}, "macro avg": {"f1-score": 0.41479077605827436, "precision": 0.5550249410992258, "recall": 0.34246919165749307, "support": 6003}, "micro avg": {"f1-score": 0.7674255340487414, "precision": 0.9642137096774194, "recall": 0.6373479926703315, "support": 6003}, "weighted avg": {"f1-score": 0.7222992702540756, "precision": 0.8750987229698096, "recall": 0.6373479926703315, "support": 6003}, "\u2205": {"f1-score": 0.9258462140481315, "precision": 0.9532634971796938, "recall": 0.899961962723469, "support": 2629}, "\u23ce": {"f1-score": 0.6063348416289593, "precision": 0.9436619718309859, "recall": 0.44666666666666666, "support": 600}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 126}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 230}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 220}, "\u2423": {"f1-score": 0.729085392284352, "precision": 0.9882491186839013, "recall": 0.5776098901098901, "support": 1456}},
  "ppcr": 0.6610028319173746
}
```
</details>
