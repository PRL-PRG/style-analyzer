# Test report for javascript / file:///tmp/top-repos-quality-repos-e6zkkr2i/src_test.git HEAD 75a696370635c28263aface6ddfc4c3f4cbb7636

### Classification report

PPCR: 0.121

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.125| 1.000| 0.222| 23| 184| 0.125 |
| `␣` | 1.000| 1.000| 0.225| 1.000| 0.367| 20| 89| 0.225 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 30| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 8| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 32| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `macro avg` | 0.333| 0.333| 0.058| 0.333| 0.098| 43| 355| 0.121 |
| `micro avg` | 1.000| 1.000| 0.121| 1.000| 0.216| 43| 355| 0.121 |
| `weighted avg` | 1.000| 1.000| 0.121| 1.000| 0.207| 43| 355| 0.121 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|161 |23 |0 |0 |0 |0 |0 |
|69 |0 |20 |0 |0 |0 |0 |
|32 |0 |0 |0 |0 |0 |0 |
|30 |0 |0 |0 |0 |0 |0 |
|8 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 43}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 43}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 43}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 23}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 20}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "macro avg": {"f1-score": 0.09819911654774038, "precision": 0.3333333333333333, "recall": 0.05828651685393258, "support": 355}, "micro avg": {"f1-score": 0.21608040201005024, "precision": 1.0, "recall": 0.12112676056338029, "support": 355}, "weighted avg": {"f1-score": 0.2071815192890267, "precision": 0.7690140845070422, "recall": 0.12112676056338029, "support": 355}, "\u2205": {"f1-score": 0.2222222222222222, "precision": 1.0, "recall": 0.125, "support": 184}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.36697247706422015, "precision": 1.0, "recall": 0.2247191011235955, "support": 89}},
  "ppcr": 0.12112676056338029
}
```
</details>
