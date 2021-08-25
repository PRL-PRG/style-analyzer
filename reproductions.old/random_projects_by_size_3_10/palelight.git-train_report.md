# Train report for javascript / file:///tmp/top-repos-quality-repos-e6buetbp/palelight.git HEAD 4da9b5af0f6a23d3523c9cb3038e942074a09839

### Classification report

PPCR: 0.725

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 0.994| 0.917| 0.989| 0.949| 11540| 12502| 0.923 |
| `␣` | 0.969| 0.950| 0.535| 0.959| 0.690| 3471| 6158| 0.564 |
| `⏎␣⁻␣⁻` | 0.964| 0.981| 0.967| 0.972| 0.966| 771| 782| 0.986 |
| `⏎␣⁺␣⁺` | 0.971| 0.936| 0.858| 0.953| 0.911| 764| 833| 0.917 |
| `'` | 1.000| 1.000| 0.339| 1.000| 0.507| 308| 908| 0.339 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 1105| 0.024 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 628| 0.006 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 360| 0.000 |
| `weighted avg` | 0.978| 0.980| 0.711| 0.979| 0.777| 16884| 23276| 0.725 |
| `micro avg` | 0.980| 0.980| 0.711| 0.980| 0.824| 16884| 23276| 0.725 |
| `macro avg` | 0.611| 0.607| 0.452| 0.609| 0.503| 16884| 23276| 0.725 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|962 |11465 |65 |0 |0 |2 |8 |0 |0 |
|2687 |145 |3297 |0 |0 |14 |15 |0 |0 |
|1079 |15 |3 |0 |0 |3 |5 |0 |0 |
|600 |0 |0 |0 |308 |0 |0 |0 |0 |
|69 |12 |37 |0 |0 |715 |0 |0 |0 |
|11 |14 |0 |0 |0 |1 |756 |0 |0 |
|624 |3 |0 |0 |0 |1 |0 |0 |0 |
|360 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| assets/js/plugins/jquery.magnific-popup.js | 163 |
| assets/js/lunr/lunr.js | 129 |
| assets/js/plugins/jquery.fitvids.js | 25 |
| banner.js | 11 |
| assets/js/plugins/jquery.greedy-navigation.js | 10 |
| assets/js/_main.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 308}, "macro avg": {"f1-score": 0.6092130898682666, "precision": 0.6110839125463563, "recall": 0.6074724802928906, "support": 16884}, "micro avg": {"f1-score": 0.9796849087893864, "precision": 0.9796849087893864, "recall": 0.9796849087893864, "support": 16884}, "weighted avg": {"f1-score": 0.9787239231089814, "precision": 0.9778716270029406, "recall": 0.9796849087893864, "support": 16884}, "\u2205": {"f1-score": 0.988617745968785, "precision": 0.9837823923116527, "recall": 0.9935008665511266, "support": 11540}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9533333333333333, "precision": 0.9714673913043478, "recall": 0.9358638743455497, "support": 764}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9723472668810289, "precision": 0.9642857142857143, "recall": 0.980544747081712, "support": 771}, "\u2423": {"f1-score": 0.9594063727629857, "precision": 0.9691358024691358, "recall": 0.9498703543647364, "support": 3471}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 360}, "\u0027": {"f1-score": 0.5065789473684211, "precision": 1.0, "recall": 0.3392070484581498, "support": 908}, "macro avg": {"f1-score": 0.502812530899366, "precision": 0.6110839125463563, "recall": 0.4520945849603554, "support": 23276}, "micro avg": {"f1-score": 0.8237549800796813, "precision": 0.9796849087893864, "recall": 0.7106461591338717, "support": 23276}, "weighted avg": {"f1-score": 0.7771595670603072, "precision": 0.8909816766546301, "recall": 0.7106461591338717, "support": 23276}, "\u2205": {"f1-score": 0.9492465640006624, "precision": 0.9837823923116527, "recall": 0.9170532714765638, "support": 12502}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1105}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 628}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9114085404716379, "precision": 0.9714673913043478, "recall": 0.858343337334934, "support": 833}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9655172413793103, "precision": 0.9642857142857143, "recall": 0.9667519181585678, "support": 782}, "\u2423": {"f1-score": 0.6897489539748954, "precision": 0.9691358024691358, "recall": 0.5354011042546282, "support": 6158}},
  "ppcr": 0.7253823681044853
}
```
</details>
