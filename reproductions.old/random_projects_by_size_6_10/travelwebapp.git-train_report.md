# Train report for javascript / file:///tmp/top-repos-quality-repos-1j3933we/travelwebapp.git HEAD 42ca0cb12e4ba5118f738b084a4ad75dd03c7fd1

### Classification report

PPCR: 0.503

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.984| 0.713| 0.979| 0.824| 8675| 11968| 0.725 |
| `␣` | 0.946| 0.966| 0.426| 0.956| 0.588| 2573| 5829| 0.441 |
| `⏎` | 0.997| 0.915| 0.273| 0.955| 0.429| 414| 1386| 0.299 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 53| 755| 0.070 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 827| 0.031 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 89| 0.101 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 114| 0.061 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 290| 0.014 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1663| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 473| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.961| 0.969| 0.487| 0.965| 0.593| 11761| 23400| 0.503 |
| `micro avg` | 0.969| 0.969| 0.487| 0.969| 0.648| 11761| 23400| 0.503 |
| `macro avg` | 0.243| 0.239| 0.118| 0.241| 0.153| 11761| 23400| 0.503 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎⏎⇥⁺| ⏎⏎⇥⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3293 |8535 |140 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3256 |88 |2485 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1663 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|972 |35 |0 |0 |379 |0 |0 |0 |0 |0 |0 |0 |
|801 |26 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|702 |53 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|473 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|286 |3 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|107 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|80 |8 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| kompres2015/static/js/app.js | 98 |
| kompres2015/static/js/json3.js | 83 |
| kompres2015/static/js/AssimpJSONLoader.js | 58 |
| kompres2015/static/js/controllers.js | 30 |
| kompres2015/static/js/services.js | 21 |
| kompres2015/static/js/controllers/main.js | 16 |
| kompres2015/static/js/services/djangoAuth.js | 12 |
| kompres2015/static/js/livereload.js | 9 |
| kompres2015/static/js/tjsModelViewer.js | 9 |
| kompres2015/static/js/services/validate.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24082991439745072, "precision": 0.2432121743441613, "recall": 0.23875994060442643, "support": 11761}, "micro avg": {"f1-score": 0.9692203043958847, "precision": 0.9692203043958847, "recall": 0.9692203043958847, "support": 11761}, "weighted avg": {"f1-score": 0.9651154344557695, "precision": 0.9612079397325671, "recall": 0.9692203043958847, "support": 11761}, "\u2205": {"f1-score": 0.9793459552495697, "precision": 0.9748715019988577, "recall": 0.9838616714697407, "support": 8675}, "\u23ce": {"f1-score": 0.9546599496221662, "precision": 0.9973684210526316, "recall": 0.9154589371980676, "support": 414}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.9559530678976726, "precision": 0.9463061690784463, "recall": 0.965798678585309, "support": 2573}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 473}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1663}, "macro avg": {"f1-score": 0.1533965721926526, "precision": 0.2432121743441613, "recall": 0.11774310031806302, "support": 23400}, "micro avg": {"f1-score": 0.6483888399078523, "precision": 0.9692203043958847, "recall": 0.4871367521367521, "support": 23400}, "weighted avg": {"f1-score": 0.5931451852169634, "precision": 0.7934031379085273, "recall": 0.4871367521367521, "support": 23400}, "\u2205": {"f1-score": 0.8237224340105197, "precision": 0.9748715019988577, "recall": 0.7131517379679144, "support": 11968}, "\u23ce": {"f1-score": 0.42921857304643257, "precision": 0.9973684210526316, "recall": 0.27344877344877344, "support": 1386}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 290}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 827}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 89}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 755}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u2423": {"f1-score": 0.5878178592548787, "precision": 0.9463061690784463, "recall": 0.42631669240006864, "support": 5829}},
  "ppcr": 0.5026068376068376
}
```
</details>
