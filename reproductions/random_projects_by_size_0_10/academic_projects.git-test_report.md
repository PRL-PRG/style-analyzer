# Test report for javascript / file:///tmp/top-repos-quality-repos-io60klpb/academic_projects.git HEAD 16859c4d586c7d1200631e4aadee46a7bb518ecd

### Classification report

PPCR: 0.747

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.932| 0.722| 0.961| 0.836| 1645| 2122| 0.775 |
| `␣` | 0.894| 1.000| 0.926| 0.944| 0.910| 1214| 1311| 0.926 |
| `"` | 1.000| 0.899| 0.644| 0.947| 0.784| 129| 180| 0.717 |
| `⏎` | 1.000| 0.552| 0.177| 0.712| 0.301| 67| 209| 0.321 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 140| 0.071 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 140| 0.021 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.945| 0.945| 0.706| 0.945| 0.809| 3068| 4105| 0.747 |
| `weighted avg` | 0.950| 0.945| 0.706| 0.944| 0.773| 3068| 4105| 0.747 |
| `macro avg` | 0.324| 0.282| 0.206| 0.297| 0.236| 3068| 4105| 0.747 |

### Confusion matrix

|refusal|  ␣| ∅| "| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|97 |1214 |0 |0 |0 |0 |0 |0 |0 |0 |
|477 |112 |1533 |0 |0 |0 |0 |0 |0 |0 |
|51 |2 |11 |116 |0 |0 |0 |0 |0 |0 |
|142 |30 |0 |0 |37 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|137 |0 |0 |0 |0 |3 |0 |0 |0 |0 |
|130 |0 |0 |0 |0 |0 |10 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9469387755102041, "precision": 1.0, "recall": 0.8992248062015504, "support": 129}, "macro avg": {"f1-score": 0.29699329950518777, "precision": 0.32390311300525765, "recall": 0.28194820881572674, "support": 3068}, "micro avg": {"f1-score": 0.9452411994784876, "precision": 0.9452411994784876, "recall": 0.9452411994784876, "support": 3068}, "weighted avg": {"f1-score": 0.9443974882292455, "precision": 0.9499836878766855, "recall": 0.9452411994784876, "support": 3068}, "\u2205": {"f1-score": 0.9614299153339605, "precision": 0.9928756476683938, "recall": 0.9319148936170213, "support": 1645}, "\u23ce": {"f1-score": 0.7115384615384616, "precision": 1.0, "recall": 0.5522388059701493, "support": 67}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.9440124416796268, "precision": 0.8939617083946981, "recall": 1.0, "support": 1214}},
  "cl_report_full": {"\"": {"f1-score": 0.7837837837837839, "precision": 1.0, "recall": 0.6444444444444445, "support": 180}, "macro avg": {"f1-score": 0.235886223316089, "precision": 0.32390311300525765, "recall": 0.2058266903646678, "support": 4105}, "micro avg": {"f1-score": 0.8085877596542591, "precision": 0.9452411994784876, "recall": 0.7064555420219245, "support": 4105}, "weighted avg": {"f1-score": 0.7725392074463622, "precision": 0.8935117963599953, "recall": 0.7064555420219245, "support": 4105}, "\u2205": {"f1-score": 0.8363338788870704, "precision": 0.9928756476683938, "recall": 0.7224316682375118, "support": 2122}, "\u23ce": {"f1-score": 0.3008130081300813, "precision": 1.0, "recall": 0.17703349282296652, "support": 209}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u2423": {"f1-score": 0.909704008992132, "precision": 0.8939617083946981, "recall": 0.9260106788710908, "support": 1311}},
  "ppcr": 0.7473812423873325
}
```
</details>
