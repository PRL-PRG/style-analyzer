# Test report for javascript / file:///tmp/top-repos-quality-repos-xqpqsc76/event-admin.git HEAD 774b6aa49cd816b004202f065c8e14ecaf707898

### Classification report

PPCR: 0.637

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.857| 1.000| 0.839| 0.923| 0.848| 203| 242| 0.839 |
| `"` | 1.000| 1.000| 0.856| 1.000| 0.922| 77| 90| 0.856 |
| `␣` | 1.000| 0.646| 0.226| 0.785| 0.369| 48| 137| 0.350 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 45| 0.267 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 14| 0.214 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 14| 0.143 |
| `micro avg` | 0.901| 0.901| 0.574| 0.901| 0.701| 345| 542| 0.637 |
| `weighted avg` | 0.866| 0.901| 0.574| 0.875| 0.625| 345| 542| 0.637 |
| `macro avg` | 0.476| 0.441| 0.320| 0.451| 0.356| 345| 542| 0.637 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|39 |203 |0 |0 |0 |0 |0 |
|89 |17 |31 |0 |0 |0 |0 |
|13 |0 |0 |77 |0 |0 |0 |
|33 |12 |0 |0 |0 |0 |0 |
|11 |3 |0 |0 |0 |0 |0 |
|12 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 77}, "macro avg": {"f1-score": 0.45125623321825853, "precision": 0.4760900140646976, "recall": 0.44097222222222227, "support": 345}, "micro avg": {"f1-score": 0.9014492753623189, "precision": 0.9014492753623189, "recall": 0.9014492753623189, "support": 345}, "weighted avg": {"f1-score": 0.8753174563466253, "precision": 0.866311991683483, "recall": 0.9014492753623189, "support": 345}, "\u2205": {"f1-score": 0.9227272727272727, "precision": 0.8565400843881856, "recall": 1.0, "support": 203}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.7848101265822784, "precision": 1.0, "recall": 0.6458333333333334, "support": 48}},
  "cl_report_full": {"\"": {"f1-score": 0.9221556886227544, "precision": 1.0, "recall": 0.8555555555555555, "support": 90}, "macro avg": {"f1-score": 0.3564670787662174, "precision": 0.4760900140646976, "recall": 0.32011265050415677, "support": 542}, "micro avg": {"f1-score": 0.7012401352874859, "precision": 0.9014492753623189, "recall": 0.5738007380073801, "support": 542}, "weighted avg": {"f1-score": 0.6248570732433377, "precision": 0.8012595948744298, "recall": 0.5738007380073801, "support": 542}, "\u2205": {"f1-score": 0.8475991649269311, "precision": 0.8565400843881856, "recall": 0.8388429752066116, "support": 242}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.369047619047619, "precision": 1.0, "recall": 0.22627737226277372, "support": 137}},
  "ppcr": 0.6365313653136532
}
```
</details>
