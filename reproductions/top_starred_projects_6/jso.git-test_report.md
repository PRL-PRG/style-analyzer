# Test report for javascript / file:///tmp/top-repos-quality-repos-ax5dqr3q/jso.git HEAD b1be102c5c8eedb15c38dea2ba56d83b749eed94

### Classification report

PPCR: 0.703

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.898| 0.985| 0.963| 0.940| 0.930| 476| 487| 0.977 |
| `␣` | 0.863| 0.743| 0.435| 0.799| 0.578| 152| 260| 0.585 |
| `⏎` | 0.833| 0.417| 0.061| 0.556| 0.114| 12| 82| 0.146 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 28| 0.357 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 27| 0.222 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 28| 0.107 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 14| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `micro avg` | 0.891| 0.891| 0.626| 0.891| 0.735| 659| 938| 0.703 |
| `weighted avg` | 0.863| 0.891| 0.626| 0.873| 0.653| 659| 938| 0.703 |
| `macro avg` | 0.324| 0.268| 0.182| 0.287| 0.203| 659| 938| 0.703 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|11 |469 |7 |0 |0 |0 |0 |0 |0 |
|108 |39 |113 |0 |0 |0 |0 |0 |0 |
|70 |7 |0 |5 |0 |0 |0 |0 |0 |
|25 |1 |2 |0 |0 |0 |0 |0 |0 |
|18 |1 |9 |0 |0 |0 |0 |0 |0 |
|21 |5 |0 |1 |0 |0 |0 |0 |0 |
|14 |0 |0 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "macro avg": {"f1-score": 0.28675273593909456, "precision": 0.32429952326635664, "recall": 0.2681727296181631, "support": 659}, "micro avg": {"f1-score": 0.8907435508345979, "precision": 0.8907435508345979, "recall": 0.8907435508345979, "support": 659}, "weighted avg": {"f1-score": 0.8731936133662056, "precision": 0.8631031895312368, "recall": 0.8907435508345979, "support": 659}, "\u2205": {"f1-score": 0.9398797595190381, "precision": 0.8984674329501916, "recall": 0.9852941176470589, "support": 476}, "\u23ce": {"f1-score": 0.5555555555555556, "precision": 0.8333333333333334, "recall": 0.4166666666666667, "support": 12}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.7985865724381627, "precision": 0.8625954198473282, "recall": 0.743421052631579, "support": 152}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "macro avg": {"f1-score": 0.2026593473779002, "precision": 0.32429952326635664, "recall": 0.18232875109314986, "support": 938}, "micro avg": {"f1-score": 0.7351283656856605, "precision": 0.8907435508345979, "recall": 0.6257995735607675, "support": 938}, "weighted avg": {"f1-score": 0.6528048283435527, "precision": 0.7784240749897463, "recall": 0.6257995735607675, "support": 938}, "\u2205": {"f1-score": 0.929633300297324, "precision": 0.8984674329501916, "recall": 0.9630390143737166, "support": 487}, "\u23ce": {"f1-score": 0.11363636363636363, "precision": 0.8333333333333334, "recall": 0.06097560975609756, "support": 82}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u2423": {"f1-score": 0.578005115089514, "precision": 0.8625954198473282, "recall": 0.4346153846153846, "support": 260}},
  "ppcr": 0.7025586353944563
}
```
</details>
