# Test report for javascript / file:///tmp/top-repos-quality-repos-93utj6q6/lisk-roulette.git HEAD 4f30d12e9c770f1a1d532b91e56271091ed3cd83

### Classification report

PPCR: 0.853

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.920| 0.981| 0.951| 0.949| 0.935| 2279| 2350| 0.970 |
| `␣` | 0.424| 0.729| 0.517| 0.536| 0.465| 365| 515| 0.709 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 272| 476| 0.571 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 79| 120| 0.658 |
| `⏎␣⁻␣⁻` | 1.000| 0.603| 0.493| 0.753| 0.660| 58| 71| 0.817 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 78| 0.692 |
| `'` | 0.344| 0.468| 0.333| 0.396| 0.338| 47| 66| 0.712 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 26| 0.115 |
| `weighted avg` | 0.736| 0.810| 0.691| 0.767| 0.677| 3157| 3702| 0.853 |
| `micro avg` | 0.810| 0.810| 0.691| 0.810| 0.746| 3157| 3702| 0.853 |
| `macro avg` | 0.336| 0.348| 0.287| 0.329| 0.300| 3157| 3702| 0.853 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|71 |2235 |44 |0 |0 |0 |0 |0 |0 |
|150 |99 |266 |0 |0 |0 |0 |0 |0 |
|41 |33 |46 |0 |0 |0 |0 |0 |0 |
|19 |0 |25 |0 |22 |0 |0 |0 |0 |
|204 |0 |233 |0 |39 |0 |0 |0 |0 |
|24 |41 |10 |0 |3 |0 |0 |0 |0 |
|13 |19 |4 |0 |0 |0 |0 |35 |0 |
|23 |3 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 272}, "\u0027": {"f1-score": 0.39639639639639634, "precision": 0.34375, "recall": 0.46808510638297873, "support": 47}, "macro avg": {"f1-score": 0.32926011808053757, "precision": 0.3358837456750806, "recall": 0.3476242240077373, "support": 3157}, "micro avg": {"f1-score": 0.8102629078238834, "precision": 0.8102629078238834, "recall": 0.8102629078238834, "support": 3157}, "weighted avg": {"f1-score": 0.7669202737114236, "precision": 0.73641920962263, "recall": 0.8102629078238834, "support": 3157}, "\u2205": {"f1-score": 0.9492461244425567, "precision": 0.9197530864197531, "recall": 0.9806932865291794, "support": 2279}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7526881720430108, "precision": 1.0, "recall": 0.603448275862069, "support": 58}, "\u2423": {"f1-score": 0.5357502517623365, "precision": 0.42356687898089174, "recall": 0.7287671232876712, "support": 365}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 476}, "\u0027": {"f1-score": 0.3384615384615385, "precision": 0.34375, "recall": 0.3333333333333333, "support": 66}, "macro avg": {"f1-score": 0.2999283950299097, "precision": 0.3358837456750806, "recall": 0.28673247049604655, "support": 3702}, "micro avg": {"f1-score": 0.7458813238081353, "precision": 0.8102629078238834, "recall": 0.690977849810913, "support": 3702}, "weighted avg": {"f1-score": 0.6770723753197079, "precision": 0.668083251151156, "recall": 0.690977849810913, "support": 3702}, "\u2205": {"f1-score": 0.9351464435146444, "precision": 0.9197530864197531, "recall": 0.951063829787234, "support": 2350}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 120}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 78}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.660377358490566, "precision": 1.0, "recall": 0.49295774647887325, "support": 71}, "\u2423": {"f1-score": 0.4654418197725284, "precision": 0.42356687898089174, "recall": 0.516504854368932, "support": 515}},
  "ppcr": 0.8527822798487305
}
```
</details>
