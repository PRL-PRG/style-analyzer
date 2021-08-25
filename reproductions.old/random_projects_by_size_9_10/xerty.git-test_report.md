# Test report for javascript / file:///tmp/top-repos-quality-repos-s6iy5kje/xerty.git HEAD e6996544af10ffc291c08b54df13b43fc299d6b3

### Classification report

PPCR: 0.755

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.883| 1.000| 0.910| 0.938| 0.896| 242| 266| 0.910 |
| `␣` | 1.000| 0.667| 0.374| 0.800| 0.544| 51| 91| 0.560 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 17| 0.882 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 18| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 16| 0.000 |
| `micro avg` | 0.896| 0.896| 0.676| 0.896| 0.771| 308| 408| 0.755 |
| `weighted avg` | 0.860| 0.896| 0.676| 0.869| 0.706| 308| 408| 0.755 |
| `macro avg` | 0.377| 0.333| 0.257| 0.348| 0.288| 308| 408| 0.755 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|24 |242 |0 |0 |0 |0 |
|40 |17 |34 |0 |0 |0 |
|18 |0 |0 |0 |0 |0 |
|2 |15 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.34759689922480624, "precision": 0.37664233576642336, "recall": 0.3333333333333333, "support": 308}, "micro avg": {"f1-score": 0.8961038961038962, "precision": 0.8961038961038961, "recall": 0.8961038961038961, "support": 308}, "weighted avg": {"f1-score": 0.8694553508506997, "precision": 0.8595364489525074, "recall": 0.8961038961038961, "support": 308}, "\u2205": {"f1-score": 0.937984496124031, "precision": 0.8832116788321168, "recall": 1.0, "support": 242}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 51}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "macro avg": {"f1-score": 0.2880592592592593, "precision": 0.37664233576642336, "recall": 0.25668016194331983, "support": 408}, "micro avg": {"f1-score": 0.770949720670391, "precision": 0.8961038961038961, "recall": 0.6764705882352942, "support": 408}, "weighted avg": {"f1-score": 0.705683369644154, "precision": 0.7988585945327037, "recall": 0.6764705882352942, "support": 408}, "\u2205": {"f1-score": 0.8962962962962964, "precision": 0.8832116788321168, "recall": 0.9097744360902256, "support": 266}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.544, "precision": 1.0, "recall": 0.37362637362637363, "support": 91}},
  "ppcr": 0.7549019607843137
}
```
</details>
