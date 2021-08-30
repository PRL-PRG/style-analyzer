# Train report for javascript / file:///tmp/top-repos-quality-repos-kh7ww7en/dinero.js.git HEAD d54bf9492e7f7a5bea926d91211a590d2c95b510

### Classification report

PPCR: 0.073

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 281| 562| 0.500 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1585| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1016| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 298| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 183| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 181| 0.000 |
| `micro avg` | 1.000| 1.000| 0.073| 1.000| 0.137| 281| 3825| 0.073 |
| `macro avg` | 0.167| 0.167| 0.083| 0.167| 0.111| 281| 3825| 0.073 |
| `weighted avg` | 1.000| 1.000| 0.073| 1.000| 0.098| 281| 3825| 0.073 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1585 |0 |0 |0 |0 |0 |0 |
|1016 |0 |0 |0 |0 |0 |0 |
|281 |0 |0 |281 |0 |0 |0 |
|298 |0 |0 |0 |0 |0 |0 |
|183 |0 |0 |0 |0 |0 |0 |
|181 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 281}, "macro avg": {"f1-score": 0.16666666666666666, "precision": 0.16666666666666666, "recall": 0.16666666666666666, "support": 281}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 281}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 281}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 562}, "macro avg": {"f1-score": 0.1111111111111111, "precision": 0.16666666666666666, "recall": 0.08333333333333333, "support": 3825}, "micro avg": {"f1-score": 0.13687286897223574, "precision": 1.0, "recall": 0.0734640522875817, "support": 3825}, "weighted avg": {"f1-score": 0.09795206971677559, "precision": 0.1469281045751634, "recall": 0.0734640522875817, "support": 3825}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1585}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 298}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 183}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 181}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1016}},
  "ppcr": 0.0734640522875817
}
```
</details>
