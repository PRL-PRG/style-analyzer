# Test report for javascript / file:///tmp/top-repos-quality-repos-0oxqvjh0/homeautomation.git HEAD f7477e56b88cc724c02bb13c50b41099b44e167a

### Classification report

PPCR: 0.422

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.857| 1.000| 0.659| 0.923| 0.745| 60| 91| 0.659 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 36| 0.194 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 16| 0.125 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 20| 0.050 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `macro avg` | 0.171| 0.200| 0.132| 0.185| 0.149| 70| 166| 0.422 |
| `micro avg` | 0.857| 0.857| 0.361| 0.857| 0.508| 70| 166| 0.422 |
| `weighted avg` | 0.735| 0.857| 0.361| 0.791| 0.409| 70| 166| 0.422 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|31 |60 |0 |0 |0 |0 |
|29 |7 |0 |0 |0 |0 |
|19 |1 |0 |0 |0 |0 |
|14 |2 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "macro avg": {"f1-score": 0.1846153846153846, "precision": 0.17142857142857143, "recall": 0.2, "support": 70}, "micro avg": {"f1-score": 0.8571428571428571, "precision": 0.8571428571428571, "recall": 0.8571428571428571, "support": 70}, "weighted avg": {"f1-score": 0.7912087912087912, "precision": 0.7346938775510203, "recall": 0.8571428571428571, "support": 70}, "\u2205": {"f1-score": 0.923076923076923, "precision": 0.8571428571428571, "recall": 1.0, "support": 60}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "macro avg": {"f1-score": 0.14906832298136646, "precision": 0.17142857142857143, "recall": 0.13186813186813187, "support": 166}, "micro avg": {"f1-score": 0.5084745762711865, "precision": 0.8571428571428571, "recall": 0.3614457831325301, "support": 166}, "weighted avg": {"f1-score": 0.4085908852802514, "precision": 0.46987951807228917, "recall": 0.3614457831325301, "support": 166}, "\u2205": {"f1-score": 0.7453416149068323, "precision": 0.8571428571428571, "recall": 0.6593406593406593, "support": 91}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}},
  "ppcr": 0.42168674698795183
}
```
</details>
