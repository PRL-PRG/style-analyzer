# Test report for javascript / file:///tmp/top-repos-quality-repos-oasx7q99/alphastoka-worker-instagram-legacy.git HEAD c0f5f1f81154dec615a1c47deec2f41ec09d339b

### Classification report

PPCR: 0.593

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.954| 0.963| 0.896| 0.958| 0.924| 107| 115| 0.930 |
| `␣` | 0.829| 0.879| 0.382| 0.853| 0.523| 33| 76| 0.434 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 28| 0.071 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 10| 0.100 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `micro avg` | 0.923| 0.923| 0.548| 0.923| 0.688| 143| 241| 0.593 |
| `weighted avg` | 0.905| 0.923| 0.548| 0.914| 0.606| 143| 241| 0.593 |
| `macro avg` | 0.178| 0.184| 0.128| 0.181| 0.145| 143| 241| 0.593 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8 |103 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|43 |4 |29 |0 |0 |0 |0 |0 |0 |0 |0 |
|9 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|26 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "macro avg": {"f1-score": 0.18110807113543093, "precision": 0.17822751322751323, "recall": 0.18414047012177853, "support": 143}, "micro avg": {"f1-score": 0.9230769230769231, "precision": 0.9230769230769231, "recall": 0.9230769230769231, "support": 143}, "weighted avg": {"f1-score": 0.9137621612313814, "precision": 0.904819254819255, "recall": 0.9230769230769231, "support": 143}, "\u2205": {"f1-score": 0.958139534883721, "precision": 0.9537037037037037, "recall": 0.9626168224299065, "support": 107}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8529411764705883, "precision": 0.8285714285714286, "recall": 0.8787878787878788, "support": 33}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "macro avg": {"f1-score": 0.14462893386660203, "precision": 0.17822751322751323, "recall": 0.12772311212814647, "support": 241}, "micro avg": {"f1-score": 0.6875000000000001, "precision": 0.9230769230769231, "recall": 0.5477178423236515, "support": 241}, "weighted avg": {"f1-score": 0.6055804795361575, "precision": 0.7163790643043756, "recall": 0.5477178423236515, "support": 241}, "\u2205": {"f1-score": 0.9237668161434978, "precision": 0.9537037037037037, "recall": 0.8956521739130435, "support": 115}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.5225225225225226, "precision": 0.8285714285714286, "recall": 0.3815789473684211, "support": 76}},
  "ppcr": 0.5933609958506224
}
```
</details>
