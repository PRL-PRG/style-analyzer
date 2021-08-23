# Train report for javascript / file:///tmp/top-repos-quality-repos-j5lewy8n/soundcloud-uploadr.git HEAD 528833f9ee4004b19547de73e704d6315d2ab43e

### Classification report

PPCR: 0.369

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 1.000| 0.656| 0.989| 0.785| 1626| 2480| 0.656 |
| `⏎` | 0.965| 0.995| 0.475| 0.980| 0.637| 222| 465| 0.477 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 1345| 0.025 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 115| 0.043 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 100| 0.040 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 275| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 235| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 103| 0.000 |
| `weighted avg` | 0.956| 0.977| 0.361| 0.966| 0.438| 1890| 5118| 0.369 |
| `micro avg` | 0.977| 0.977| 0.361| 0.977| 0.527| 1890| 5118| 0.369 |
| `macro avg` | 0.243| 0.249| 0.141| 0.246| 0.178| 1890| 5118| 0.369 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|854 |1626 |0 |0 |0 |0 |0 |0 |0 |
|1312 |33 |0 |0 |0 |0 |0 |0 |0 |
|243 |1 |0 |221 |0 |0 |0 |0 |0 |
|275 |0 |0 |0 |0 |0 |0 |0 |0 |
|235 |0 |0 |0 |0 |0 |0 |0 |0 |
|103 |0 |0 |0 |0 |0 |0 |0 |0 |
|96 |1 |0 |3 |0 |0 |0 |0 |0 |
|110 |0 |0 |5 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/js/uploadr.js | 33 |
| static/js/FileSaver.js | 5 |
| static/js/StreamSaver.js | 3 |
| static/js/downloadr.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2461745423245642, "precision": 0.2429992323244008, "recall": 0.24943693693693694, "support": 1890}, "micro avg": {"f1-score": 0.9772486772486773, "precision": 0.9772486772486773, "recall": 0.9772486772486773, "support": 1890}, "weighted avg": {"f1-score": 0.9662731137127688, "precision": 0.9555460576774091, "recall": 0.9772486772486773, "support": 1890}, "\u2205": {"f1-score": 0.9893519926985093, "precision": 0.9789283564118001, "recall": 1.0, "support": 1626}, "\u23ce": {"f1-score": 0.9800443458980044, "precision": 0.9650655021834061, "recall": 0.9954954954954955, "support": 222}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 275}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 235}, "macro avg": {"f1-score": 0.17777564552687786, "precision": 0.2429992323244008, "recall": 0.14136424731182795, "support": 5118}, "micro avg": {"f1-score": 0.5271118721461187, "precision": 0.9772486772486773, "recall": 0.36088315748339195, "support": 5118}, "weighted avg": {"f1-score": 0.4384017735431595, "precision": 0.5620355182525495, "recall": 0.36088315748339195, "support": 5118}, "\u2205": {"f1-score": 0.7853175561458585, "precision": 0.9789283564118001, "recall": 0.6556451612903226, "support": 2480}, "\u23ce": {"f1-score": 0.6368876080691643, "precision": 0.9650655021834061, "recall": 0.4752688172043011, "support": 465}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1345}},
  "ppcr": 0.369284876905041
}
```
</details>
