# Test report for javascript / file:///tmp/top-repos-quality-repos-ao_93aiy/movies.git HEAD 9273f225debd05495b6946ce6545211ac0497bd4

### Classification report

PPCR: 0.051

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.900| 1.000| 0.947| 9| 10| 0.900 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 87| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 7| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 62| 0.000 |
| `macro avg` | 0.167| 0.167| 0.150| 0.167| 0.158| 9| 178| 0.051 |
| `weighted avg` | 1.000| 1.000| 0.051| 1.000| 0.053| 9| 178| 0.051 |
| `micro avg` | 1.000| 1.000| 0.051| 1.000| 0.096| 9| 178| 0.051 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|87 |0 |0 |0 |0 |0 |0 |
|62 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |9 |0 |0 |0 |
|7 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9}, "macro avg": {"f1-score": 0.16666666666666666, "precision": 0.16666666666666666, "recall": 0.16666666666666666, "support": 9}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9473684210526316, "precision": 1.0, "recall": 0.9, "support": 10}, "macro avg": {"f1-score": 0.15789473684210528, "precision": 0.16666666666666666, "recall": 0.15, "support": 178}, "micro avg": {"f1-score": 0.0962566844919786, "precision": 1.0, "recall": 0.05056179775280899, "support": 178}, "weighted avg": {"f1-score": 0.05322294500295684, "precision": 0.056179775280898875, "recall": 0.05056179775280899, "support": 178}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 87}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}},
  "ppcr": 0.05056179775280899
}
```
</details>
