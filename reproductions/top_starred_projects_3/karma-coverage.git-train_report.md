# Train report for javascript / file:///tmp/top-repos-quality-repos-tngnhgr_/karma-coverage.git HEAD 6b1419fb2b14c24f367ef9718975870531b129d2

### Classification report

PPCR: 0.538

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.999| 0.988| 0.599| 0.994| 0.749| 857| 1414| 0.606 |
| `␣` | 0.970| 0.999| 0.711| 0.984| 0.821| 678| 952| 0.712 |
| `'` | 1.000| 1.000| 0.680| 1.000| 0.809| 155| 228| 0.680 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 106| 0.057 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 193| 0.016 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 132| 0.015 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 136| 0.000 |
| `weighted avg` | 0.981| 0.987| 0.531| 0.984| 0.641| 1701| 3161| 0.538 |
| `micro avg` | 0.987| 0.987| 0.531| 0.987| 0.691| 1701| 3161| 0.538 |
| `macro avg` | 0.424| 0.427| 0.284| 0.425| 0.340| 1701| 3161| 0.538 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|557 |847 |10 |0 |0 |0 |0 |0 |
|274 |1 |677 |0 |0 |0 |0 |0 |
|73 |0 |0 |155 |0 |0 |0 |0 |
|190 |0 |3 |0 |0 |0 |0 |0 |
|136 |0 |0 |0 |0 |0 |0 |0 |
|130 |0 |2 |0 |0 |0 |0 |0 |
|100 |0 |6 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/coverage-map.js | 4 |
| lib/reporter.js | 4 |
| lib/report-creator.js | 3 |
| lib/preprocessor.js | 3 |
| lib/in-memory-report.js | 3 |
| gruntfile.js | 2 |
| lib/source-map-store.js | 2 |
| lib/index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 155}, "macro avg": {"f1-score": 0.4253657164291073, "precision": 0.42410497069022773, "recall": 0.4266937803301533, "support": 1701}, "micro avg": {"f1-score": 0.9870664315108759, "precision": 0.9870664315108759, "recall": 0.9870664315108759, "support": 1701}, "weighted avg": {"f1-score": 0.9839099655866348, "precision": 0.98094715225759, "recall": 0.9870664315108759, "support": 1701}, "\u2205": {"f1-score": 0.9935483870967742, "precision": 0.9988207547169812, "recall": 0.9883313885647608, "support": 857}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9840116279069768, "precision": 0.9699140401146131, "recall": 0.9985250737463127, "support": 678}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8093994778067886, "precision": 1.0, "recall": 0.6798245614035088, "support": 228}, "macro avg": {"f1-score": 0.33984290311291304, "precision": 0.42410497069022773, "recall": 0.2842812737393029, "support": 3161}, "micro avg": {"f1-score": 0.6906622788975729, "precision": 0.9870664315108759, "recall": 0.5311610249920911, "support": 3161}, "weighted avg": {"f1-score": 0.6405242879888521, "precision": 0.8110378719895359, "recall": 0.5311610249920911, "support": 3161}, "\u2205": {"f1-score": 0.748894783377542, "precision": 0.9988207547169812, "recall": 0.599009900990099, "support": 1414}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u2423": {"f1-score": 0.8206060606060606, "precision": 0.9699140401146131, "recall": 0.7111344537815126, "support": 952}},
  "ppcr": 0.5381208478329642
}
```
</details>
