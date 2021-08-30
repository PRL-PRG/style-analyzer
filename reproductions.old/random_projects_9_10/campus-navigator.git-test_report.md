# Test report for javascript / file:///tmp/top-repos-quality-repos-aes85ail/campus-navigator.git HEAD 42004fbd9b618b68d7430d91d10190ecac7dc7d4

### Classification report

PPCR: 0.722

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.861| 0.950| 0.788| 0.904| 0.823| 281| 339| 0.829 |
| `␣` | 0.812| 0.798| 0.572| 0.805| 0.672| 114| 159| 0.717 |
| `⏎` | 0.893| 0.806| 0.641| 0.847| 0.746| 31| 39| 0.795 |
| `'` | 0.879| 1.000| 0.659| 0.935| 0.753| 29| 44| 0.659 |
| `⏎␣⁺␣⁺` | 1.000| 0.467| 0.206| 0.636| 0.341| 15| 34| 0.441 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 22| 0.273 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 26| 0.192 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 8| 0.500 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 6| 0.667 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 2| 0.500 |
| `micro avg` | 0.855| 0.855| 0.617| 0.855| 0.717| 490| 679| 0.722 |
| `macro avg` | 0.445| 0.402| 0.287| 0.413| 0.334| 490| 679| 0.722 |
| `weighted avg` | 0.822| 0.855| 0.617| 0.834| 0.677| 490| 679| 0.722 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|58 |267 |14 |0 |0 |0 |0 |0 |0 |0 |0 |
|45 |23 |91 |0 |0 |0 |0 |0 |0 |0 |0 |
|8 |5 |1 |25 |0 |0 |0 |0 |0 |0 |0 |
|15 |0 |0 |0 |29 |0 |0 |0 |0 |0 |0 |
|19 |5 |3 |0 |0 |7 |0 |0 |0 |0 |0 |
|16 |1 |2 |3 |0 |0 |0 |0 |0 |0 |0 |
|21 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|2 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |4 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u0027": {"f1-score": 0.9354838709677419, "precision": 0.8787878787878788, "recall": 1.0, "support": 29}, "macro avg": {"f1-score": 0.41281681684556826, "precision": 0.44454353442256667, "recall": 0.4021541829548041, "support": 490}, "micro avg": {"f1-score": 0.8551020408163266, "precision": 0.8551020408163266, "recall": 0.8551020408163266, "support": 490}, "weighted avg": {"f1-score": 0.8339784900736137, "precision": 0.8220632664460841, "recall": 0.8551020408163266, "support": 490}, "\u2205": {"f1-score": 0.9035532994923858, "precision": 0.8612903225806452, "recall": 0.9501779359430605, "support": 281}, "\u23ce": {"f1-score": 0.8474576271186439, "precision": 0.8928571428571429, "recall": 0.8064516129032258, "support": 31}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6363636363636364, "precision": 1.0, "recall": 0.4666666666666667, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.8053097345132744, "precision": 0.8125, "recall": 0.7982456140350878, "support": 114}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u0027": {"f1-score": 0.7532467532467532, "precision": 0.8787878787878788, "recall": 0.6590909090909091, "support": 44}, "macro avg": {"f1-score": 0.3335369854794214, "precision": 0.44454353442256667, "recall": 0.2865936566551911, "support": 679}, "micro avg": {"f1-score": 0.716852010265184, "precision": 0.8551020408163266, "recall": 0.6170839469808542, "support": 679}, "weighted avg": {"f1-score": 0.6768336395184357, "precision": 0.7785758683253813, "recall": 0.6170839469808542, "support": 679}, "\u2205": {"f1-score": 0.8228043143297381, "precision": 0.8612903225806452, "recall": 0.7876106194690266, "support": 339}, "\u23ce": {"f1-score": 0.746268656716418, "precision": 0.8928571428571429, "recall": 0.6410256410256411, "support": 39}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.34146341463414637, "precision": 1.0, "recall": 0.20588235294117646, "support": 34}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.6715867158671586, "precision": 0.8125, "recall": 0.5723270440251572, "support": 159}},
  "ppcr": 0.7216494845360825
}
```
</details>