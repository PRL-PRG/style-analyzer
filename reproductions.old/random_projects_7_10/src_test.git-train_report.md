# Train report for javascript / file:///tmp/top-repos-quality-repos-e6zkkr2i/src_test.git HEAD 75a696370635c28263aface6ddfc4c3f4cbb7636

### Classification report

PPCR: 0.052

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.101| 1.000| 0.184| 157| 1551| 0.101 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 900| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 197| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 178| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 116| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 103| 0.000 |
| `macro avg` | 0.167| 0.167| 0.017| 0.167| 0.031| 157| 3045| 0.052 |
| `micro avg` | 1.000| 1.000| 0.052| 1.000| 0.098| 157| 3045| 0.052 |
| `weighted avg` | 1.000| 1.000| 0.052| 1.000| 0.094| 157| 3045| 0.052 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1394 |157 |0 |0 |0 |0 |0 |
|900 |0 |0 |0 |0 |0 |0 |
|197 |0 |0 |0 |0 |0 |0 |
|178 |0 |0 |0 |0 |0 |0 |
|116 |0 |0 |0 |0 |0 |0 |
|103 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16666666666666666, "precision": 0.16666666666666666, "recall": 0.16666666666666666, "support": 157}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 157}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 157}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 157}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 178}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "macro avg": {"f1-score": 0.030640124902419987, "precision": 0.16666666666666666, "recall": 0.01687083601977219, "support": 3045}, "micro avg": {"f1-score": 0.09806371018113678, "precision": 1.0, "recall": 0.05155993431855501, "support": 3045}, "weighted avg": {"f1-score": 0.09364105167222345, "precision": 0.50935960591133, "recall": 0.05155993431855501, "support": 3045}, "\u2205": {"f1-score": 0.18384074941451992, "precision": 1.0, "recall": 0.10122501611863315, "support": 1551}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 197}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 900}},
  "ppcr": 0.05155993431855501
}
```
</details>
