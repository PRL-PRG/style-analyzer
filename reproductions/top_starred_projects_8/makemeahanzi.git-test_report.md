# Test report for javascript / file:///tmp/top-repos-quality-repos-4l3wri28/makemeahanzi.git HEAD 618dbab8a8ddefb958763c8b4afbaa741a4460de

### Classification report

PPCR: 0.801

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.902| 0.952| 0.880| 0.926| 0.891| 270| 292| 0.925 |
| `␣` | 0.182| 0.067| 0.023| 0.098| 0.041| 30| 86| 0.349 |
| `⏎` | 0.667| 0.824| 0.778| 0.737| 0.718| 17| 18| 0.944 |
| `weighted avg` | 0.821| 0.861| 0.689| 0.838| 0.698| 317| 396| 0.801 |
| `macro avg` | 0.583| 0.614| 0.560| 0.587| 0.550| 317| 396| 0.801 |
| `micro avg` | 0.861| 0.861| 0.689| 0.861| 0.766| 317| 396| 0.801 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|22 |257 |9 |4 |
|56 |25 |2 |3 |
|1 |3 |0 |14 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.58684306899968, "precision": 0.5834130781499202, "recall": 0.6140159767610748, "support": 317}, "micro avg": {"f1-score": 0.861198738170347, "precision": 0.861198738170347, "recall": 0.861198738170347, "support": 317}, "weighted avg": {"f1-score": 0.8375621423085817, "precision": 0.8210143943167354, "recall": 0.861198738170347, "support": 317}, "\u2205": {"f1-score": 0.9261261261261261, "precision": 0.9017543859649123, "recall": 0.9518518518518518, "support": 270}, "\u23ce": {"f1-score": 0.7368421052631577, "precision": 0.6666666666666666, "recall": 0.8235294117647058, "support": 17}, "\u2423": {"f1-score": 0.0975609756097561, "precision": 0.18181818181818182, "recall": 0.06666666666666667, "support": 30}},
  "cl_report_full": {"macro avg": {"f1-score": 0.5500001298032351, "precision": 0.5834130781499202, "recall": 0.5603901926775453, "support": 396}, "micro avg": {"f1-score": 0.7657784011220196, "precision": 0.861198738170347, "recall": 0.6893939393939394, "support": 396}, "weighted avg": {"f1-score": 0.6984528273456073, "precision": 0.7347187988336313, "recall": 0.6893939393939394, "support": 396}, "\u2205": {"f1-score": 0.8908145580589255, "precision": 0.9017543859649123, "recall": 0.8801369863013698, "support": 292}, "\u23ce": {"f1-score": 0.717948717948718, "precision": 0.6666666666666666, "recall": 0.7777777777777778, "support": 18}, "\u2423": {"f1-score": 0.04123711340206186, "precision": 0.18181818181818182, "recall": 0.023255813953488372, "support": 86}},
  "ppcr": 0.8005050505050505
}
```
</details>
