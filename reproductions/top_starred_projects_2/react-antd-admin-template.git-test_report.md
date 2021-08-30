# Test report for javascript / file:///tmp/top-repos-quality-repos-994dmf85/react-antd-admin-template.git HEAD 096a1d5bd8bb140b6fe74ab5905d2f3d86399308

### Classification report

PPCR: 0.478

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.924| 0.993| 0.847| 0.957| 0.884| 854| 1001| 0.853 |
| `␣` | 0.952| 0.805| 0.188| 0.872| 0.314| 174| 745| 0.234 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 157| 0.083 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 68| 0.162 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 85| 0.118 |
| `"` | 0.273| 0.375| 0.038| 0.316| 0.067| 8| 78| 0.103 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 92| 0.065 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 26| 0.000 |
| `micro avg` | 0.921| 0.921| 0.440| 0.921| 0.596| 1076| 2252| 0.478 |
| `macro avg` | 0.269| 0.272| 0.134| 0.268| 0.158| 1076| 2252| 0.478 |
| `weighted avg` | 0.889| 0.921| 0.440| 0.903| 0.499| 1076| 2252| 0.478 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|147 |848 |6 |0 |0 |0 |0 |0 |0 |
|571 |34 |140 |0 |0 |0 |0 |0 |0 |
|144 |12 |1 |0 |0 |0 |0 |0 |0 |
|70 |5 |0 |0 |3 |0 |0 |0 |0 |
|57 |3 |0 |0 |8 |0 |0 |0 |0 |
|86 |6 |0 |0 |0 |0 |0 |0 |0 |
|75 |10 |0 |0 |0 |0 |0 |0 |0 |
|26 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.3157894736842105, "precision": 0.2727272727272727, "recall": 0.375, "support": 8}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "macro avg": {"f1-score": 0.2681467783084005, "precision": 0.26860693772458477, "recall": 0.2715714925031629, "support": 1076}, "micro avg": {"f1-score": 0.921003717472119, "precision": 0.921003717472119, "recall": 0.921003717472119, "support": 1076}, "weighted avg": {"f1-score": 0.9030431944429991, "precision": 0.889197284561378, "recall": 0.921003717472119, "support": 1076}, "\u2205": {"f1-score": 0.9571106094808126, "precision": 0.9237472766884531, "recall": 0.9929742388758782, "support": 854}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.8722741433021808, "precision": 0.9523809523809523, "recall": 0.8045977011494253, "support": 174}},
  "cl_report_full": {"\"": {"f1-score": 0.06741573033707866, "precision": 0.2727272727272727, "recall": 0.038461538461538464, "support": 78}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "macro avg": {"f1-score": 0.15813883976883816, "precision": 0.26860693772458477, "recall": 0.13419173108770424, "support": 2252}, "micro avg": {"f1-score": 0.5955528846153846, "precision": 0.921003717472119, "recall": 0.4400532859680284, "support": 2252}, "weighted avg": {"f1-score": 0.49901970043211086, "precision": 0.7351099292902658, "recall": 0.4400532859680284, "support": 2252}, "\u2205": {"f1-score": 0.883793642522147, "precision": 0.9237472766884531, "recall": 0.8471528471528471, "support": 1001}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 85}, "\u2423": {"f1-score": 0.31390134529147984, "precision": 0.9523809523809523, "recall": 0.18791946308724833, "support": 745}},
  "ppcr": 0.477797513321492
}
```
</details>
