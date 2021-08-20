# Train report for javascript / file:///tmp/top-repos-quality-repos-ao_93aiy/movies.git HEAD 9273f225debd05495b6946ce6545211ac0497bd4

### Classification report

PPCR: 0.096

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.890| 1.000| 0.942| 251| 282| 0.890 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1369| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 556| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 200| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 106| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 95| 0.000 |
| `macro avg` | 0.167| 0.167| 0.148| 0.167| 0.157| 251| 2608| 0.096 |
| `weighted avg` | 1.000| 1.000| 0.096| 1.000| 0.102| 251| 2608| 0.096 |
| `micro avg` | 1.000| 1.000| 0.096| 1.000| 0.176| 251| 2608| 0.096 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1369 |0 |0 |0 |0 |0 |0 |
|556 |0 |0 |0 |0 |0 |0 |
|31 |0 |0 |251 |0 |0 |0 |
|200 |0 |0 |0 |0 |0 |0 |
|106 |0 |0 |0 |0 |0 |0 |
|95 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 251}, "macro avg": {"f1-score": 0.16666666666666666, "precision": 0.16666666666666666, "recall": 0.16666666666666666, "support": 251}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 251}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 251}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9418386491557224, "precision": 1.0, "recall": 0.8900709219858156, "support": 282}, "macro avg": {"f1-score": 0.1569731081926204, "precision": 0.16666666666666666, "recall": 0.1483451536643026, "support": 2608}, "micro avg": {"f1-score": 0.17558586918502972, "precision": 1.0, "recall": 0.09624233128834356, "support": 2608}, "weighted avg": {"f1-score": 0.10183991528447611, "precision": 0.10812883435582822, "recall": 0.09624233128834356, "support": 2608}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1369}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 200}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 556}},
  "ppcr": 0.09624233128834356
}
```
</details>
