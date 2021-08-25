# Test report for javascript / file:///tmp/top-repos-quality-repos-7_kmrrgz/mac-torrents.git HEAD b533ade849aefb261185fab7c941c843a6e9c046

### Classification report

PPCR: 0.227

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.895| 1.000| 0.367| 0.944| 0.520| 51| 139| 0.367 |
| `'` | 1.000| 0.933| 0.875| 0.966| 0.933| 45| 48| 0.938 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 195| 0.010 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 36| 0.028 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 19| 0.000 |
| `weighted avg` | 0.915| 0.939| 0.213| 0.925| 0.268| 99| 437| 0.227 |
| `micro avg` | 0.939| 0.939| 0.213| 0.939| 0.347| 99| 437| 0.227 |
| `macro avg` | 0.379| 0.387| 0.248| 0.382| 0.291| 99| 437| 0.227 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|193 |0 |2 |0 |0 |0 |
|88 |0 |51 |0 |0 |0 |
|3 |0 |3 |42 |0 |0 |
|35 |0 |1 |0 |0 |0 |
|19 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9655172413793104, "precision": 1.0, "recall": 0.9333333333333333, "support": 45}, "macro avg": {"f1-score": 0.38199233716475095, "precision": 0.37894736842105264, "recall": 0.38666666666666666, "support": 99}, "micro avg": {"f1-score": 0.9393939393939394, "precision": 0.9393939393939394, "recall": 0.9393939393939394, "support": 99}, "weighted avg": {"f1-score": 0.9254034598862185, "precision": 0.9154704944178629, "recall": 0.9393939393939394, "support": 99}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9444444444444444, "precision": 0.8947368421052632, "recall": 1.0, "support": 51}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9333333333333333, "precision": 1.0, "recall": 0.875, "support": 48}, "macro avg": {"f1-score": 0.2907482993197279, "precision": 0.37894736842105264, "recall": 0.24838129496402878, "support": 437}, "micro avg": {"f1-score": 0.34701492537313433, "precision": 0.9393939393939394, "recall": 0.2128146453089245, "support": 437}, "weighted avg": {"f1-score": 0.2680474478120768, "precision": 0.3944357461158617, "recall": 0.2128146453089245, "support": 437}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 195}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u2423": {"f1-score": 0.5204081632653061, "precision": 0.8947368421052632, "recall": 0.3669064748201439, "support": 139}},
  "ppcr": 0.22654462242562928
}
```
</details>
