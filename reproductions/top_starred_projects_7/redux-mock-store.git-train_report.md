# Train report for javascript / file:///tmp/top-repos-quality-repos-kr10su2p/redux-mock-store.git HEAD b943c3ba0abf6d3e7f9918bd470525e85d166cff

### Classification report

PPCR: 0.221

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 1.000| 0.390| 0.984| 0.556| 274| 702| 0.390 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 440| 0.020 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 138| 0.000 |
| `macro avg` | 0.323| 0.333| 0.130| 0.328| 0.185| 283| 1280| 0.221 |
| `weighted avg` | 0.937| 0.968| 0.214| 0.953| 0.305| 283| 1280| 0.221 |
| `micro avg` | 0.968| 0.968| 0.214| 0.968| 0.351| 283| 1280| 0.221 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|428 |274 |0 |0 |
|431 |9 |0 |0 |
|138 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/index.js | 7 |
| test/index.js | 1 |
| rollup.config.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.32794733692399763, "precision": 0.32273262661955243, "recall": 0.3333333333333333, "support": 283}, "micro avg": {"f1-score": 0.9681978798586572, "precision": 0.9681978798586572, "recall": 0.9681978798586572, "support": 283}, "weighted avg": {"f1-score": 0.9525537489453217, "precision": 0.9374071345627988, "recall": 0.9681978798586572, "support": 283}, "\u2205": {"f1-score": 0.9838420107719928, "precision": 0.9681978798586572, "recall": 1.0, "support": 274}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 138}, "macro avg": {"f1-score": 0.18544839255499157, "precision": 0.32273262661955243, "recall": 0.13010446343779677, "support": 1280}, "micro avg": {"f1-score": 0.35060780550223924, "precision": 0.9681978798586572, "recall": 0.2140625, "support": 1280}, "weighted avg": {"f1-score": 0.30512055837563457, "precision": 0.5309960247349823, "recall": 0.2140625, "support": 1280}, "\u2205": {"f1-score": 0.5563451776649747, "precision": 0.9681978798586572, "recall": 0.3903133903133903, "support": 702}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 440}},
  "ppcr": 0.22109375
}
```
</details>
