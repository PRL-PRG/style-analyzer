# Test report for javascript / file:///tmp/top-repos-quality-repos-t3cguefe/iotc.git HEAD 4b2b745cb343fe5969218c79f080d98b4bbc02f9

### Classification report

PPCR: 0.440

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.939| 1.000| 0.564| 0.969| 0.705| 31| 55| 0.564 |
| `␣` | 1.000| 0.667| 0.167| 0.800| 0.286| 6| 24| 0.250 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 5| 0.000 |
| `macro avg` | 0.646| 0.556| 0.243| 0.590| 0.330| 37| 84| 0.440 |
| `weighted avg` | 0.949| 0.946| 0.417| 0.941| 0.543| 37| 84| 0.440 |
| `micro avg` | 0.946| 0.946| 0.417| 0.946| 0.579| 37| 84| 0.440 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|24 |31 |0 |0 |
|18 |2 |4 |0 |
|5 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.5895833333333333, "precision": 0.6464646464646465, "recall": 0.5555555555555555, "support": 37}, "micro avg": {"f1-score": 0.9459459459459459, "precision": 0.9459459459459459, "recall": 0.9459459459459459, "support": 37}, "weighted avg": {"f1-score": 0.9413851351351351, "precision": 0.9492219492219494, "recall": 0.9459459459459459, "support": 37}, "\u2205": {"f1-score": 0.96875, "precision": 0.9393939393939394, "recall": 1.0, "support": 31}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 6}},
  "cl_report_full": {"macro avg": {"f1-score": 0.3300865800865801, "precision": 0.6464646464646465, "recall": 0.24343434343434342, "support": 84}, "micro avg": {"f1-score": 0.578512396694215, "precision": 0.9459459459459459, "recall": 0.4166666666666667, "support": 84}, "weighted avg": {"f1-score": 0.5429421768707483, "precision": 0.9007936507936508, "recall": 0.4166666666666667, "support": 84}, "\u2205": {"f1-score": 0.7045454545454546, "precision": 0.9393939393939394, "recall": 0.5636363636363636, "support": 55}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.2857142857142857, "precision": 1.0, "recall": 0.16666666666666666, "support": 24}},
  "ppcr": 0.44047619047619047
}
```
</details>
