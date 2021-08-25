# Test report for javascript / file:///tmp/top-repos-quality-repos-_1nu3kix/fantasticfour.git HEAD 90ccc5247582b0a7a145ffacddf0c289847e2268

### Classification report

PPCR: 0.996

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.890| 0.998| 0.993| 0.941| 0.939| 2725| 2739| 0.995 |
| `⏎` | 0.965| 0.677| 0.677| 0.796| 0.796| 371| 371| 1.000 |
| `␣` | 1.000| 0.331| 0.331| 0.497| 0.497| 366| 366| 1.000 |
| `"` | 0.667| 1.000| 1.000| 0.800| 0.800| 52| 52| 1.000 |
| `macro avg` | 0.881| 0.751| 0.750| 0.758| 0.758| 3514| 3528| 0.996 |
| `micro avg` | 0.894| 0.894| 0.891| 0.894| 0.893| 3514| 3528| 0.996 |
| `weighted avg` | 0.906| 0.894| 0.891| 0.877| 0.876| 3514| 3528| 0.996 |

### Confusion matrix

|refusal|  ∅| ⏎| ␣| "| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|14 |2719 |3 |0 |3 |
|0 |118 |251 |0 |2 |
|0 |218 |6 |121 |21 |
|0 |0 |0 |0 |52 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8, "precision": 0.6666666666666666, "recall": 1.0, "support": 52}, "macro avg": {"f1-score": 0.7583282416851487, "precision": 0.8805169121658483, "recall": 0.751237280815725, "support": 3514}, "micro avg": {"f1-score": 0.8944223107569721, "precision": 0.8944223107569721, "recall": 0.8944223107569721, "support": 3514}, "weighted avg": {"f1-score": 0.8771739868993088, "precision": 0.9061237785977126, "recall": 0.8944223107569721, "support": 3514}, "\u2205": {"f1-score": 0.9408304498269896, "precision": 0.8900163666121113, "recall": 0.9977981651376147, "support": 2725}, "\u23ce": {"f1-score": 0.7955625990491283, "precision": 0.9653846153846154, "recall": 0.6765498652291105, "support": 371}, "\u2423": {"f1-score": 0.49691991786447637, "precision": 1.0, "recall": 0.33060109289617484, "support": 366}},
  "cl_report_full": {"\"": {"f1-score": 0.8, "precision": 0.6666666666666666, "recall": 1.0, "support": 52}, "macro avg": {"f1-score": 0.7577599112442797, "precision": 0.8805169121658483, "recall": 0.7499622557781267, "support": 3528}, "micro avg": {"f1-score": 0.8926441351888669, "precision": 0.8944223107569721, "recall": 0.8908730158730159, "support": 3528}, "weighted avg": {"f1-score": 0.8756616745894528, "precision": 0.9060598602961825, "recall": 0.8908730158730159, "support": 3528}, "\u2205": {"f1-score": 0.938557128063514, "precision": 0.8900163666121113, "recall": 0.9926980649872216, "support": 2739}, "\u23ce": {"f1-score": 0.7955625990491283, "precision": 0.9653846153846154, "recall": 0.6765498652291105, "support": 371}, "\u2423": {"f1-score": 0.49691991786447637, "precision": 1.0, "recall": 0.33060109289617484, "support": 366}},
  "ppcr": 0.996031746031746
}
```
</details>
