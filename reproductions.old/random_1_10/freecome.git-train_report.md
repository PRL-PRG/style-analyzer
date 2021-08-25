# Train report for javascript / file:///tmp/top-repos-quality-repos-2a0so8st/freecome.git HEAD b7866f447c40a5687baefc13ba33244f97d62392

### Classification report

PPCR: 0.648

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 1.000| 0.905| 0.974| 0.926| 1339| 1480| 0.905 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 72| 371| 0.194 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 328| 0.000 |
| `weighted avg` | 0.901| 0.949| 0.615| 0.924| 0.629| 1411| 2179| 0.648 |
| `micro avg` | 0.949| 0.949| 0.615| 0.949| 0.746| 1411| 2179| 0.648 |
| `macro avg` | 0.316| 0.333| 0.302| 0.325| 0.309| 1411| 2179| 0.648 |

### Confusion matrix

|refusal|  ∅| "| ␣| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|141 |1339 |0 |0 |
|328 |0 |0 |0 |
|299 |72 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/js/app.js | 19 |
| frontend/webpack.config.js | 10 |
| frontend/src/js/reducers/sampleReducer.js | 6 |
| frontend/src/js/components/Layout.js | 5 |
| frontend/src/js/components/Dashboard.js | 5 |
| frontend/src/js/store.js | 5 |
| frontend/src/js/components/HelloWorld.js | 4 |
| frontend/src/js/components/Login.js | 4 |
| frontend/src/js/components/Join.js | 4 |
| frontend/src/js/components/Income.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3246060606060606, "precision": 0.3163241200094496, "recall": 0.3333333333333333, "support": 1411}, "micro avg": {"f1-score": 0.9489723600283487, "precision": 0.9489723600283487, "recall": 0.9489723600283487, "support": 1411}, "weighted avg": {"f1-score": 0.9241265382385154, "precision": 0.9005485400977739, "recall": 0.9489723600283487, "support": 1411}, "\u2205": {"f1-score": 0.9738181818181818, "precision": 0.9489723600283487, "recall": 1.0, "support": 1339}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 328}, "macro avg": {"f1-score": 0.30877435720050733, "precision": 0.3163241200094496, "recall": 0.3015765765765766, "support": 2179}, "micro avg": {"f1-score": 0.7459610027855154, "precision": 0.9489723600283487, "recall": 0.614502065167508, "support": 2179}, "weighted avg": {"f1-score": 0.6291684928729933, "precision": 0.6445521307214117, "recall": 0.614502065167508, "support": 2179}, "\u2205": {"f1-score": 0.9263230716015219, "precision": 0.9489723600283487, "recall": 0.9047297297297298, "support": 1480}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 371}},
  "ppcr": 0.6475447452960074
}
```
</details>
