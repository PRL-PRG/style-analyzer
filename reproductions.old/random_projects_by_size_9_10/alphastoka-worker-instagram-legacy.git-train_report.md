# Train report for javascript / file:///tmp/top-repos-quality-repos-oasx7q99/alphastoka-worker-instagram-legacy.git HEAD c0f5f1f81154dec615a1c47deec2f41ec09d339b

### Classification report

PPCR: 0.242

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.996| 0.996| 0.429| 0.996| 0.599| 1665| 3870| 0.430 |
| `␣` | 0.956| 0.992| 0.061| 0.974| 0.115| 130| 2115| 0.061 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 389| 0.005 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 129| 0.016 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 89| 0.011 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 250| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 238| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 140| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 125| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 83| 0.000 |
| `micro avg` | 0.993| 0.993| 0.241| 0.993| 0.388| 1800| 7428| 0.242 |
| `weighted avg` | 0.991| 0.993| 0.241| 0.992| 0.345| 1800| 7428| 0.242 |
| `macro avg` | 0.195| 0.199| 0.049| 0.197| 0.071| 1800| 7428| 0.242 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2205 |1659 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|1985 |1 |129 |0 |0 |0 |0 |0 |0 |0 |0 |
|387 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|250 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|238 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|140 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|127 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|125 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|88 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|83 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| walker.js | 5 |
| lnwwalka.js | 3 |
| scripts/stomp.js | 2 |
| jsontoexcel.js | 1 |
| split.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19699813020567739, "precision": 0.1951951951951952, "recall": 0.19887040887040888, "support": 1800}, "micro avg": {"f1-score": 0.9933333333333333, "precision": 0.9933333333333333, "recall": 0.9933333333333333, "support": 1800}, "weighted avg": {"f1-score": 0.9919811320754717, "precision": 0.990679012345679, "recall": 0.9933333333333333, "support": 1800}, "\u2205": {"f1-score": 0.9963963963963964, "precision": 0.9963963963963964, "recall": 0.9963963963963964, "support": 1665}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9735849056603774, "precision": 0.9555555555555556, "recall": 0.9923076923076923, "support": 130}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 250}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 238}, "macro avg": {"f1-score": 0.07141246612466126, "precision": 0.1951951951951952, "recall": 0.048967507834405406, "support": 7428}, "micro avg": {"f1-score": 0.3875162548764629, "precision": 0.9933333333333333, "recall": 0.2407108239095315, "support": 7428}, "weighted avg": {"f1-score": 0.3449680181777585, "precision": 0.7912027536421721, "recall": 0.2407108239095315, "support": 7428}, "\u2205": {"f1-score": 0.5994579945799459, "precision": 0.9963963963963964, "recall": 0.42868217054263563, "support": 3870}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 389}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 125}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 89}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 129}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u2423": {"f1-score": 0.11466666666666665, "precision": 0.9555555555555556, "recall": 0.06099290780141844, "support": 2115}},
  "ppcr": 0.24232633279483037
}
```
</details>
