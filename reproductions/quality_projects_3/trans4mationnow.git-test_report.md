# Test report for javascript / file:///tmp/top-repos-quality-repos-2p43v1hn/trans4mationnow.git HEAD fa4ae9a3dd29627f02bcbfb8de842c31ce548e61

### Classification report

PPCR: 0.815

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.888| 0.991| 0.960| 0.937| 0.923| 2199| 2271| 0.968 |
| `␣` | 0.890| 0.724| 0.571| 0.799| 0.696| 493| 625| 0.789 |
| `'` | 0.891| 0.991| 0.494| 0.939| 0.635| 232| 466| 0.498 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 68| 93| 0.731 |
| `⏎` | 1.000| 0.048| 0.014| 0.092| 0.028| 62| 213| 0.291 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 84| 0.476 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 57| 0.193 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 14| 0.714 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 1| 1.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `weighted avg` | 0.854| 0.889| 0.724| 0.859| 0.741| 3116| 3825| 0.815 |
| `micro avg` | 0.889| 0.889| 0.724| 0.889| 0.798| 3116| 3825| 0.815 |
| `macro avg` | 0.367| 0.276| 0.204| 0.277| 0.228| 3116| 3825| 0.815 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|72 |2180 |19 |0 |0 |0 |0 |0 |0 |0 |0 |
|132 |122 |357 |14 |0 |0 |0 |0 |0 |0 |0 |
|234 |0 |2 |230 |0 |0 |0 |0 |0 |0 |0 |
|151 |55 |4 |0 |3 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |10 |0 |0 |0 |0 |0 |0 |0 |
|25 |52 |12 |4 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|44 |36 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|46 |8 |3 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.9387755102040817, "precision": 0.8914728682170543, "recall": 0.9913793103448276, "support": 232}, "macro avg": {"f1-score": 0.2766770793832665, "precision": 0.36700927407037265, "recall": 0.27552640471121215, "support": 3116}, "micro avg": {"f1-score": 0.8889602053915276, "precision": 0.8889602053915276, "recall": 0.8889602053915276, "support": 3116}, "weighted avg": {"f1-score": 0.859366476730093, "precision": 0.8540432686054794, "recall": 0.8889602053915276, "support": 3116}, "\u2205": {"f1-score": 0.9370298732000859, "precision": 0.8883455582722086, "recall": 0.9913597089586176, "support": 2199}, "\u23ce": {"f1-score": 0.09230769230769231, "precision": 1.0, "recall": 0.04838709677419355, "support": 62}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u2423": {"f1-score": 0.7986577181208052, "precision": 0.8902743142144638, "recall": 0.7241379310344828, "support": 493}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u0027": {"f1-score": 0.6353591160220994, "precision": 0.8914728682170543, "recall": 0.49356223175965663, "support": 466}, "macro avg": {"f1-score": 0.22817946492997382, "precision": 0.36700927407037265, "recall": 0.20387762852572164, "support": 3825}, "micro avg": {"f1-score": 0.7981558853191183, "precision": 0.8889602053915276, "recall": 0.7241830065359477, "support": 3825}, "weighted avg": {"f1-score": 0.7405238664494261, "precision": 0.8371975335449341, "recall": 0.7241830065359477, "support": 3825}, "\u2205": {"f1-score": 0.9227513227513227, "precision": 0.8883455582722086, "recall": 0.959929546455306, "support": 2271}, "\u23ce": {"f1-score": 0.02777777777777778, "precision": 1.0, "recall": 0.014084507042253521, "support": 213}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 93}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u2423": {"f1-score": 0.695906432748538, "precision": 0.8902743142144638, "recall": 0.5712, "support": 625}},
  "ppcr": 0.814640522875817
}
```
</details>
