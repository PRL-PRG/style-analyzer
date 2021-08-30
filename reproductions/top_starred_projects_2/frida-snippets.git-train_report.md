# Train report for javascript / file:///tmp/top-repos-quality-repos-ztfk92sj/frida-snippets.git HEAD e627f950c1aa26856b64aaff530d86599793e9fc

### Classification report

PPCR: 0.403

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 1.000| 0.691| 0.997| 0.815| 1239| 1793| 0.691 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 882| 0.009 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 178| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 123| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 115| 0.000 |
| `micro avg` | 0.994| 0.994| 0.401| 0.994| 0.571| 1247| 3091| 0.403 |
| `macro avg` | 0.199| 0.200| 0.138| 0.199| 0.163| 1247| 3091| 0.403 |
| `weighted avg` | 0.987| 0.994| 0.401| 0.990| 0.473| 1247| 3091| 0.403 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|554 |1239 |0 |0 |0 |0 |
|874 |8 |0 |0 |0 |0 |
|178 |0 |0 |0 |0 |0 |
|123 |0 |0 |0 |0 |0 |
|115 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| scripts/trace_class.js | 7 |
| scripts/enumerateNativeMethods.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1993563958165728, "precision": 0.19871692060946272, "recall": 0.2, "support": 1247}, "micro avg": {"f1-score": 0.9935846030473136, "precision": 0.9935846030473136, "recall": 0.9935846030473136, "support": 1247}, "weighted avg": {"f1-score": 0.9903872270117631, "precision": 0.9872103634126876, "recall": 0.9935846030473136, "support": 1247}, "\u2205": {"f1-score": 0.996781979082864, "precision": 0.9935846030473136, "recall": 1.0, "support": 1239}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "macro avg": {"f1-score": 0.16302631578947366, "precision": 0.19871692060946272, "recall": 0.13820412716118238, "support": 3091}, "micro avg": {"f1-score": 0.5712309820193638, "precision": 0.9935846030473136, "recall": 0.40084115173083146, "support": 3091}, "weighted avg": {"f1-score": 0.472834332271961, "precision": 0.5763497875327833, "recall": 0.40084115173083146, "support": 3091}, "\u2205": {"f1-score": 0.8151315789473683, "precision": 0.9935846030473136, "recall": 0.6910206358059119, "support": 1793}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 178}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 882}},
  "ppcr": 0.4034293109026205
}
```
</details>
