# Train report for javascript / file:///tmp/top-repos-quality-repos-paqp5s9s/backup.git HEAD d0ef45a429035efb5b93626b422f2365157ba52e

### Classification report

PPCR: 0.195

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 1.000| 0.344| 0.996| 0.510| 469| 1365| 0.344 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 190| 0.021 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 467| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 332| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 70| 0.000 |
| `macro avg` | 0.198| 0.200| 0.069| 0.199| 0.102| 473| 2424| 0.195 |
| `weighted avg` | 0.983| 0.992| 0.193| 0.987| 0.287| 473| 2424| 0.195 |
| `micro avg` | 0.992| 0.992| 0.193| 0.992| 0.324| 473| 2424| 0.195 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|896 |469 |0 |0 |0 |0 |
|467 |0 |0 |0 |0 |0 |
|332 |0 |0 |0 |0 |0 |
|186 |4 |0 |0 |0 |0 |
|70 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| 2018.04.26/test2.js | 1 |
| 2018.05.03/insert.js | 1 |
| 2018.05.01/insert.js | 1 |
| 2018.04.26/test.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1991507430997877, "precision": 0.19830866807610995, "recall": 0.2, "support": 473}, "micro avg": {"f1-score": 0.9915433403805497, "precision": 0.9915433403805497, "recall": 0.9915433403805497, "support": 473}, "weighted avg": {"f1-score": 0.987332965262161, "precision": 0.9831581958530187, "recall": 0.9915433403805497, "support": 473}, "\u2205": {"f1-score": 0.9957537154989385, "precision": 0.9915433403805497, "recall": 1.0, "support": 469}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 70}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 332}, "macro avg": {"f1-score": 0.10206746463547334, "precision": 0.19830866807610995, "recall": 0.06871794871794872, "support": 2424}, "micro avg": {"f1-score": 0.32378322402485327, "precision": 0.9915433403805497, "recall": 0.19348184818481848, "support": 2424}, "weighted avg": {"f1-score": 0.28738054708626465, "precision": 0.5583567077637996, "recall": 0.19348184818481848, "support": 2424}, "\u2205": {"f1-score": 0.5103373231773667, "precision": 0.9915433403805497, "recall": 0.3435897435897436, "support": 1365}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 190}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 467}},
  "ppcr": 0.19513201320132012
}
```
</details>
