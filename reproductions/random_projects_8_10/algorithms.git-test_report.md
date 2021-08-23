# Test report for javascript / file:///tmp/top-repos-quality-repos-ytvp6mi1/algorithms.git HEAD a6e5a59f0f51691108484cf61b1c1bf9a8ab7aaf

### Classification report

PPCR: 0.745

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.836| 0.944| 0.810| 0.887| 0.823| 54| 63| 0.857 |
| `␣` | 0.800| 0.545| 0.343| 0.649| 0.480| 22| 35| 0.629 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `macro avg` | 0.545| 0.497| 0.384| 0.512| 0.434| 76| 102| 0.745 |
| `micro avg` | 0.829| 0.829| 0.618| 0.829| 0.708| 76| 102| 0.745 |
| `weighted avg` | 0.826| 0.829| 0.618| 0.818| 0.673| 76| 102| 0.745 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|9 |51 |3 |0 |
|13 |10 |12 |0 |
|4 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.5118683901292597, "precision": 0.5453551912568306, "recall": 0.4966329966329966, "support": 76}, "micro avg": {"f1-score": 0.8289473684210527, "precision": 0.8289473684210527, "recall": 0.8289473684210527, "support": 76}, "weighted avg": {"f1-score": 0.817972663739254, "precision": 0.8256255392579811, "recall": 0.8289473684210527, "support": 76}, "\u2205": {"f1-score": 0.8869565217391304, "precision": 0.8360655737704918, "recall": 0.9444444444444444, "support": 54}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.6486486486486486, "precision": 0.8, "recall": 0.5454545454545454, "support": 22}},
  "cl_report_full": {"macro avg": {"f1-score": 0.43419354838709684, "precision": 0.5453551912568306, "recall": 0.3841269841269841, "support": 102}, "micro avg": {"f1-score": 0.7078651685393259, "precision": 0.8289473684210527, "recall": 0.6176470588235294, "support": 102}, "weighted avg": {"f1-score": 0.6727703984819735, "precision": 0.7909032465445196, "recall": 0.6176470588235294, "support": 102}, "\u2205": {"f1-score": 0.8225806451612904, "precision": 0.8360655737704918, "recall": 0.8095238095238095, "support": 63}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.48000000000000004, "precision": 0.8, "recall": 0.34285714285714286, "support": 35}},
  "ppcr": 0.7450980392156863
}
```
</details>
