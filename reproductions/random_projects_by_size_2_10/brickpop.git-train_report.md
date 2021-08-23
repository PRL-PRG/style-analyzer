# Train report for javascript / file:///tmp/top-repos-quality-repos-2j1qajhf/brickpop.git HEAD ff32b9b01266029e9d4237df5188fe4aec1b1a1e

### Classification report

PPCR: 0.369

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 1.000| 0.750| 0.997| 0.854| 1293| 1725| 0.750 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 1246| 0.006 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 96| 0.010 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 156| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 86| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 123| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 96| 0.000 |
| `macro avg` | 0.142| 0.143| 0.107| 0.142| 0.122| 1302| 3528| 0.369 |
| `micro avg` | 0.993| 0.993| 0.366| 0.993| 0.535| 1302| 3528| 0.369 |
| `weighted avg` | 0.986| 0.993| 0.366| 0.990| 0.418| 1302| 3528| 0.369 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|432 |1293 |0 |0 |0 |0 |0 |0 |
|1238 |8 |0 |0 |0 |0 |0 |0 |
|156 |0 |0 |0 |0 |0 |0 |0 |
|86 |0 |0 |0 |0 |0 |0 |0 |
|123 |0 |0 |0 |0 |0 |0 |0 |
|96 |0 |0 |0 |0 |0 |0 |0 |
|95 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| play.js | 6 |
| public/client.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.14236168455821635, "precision": 0.14186965108624094, "recall": 0.14285714285714285, "support": 1302}, "micro avg": {"f1-score": 0.9930875576036866, "precision": 0.9930875576036866, "recall": 0.9930875576036866, "support": 1302}, "weighted avg": {"f1-score": 0.9896433232998588, "precision": 0.9862228970672556, "recall": 0.9930875576036866, "support": 1302}, "\u2205": {"f1-score": 0.9965317919075144, "precision": 0.9930875576036866, "recall": 1.0, "support": 1293}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "macro avg": {"f1-score": 0.12204445702959084, "precision": 0.14186965108624094, "recall": 0.10708074534161491, "support": 3528}, "micro avg": {"f1-score": 0.5354037267080746, "precision": 0.9930875576036866, "recall": 0.3664965986394558, "support": 3528}, "weighted avg": {"f1-score": 0.41771168328580194, "precision": 0.48556577008683655, "recall": 0.3664965986394558, "support": 3528}, "\u2205": {"f1-score": 0.8543111992071358, "precision": 0.9930875576036866, "recall": 0.7495652173913043, "support": 1725}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1246}},
  "ppcr": 0.36904761904761907
}
```
</details>
