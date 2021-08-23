# Train report for javascript / file:///tmp/top-repos-quality-repos-hmj6uupd/pdc.git HEAD 6330d567bd2d77c3a0fbce5d9100300e308aab0b

### Classification report

PPCR: 0.547

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.999| 0.995| 0.804| 0.997| 0.891| 7010| 8679| 0.808 |
| `␣` | 0.989| 1.000| 0.488| 0.994| 0.654| 3132| 6414| 0.488 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 1148| 0.003 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 204| 0.010 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 838| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 553| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 296| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 271| 0.000 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 166| 0.000 |
| `micro avg` | 0.996| 0.996| 0.544| 0.996| 0.704| 10148| 18569| 0.547 |
| `weighted avg` | 0.995| 0.996| 0.544| 0.996| 0.642| 10148| 18569| 0.547 |
| `macro avg` | 0.221| 0.222| 0.144| 0.221| 0.172| 10148| 18569| 0.547 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁺␣⁺| ⏎⏎| ⏎⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1669 |6974 |36 |0 |0 |0 |0 |0 |0 |0 |
|3282 |0 |3132 |0 |0 |0 |0 |0 |0 |0 |
|1144 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|838 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|553 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|296 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|271 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|202 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|166 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Amazon Connect Sample/js/spokes.js | 21 |
| JavaScript Sample/spokes.js | 21 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.22125372998854054, "precision": 0.22086408497583282, "recall": 0.2216516088128071, "support": 10148}, "micro avg": {"f1-score": 0.9958612534489555, "precision": 0.9958612534489555, "recall": 0.9958612534489555, "support": 10148}, "weighted avg": {"f1-score": 0.9955713268601719, "precision": 0.9953077752210756, "recall": 0.9958612534489555, "support": 10148}, "\u2205": {"f1-score": 0.9969978556111508, "precision": 0.9991404011461318, "recall": 0.9948644793152639, "support": 7010}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9942857142857142, "precision": 0.9886363636363636, "recall": 1.0, "support": 3132}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 838}, "macro avg": {"f1-score": 0.17160661100950353, "precision": 0.22086408497583282, "recall": 0.14353951386180078, "support": 18569}, "micro avg": {"f1-score": 0.7038339659435178, "precision": 0.9958612534489555, "recall": 0.5442404006677797, "support": 18569}, "weighted avg": {"f1-score": 0.6421280198760998, "precision": 0.8084793568803336, "recall": 0.5442404006677797, "support": 18569}, "\u2205": {"f1-score": 0.8907337633309919, "precision": 0.9991404011461318, "recall": 0.8035487959442332, "support": 8679}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1148}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 204}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 271}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 296}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 553}, "\u2423": {"f1-score": 0.6537257357545398, "precision": 0.9886363636363636, "recall": 0.4883068288119738, "support": 6414}},
  "ppcr": 0.5465022349076417
}
```
</details>
