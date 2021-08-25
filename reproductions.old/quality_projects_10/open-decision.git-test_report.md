# Test report for javascript / file:///tmp/top-repos-quality-repos-xlc6v3s5/open-decision.git HEAD d8ebd55ed79157c881455527501b1389d8f41f45

### Classification report

PPCR: 0.754

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.852| 0.980| 0.856| 0.911| 0.854| 2324| 2662| 0.873 |
| `␣` | 0.846| 0.594| 0.259| 0.698| 0.397| 424| 972| 0.436 |
| `⏎` | 1.000| 0.284| 0.236| 0.442| 0.381| 261| 314| 0.831 |
| `'` | 1.000| 0.721| 0.441| 0.838| 0.613| 136| 222| 0.613 |
| `macro avg` | 0.924| 0.645| 0.448| 0.722| 0.561| 3145| 4170| 0.754 |
| `micro avg` | 0.859| 0.859| 0.648| 0.859| 0.739| 3145| 4170| 0.754 |
| `weighted avg` | 0.870| 0.859| 0.648| 0.840| 0.699| 3145| 4170| 0.754 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|338 |2278 |46 |0 |0 |
|548 |172 |252 |0 |0 |
|86 |38 |0 |98 |0 |
|53 |187 |0 |0 |74 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8376068376068375, "precision": 1.0, "recall": 0.7205882352941176, "support": 136}, "macro avg": {"f1-score": 0.7222102751666256, "precision": 0.9243065922348366, "recall": 0.6446648256494227, "support": 3145}, "micro avg": {"f1-score": 0.859141494435612, "precision": 0.859141494435612, "recall": 0.859141494435612, "support": 3145}, "weighted avg": {"f1-score": 0.8404617622951903, "precision": 0.8695207224225971, "recall": 0.859141494435612, "support": 3145}, "\u2205": {"f1-score": 0.911382276455291, "precision": 0.851588785046729, "recall": 0.9802065404475043, "support": 2324}, "\u23ce": {"f1-score": 0.4417910447761194, "precision": 1.0, "recall": 0.2835249042145594, "support": 261}, "\u2423": {"f1-score": 0.6980609418282547, "precision": 0.8456375838926175, "recall": 0.5943396226415094, "support": 424}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6125, "precision": 1.0, "recall": 0.44144144144144143, "support": 222}, "macro avg": {"f1-score": 0.5611141998210156, "precision": 0.9243065922348366, "recall": 0.4480292621841287, "support": 4170}, "micro avg": {"f1-score": 0.738755980861244, "precision": 0.859141494435612, "recall": 0.6479616306954437, "support": 4170}, "weighted avg": {"f1-score": 0.6987860835396791, "precision": 0.8692779561961671, "recall": 0.6479616306954437, "support": 4170}, "\u2205": {"f1-score": 0.8536631066142026, "precision": 0.851588785046729, "recall": 0.8557475582268971, "support": 2662}, "\u23ce": {"f1-score": 0.3814432989690722, "precision": 1.0, "recall": 0.2356687898089172, "support": 314}, "\u2423": {"f1-score": 0.3968503937007874, "precision": 0.8456375838926175, "recall": 0.25925925925925924, "support": 972}},
  "ppcr": 0.7541966426858513
}
```
</details>
