# Test report for javascript / file:///tmp/top-repos-quality-repos-0wjv4vks/money-mapper.git HEAD 358814894935652004760f9731e2b7a77d33aace

### Classification report

PPCR: 0.965

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.782| 0.995| 0.988| 0.876| 0.873| 853| 859| 0.993 |
| `␣` | 0.891| 0.802| 0.784| 0.844| 0.834| 389| 398| 0.977 |
| `⏎` | 0.863| 0.295| 0.288| 0.440| 0.431| 149| 153| 0.974 |
| `'` | 0.949| 0.903| 0.738| 0.925| 0.830| 103| 126| 0.817 |
| `⏎␣⁻␣⁻` | 0.978| 0.880| 0.786| 0.926| 0.871| 50| 56| 0.893 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 45| 56| 0.804 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 41| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.823| 0.823| 0.795| 0.823| 0.809| 1630| 1689| 0.965 |
| `weighted avg` | 0.791| 0.823| 0.795| 0.787| 0.770| 1630| 1689| 0.965 |
| `macro avg` | 0.558| 0.484| 0.448| 0.501| 0.480| 1630| 1689| 0.965 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|6 |849 |3 |0 |0 |0 |1 |0 |
|9 |65 |312 |5 |7 |0 |0 |0 |
|23 |2 |8 |93 |0 |0 |0 |0 |
|4 |87 |18 |0 |44 |0 |0 |0 |
|11 |43 |2 |0 |0 |0 |0 |0 |
|6 |6 |0 |0 |0 |0 |44 |0 |
|0 |34 |7 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9253731343283583, "precision": 0.9489795918367347, "recall": 0.9029126213592233, "support": 103}, "macro avg": {"f1-score": 0.5014727944163833, "precision": 0.5578373743604256, "recall": 0.48444773228521776, "support": 1630}, "micro avg": {"f1-score": 0.8233128834355828, "precision": 0.8233128834355828, "recall": 0.8233128834355828, "support": 1630}, "weighted avg": {"f1-score": 0.7868923943310727, "precision": 0.7906727527852591, "recall": 0.8233128834355828, "support": 1630}, "\u2205": {"f1-score": 0.8757091284167096, "precision": 0.7817679558011049, "recall": 0.9953106682297772, "support": 853}, "\u23ce": {"f1-score": 0.44000000000000006, "precision": 0.8627450980392157, "recall": 0.2953020134228188, "support": 149}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9263157894736842, "precision": 0.9777777777777777, "recall": 0.88, "support": 50}, "\u2423": {"f1-score": 0.8443843031123139, "precision": 0.8914285714285715, "recall": 0.8020565552699229, "support": 389}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.8303571428571429, "precision": 0.9489795918367347, "recall": 0.7380952380952381, "support": 126}, "macro avg": {"f1-score": 0.4800311414502957, "precision": 0.5578373743604256, "recall": 0.44795867220085994, "support": 1689}, "micro avg": {"f1-score": 0.808677312443507, "precision": 0.8233128834355828, "recall": 0.7945529899348727, "support": 1689}, "weighted avg": {"f1-score": 0.7704867343167056, "precision": 0.7890196741200146, "recall": 0.7945529899348727, "support": 1689}, "\u2205": {"f1-score": 0.8730077120822622, "precision": 0.7817679558011049, "recall": 0.9883585564610011, "support": 859}, "\u23ce": {"f1-score": 0.43137254901960786, "precision": 0.8627450980392157, "recall": 0.2875816993464052, "support": 153}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8712871287128713, "precision": 0.9777777777777777, "recall": 0.7857142857142857, "support": 56}, "\u2423": {"f1-score": 0.8342245989304814, "precision": 0.8914285714285715, "recall": 0.7839195979899497, "support": 398}},
  "ppcr": 0.9650680876258141
}
```
</details>
