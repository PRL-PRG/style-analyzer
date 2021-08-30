# Train report for javascript / file:///tmp/top-repos-quality-repos-kkyut8m1/enquire.js.git HEAD 2f339f1e29b2b6676f541e64b770635075af494d

### Classification report

PPCR: 0.681

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.946| 0.999| 0.942| 0.972| 0.944| 1927| 2045| 0.942 |
| `␣` | 0.993| 0.602| 0.191| 0.749| 0.320| 246| 776| 0.317 |
| `'` | 1.000| 1.000| 0.497| 1.000| 0.664| 169| 340| 0.497 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 138| 0.058 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 157| 0.019 |
| `micro avg` | 0.953| 0.953| 0.649| 0.953| 0.772| 2353| 3456| 0.681 |
| `macro avg` | 0.588| 0.520| 0.326| 0.544| 0.386| 2353| 3456| 0.681 |
| `weighted avg` | 0.951| 0.953| 0.649| 0.946| 0.696| 2353| 3456| 0.681 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|118 |1926 |1 |0 |0 |0 |
|530 |98 |148 |0 |0 |0 |
|171 |0 |0 |169 |0 |0 |
|154 |3 |0 |0 |0 |0 |
|130 |8 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/MediaQueryDispatch.js | 18 |
| spec/MediaQueryDispatch.js | 16 |
| functional-test/js/verbose.js | 14 |
| spec/QueryHandler.js | 13 |
| src/MediaQuery.js | 11 |
| spec/MediaQuery.js | 11 |
| functional-test/js/test-suite.js | 11 |
| src/QueryHandler.js | 8 |
| src/Util.js | 7 |
| Gruntfile.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 169}, "macro avg": {"f1-score": 0.544320666585729, "precision": 0.5879451874082747, "recall": 0.5202214149801072, "support": 2353}, "micro avg": {"f1-score": 0.953251168720782, "precision": 0.953251168720782, "recall": 0.953251168720782, "support": 2353}, "weighted avg": {"f1-score": 0.9463848476855637, "precision": 0.950758078994202, "recall": 0.953251168720782, "support": 2353}, "\u2205": {"f1-score": 0.9722362443210499, "precision": 0.9464373464373464, "recall": 0.9994810586403736, "support": 1927}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.7493670886075949, "precision": 0.9932885906040269, "recall": 0.6016260162601627, "support": 246}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6640471512770137, "precision": 1.0, "recall": 0.4970588235294118, "support": 340}, "macro avg": {"f1-score": 0.38563295966716743, "precision": 0.5879451874082747, "recall": 0.3259179527934986, "support": 3456}, "micro avg": {"f1-score": 0.7722499569633327, "precision": 0.953251168720782, "recall": 0.6490162037037037, "support": 3456}, "weighted avg": {"f1-score": 0.6958381422654744, "precision": 0.881439907341753, "recall": 0.6490162037037037, "support": 3456}, "\u2205": {"f1-score": 0.9441176470588236, "precision": 0.9464373464373464, "recall": 0.9418092909535453, "support": 2045}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 138}, "\u2423": {"f1-score": 0.31999999999999995, "precision": 0.9932885906040269, "recall": 0.19072164948453607, "support": 776}},
  "ppcr": 0.6808449074074074
}
```
</details>
