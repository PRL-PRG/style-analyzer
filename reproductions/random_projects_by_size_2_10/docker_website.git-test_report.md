# Test report for javascript / file:///tmp/top-repos-quality-repos-8losb434/docker_website.git HEAD ecbabb3a1a20a0e6c4346dc16fda1762ce42afc0

### Classification report

PPCR: 0.742

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.962| 1.000| 0.897| 0.981| 0.928| 505| 563| 0.897 |
| `⏎` | 1.000| 1.000| 0.881| 1.000| 0.937| 52| 59| 0.881 |
| `'` | 1.000| 0.714| 0.323| 0.833| 0.488| 28| 62| 0.452 |
| `␣` | 1.000| 0.500| 0.093| 0.667| 0.170| 24| 129| 0.186 |
| `⏎␣⁺␣⁺` | 1.000| 1.000| 1.000| 1.000| 1.000| 12| 12| 1.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `macro avg` | 0.827| 0.702| 0.532| 0.747| 0.587| 621| 837| 0.742 |
| `micro avg` | 0.968| 0.968| 0.718| 0.968| 0.824| 621| 837| 0.742 |
| `weighted avg` | 0.969| 0.968| 0.718| 0.964| 0.767| 621| 837| 0.742 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|58 |505 |0 |0 |0 |0 |0 |
|105 |12 |12 |0 |0 |0 |0 |
|7 |0 |0 |52 |0 |0 |0 |
|34 |8 |0 |0 |20 |0 |0 |
|0 |0 |0 |0 |0 |12 |0 |
|12 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8333333333333333, "precision": 1.0, "recall": 0.7142857142857143, "support": 28}, "macro avg": {"f1-score": 0.7467637540453075, "precision": 0.826984126984127, "recall": 0.7023809523809524, "support": 621}, "micro avg": {"f1-score": 0.9677938808373591, "precision": 0.9677938808373591, "recall": 0.9677938808373591, "support": 621}, "weighted avg": {"f1-score": 0.963812412384243, "precision": 0.9690207806149836, "recall": 0.9677938808373591, "support": 621}, "\u2205": {"f1-score": 0.9805825242718447, "precision": 0.9619047619047619, "recall": 1.0, "support": 505}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 52}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 24}},
  "cl_report_full": {"\u0027": {"f1-score": 0.4878048780487805, "precision": 1.0, "recall": 0.3225806451612903, "support": 62}, "macro avg": {"f1-score": 0.587210567412096, "precision": 0.826984126984127, "recall": 0.5323233824983927, "support": 837}, "micro avg": {"f1-score": 0.8244170096021948, "precision": 0.9677938808373591, "recall": 0.7180406212664278, "support": 837}, "weighted avg": {"f1-score": 0.7671666620954279, "precision": 0.9600386869204074, "recall": 0.7180406212664278, "support": 837}, "\u2205": {"f1-score": 0.9283088235294117, "precision": 0.9619047619047619, "recall": 0.8969804618117229, "support": 563}, "\u23ce": {"f1-score": 0.936936936936937, "precision": 1.0, "recall": 0.8813559322033898, "support": 59}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.1702127659574468, "precision": 1.0, "recall": 0.09302325581395349, "support": 129}},
  "ppcr": 0.7419354838709677
}
```
</details>
