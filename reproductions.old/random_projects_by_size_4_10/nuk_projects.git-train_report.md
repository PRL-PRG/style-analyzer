# Train report for javascript / file:///tmp/top-repos-quality-repos-1s6pl1mw/nuk_projects.git HEAD d33f7d0147e45526ab884805dd599cfb651e36fd

### Classification report

PPCR: 0.379

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 1.000| 0.631| 0.993| 0.770| 1581| 2505| 0.631 |
| `␣` | 1.000| 0.921| 0.144| 0.959| 0.252| 241| 1543| 0.156 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 199| 0.005 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 87| 0.011 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 245| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 116| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 118| 0.000 |
| `macro avg` | 0.284| 0.274| 0.111| 0.279| 0.146| 1824| 4813| 0.379 |
| `weighted avg` | 0.988| 0.988| 0.375| 0.988| 0.481| 1824| 4813| 0.379 |
| `micro avg` | 0.988| 0.988| 0.375| 0.988| 0.543| 1824| 4813| 0.379 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|924 |1581 |0 |0 |0 |0 |0 |0 |
|1302 |19 |222 |0 |0 |0 |0 |0 |
|198 |1 |0 |0 |0 |0 |0 |0 |
|245 |0 |0 |0 |0 |0 |0 |0 |
|116 |0 |0 |0 |0 |0 |0 |0 |
|118 |0 |0 |0 |0 |0 |0 |0 |
|86 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| _old/Numerical Methods/development/LabWork3-numerical-integration/scripts/parse-equation.js | 9 |
| _old/Numerical Methods/development/LabWork3-numerical-integration/scripts/integrate.js | 3 |
| _old/Numerical Methods/development/LabWork3-numerical-integration/scripts/integrate-by-polygon.js | 2 |
| _old/Numerical Methods/development/LabWork5-approximate/src/pages/differential/model.js | 2 |
| systems-modelling/l2/js/index.js | 1 |
| _old/Numerical Methods/development/LabWork3-numerical-integration/scripts/integrate-by-simpson.js | 1 |
| _old/Numerical Methods/development/LabWork5-approximate/src/lib/Events/Events.spec.js | 1 |
| _old/Numerical Methods/development/LabWork3-numerical-integration/scripts/main.js | 1 |
| _old/Numerical Methods/development/LabWork5-approximate/src/lib/Events/Events.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.27890939049367774, "precision": 0.28384162653825573, "recall": 0.2744516893894487, "support": 1824}, "micro avg": {"f1-score": 0.9884868421052632, "precision": 0.9884868421052632, "recall": 0.9884868421052632, "support": 1824}, "weighted avg": {"f1-score": 0.9877628429043103, "precision": 0.9875412724226296, "recall": 0.9884868421052632, "support": 1824}, "\u2205": {"f1-score": 0.9934024505183789, "precision": 0.9868913857677902, "recall": 1.0, "support": 1581}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.958963282937365, "precision": 1.0, "recall": 0.921161825726141, "support": 241}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "macro avg": {"f1-score": 0.14592330197567163, "precision": 0.28384162653825573, "recall": 0.11071618451828867, "support": 4813}, "micro avg": {"f1-score": 0.5433177640500226, "precision": 0.9884868421052632, "recall": 0.37461043008518596, "support": 4813}, "weighted avg": {"f1-score": 0.48135595954454147, "precision": 0.834232894524894, "recall": 0.37461043008518596, "support": 4813}, "\u2205": {"f1-score": 0.7699050401753105, "precision": 0.9868913857677902, "recall": 0.6311377245508982, "support": 2505}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 199}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 87}, "\u2423": {"f1-score": 0.2515580736543909, "precision": 1.0, "recall": 0.14387556707712248, "support": 1543}},
  "ppcr": 0.3789736131311033
}
```
</details>
