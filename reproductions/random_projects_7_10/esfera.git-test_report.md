# Test report for javascript / file:///tmp/top-repos-quality-repos-uhq61wg5/esfera.git HEAD 163be1282f73bccc57b2df48f51df66cde3e3b24

### Classification report

PPCR: 0.716

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.901| 1.000| 0.920| 0.948| 0.911| 1408| 1530| 0.920 |
| `␣` | 1.000| 0.500| 0.147| 0.667| 0.256| 134| 456| 0.294 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 45| 150| 0.300 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 96| 0.438 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 24| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 20| 0.000 |
| `macro avg` | 0.317| 0.250| 0.178| 0.269| 0.194| 1629| 2276| 0.716 |
| `micro avg` | 0.905| 0.905| 0.648| 0.905| 0.755| 1629| 2276| 0.716 |
| `weighted avg` | 0.861| 0.905| 0.648| 0.874| 0.664| 1629| 2276| 0.716 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|122 |1408 |0 |0 |0 |0 |0 |
|322 |67 |67 |0 |0 |0 |0 |
|105 |45 |0 |0 |0 |0 |0 |
|24 |0 |0 |0 |0 |0 |0 |
|20 |0 |0 |0 |0 |0 |0 |
|54 |42 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.26913580246913577, "precision": 0.31690140845070425, "recall": 0.25, "support": 1629}, "micro avg": {"f1-score": 0.9054634745242481, "precision": 0.905463474524248, "recall": 0.905463474524248, "support": 1629}, "weighted avg": {"f1-score": 0.8743560011822749, "precision": 0.8613769788775626, "recall": 0.905463474524248, "support": 1629}, "\u2205": {"f1-score": 0.9481481481481481, "precision": 0.9014084507042254, "recall": 1.0, "support": 1408}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u2423": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 134}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "macro avg": {"f1-score": 0.19449192265737278, "precision": 0.31690140845070425, "recall": 0.17786521041165004, "support": 2276}, "micro avg": {"f1-score": 0.7554417413572343, "precision": 0.905463474524248, "recall": 0.6480667838312829, "support": 2276}, "weighted avg": {"f1-score": 0.6635596897270535, "precision": 0.8063070868090795, "recall": 0.6480667838312829, "support": 2276}, "\u2205": {"f1-score": 0.9107373868046572, "precision": 0.9014084507042254, "recall": 0.9202614379084967, "support": 1530}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 150}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u2423": {"f1-score": 0.2562141491395794, "precision": 1.0, "recall": 0.14692982456140352, "support": 456}},
  "ppcr": 0.7157293497363796
}
```
</details>
