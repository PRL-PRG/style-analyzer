# Train report for javascript / file:///tmp/top-repos-quality-repos-gnqspq34/scheduler.git HEAD 1f7eb76b892bdd1132e468bc2f8280c66377e62f

### Classification report

PPCR: 0.340

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.998| 1.000| 0.563| 0.999| 0.720| 475| 844| 0.563 |
| `'` | 1.000| 0.933| 0.350| 0.966| 0.519| 30| 80| 0.375 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 454| 0.002 |
| `"` | 0.333| 1.000| 0.500| 0.500| 0.400| 1| 2| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 84| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 28| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.995| 0.994| 0.338| 0.994| 0.435| 507| 1492| 0.340 |
| `macro avg` | 0.233| 0.293| 0.141| 0.246| 0.164| 507| 1492| 0.340 |
| `micro avg` | 0.994| 0.994| 0.338| 0.994| 0.504| 507| 1492| 0.340 |

### Confusion matrix

|refusal|  ␣| ∅| ⏎| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎⏎⇥⁺| '| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|453 |0 |1 |0 |0 |0 |0 |
|369 |0 |475 |0 |0 |0 |0 |
|84 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |1 |0 |0 |
|28 |0 |0 |0 |0 |0 |0 |
|50 |0 |0 |0 |2 |0 |28 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/js/constants.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.5, "precision": 0.3333333333333333, "recall": 1.0, "support": 1}, "\u0027": {"f1-score": 0.9655172413793104, "precision": 1.0, "recall": 0.9333333333333333, "support": 30}, "macro avg": {"f1-score": 0.24644657166684797, "precision": 0.23312324929971986, "recall": 0.29333333333333333, "support": 507}, "micro avg": {"f1-score": 0.9940828402366864, "precision": 0.9940828402366864, "recall": 0.9940828402366864, "support": 507}, "weighted avg": {"f1-score": 0.9940158639126918, "precision": 0.9947444461019121, "recall": 0.9940828402366864, "support": 507}, "\u2205": {"f1-score": 0.9989484752891693, "precision": 0.9978991596638656, "recall": 1.0, "support": 475}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"\"": {"f1-score": 0.4, "precision": 0.3333333333333333, "recall": 0.5, "support": 2}, "\u0027": {"f1-score": 0.5185185185185185, "precision": 1.0, "recall": 0.35, "support": 80}, "macro avg": {"f1-score": 0.1638215488215488, "precision": 0.23312324929971986, "recall": 0.14127962085308057, "support": 1492}, "micro avg": {"f1-score": 0.5042521260630316, "precision": 0.9940828402366864, "recall": 0.3378016085790885, "support": 1492}, "weighted avg": {"f1-score": 0.43545960047300536, "precision": 0.6185613655649927, "recall": 0.3378016085790885, "support": 1492}, "\u2205": {"f1-score": 0.7196969696969697, "precision": 0.9978991596638656, "recall": 0.5627962085308057, "support": 844}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 454}},
  "ppcr": 0.3398123324396783
}
```
</details>
