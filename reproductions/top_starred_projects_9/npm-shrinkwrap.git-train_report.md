# Train report for javascript / file:///tmp/top-repos-quality-repos-xskfgb4j/npm-shrinkwrap.git HEAD 2c9016a1af6301943179ce1e552068fd60ae800c

### Classification report

PPCR: 0.619

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 1.000| 0.893| 0.992| 0.936| 4402| 4929| 0.893 |
| `'` | 1.000| 1.000| 0.907| 1.000| 0.951| 734| 809| 0.907 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.980| 0.972| 0.850| 0.976| 0.910| 397| 454| 0.874 |
| `␣` | 1.000| 0.882| 0.120| 0.937| 0.214| 382| 2815| 0.136 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.988| 0.922| 0.866| 0.954| 0.923| 358| 381| 0.940 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 379| 0.003 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 373| 0.000 |
| `weighted avg` | 0.986| 0.986| 0.610| 0.986| 0.666| 6274| 10140| 0.619 |
| `micro avg` | 0.986| 0.986| 0.610| 0.986| 0.754| 6274| 10140| 0.619 |
| `macro avg` | 0.707| 0.682| 0.519| 0.694| 0.562| 6274| 10140| 0.619 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|527 |4402 |0 |0 |0 |0 |0 |0 |
|2433 |34 |337 |0 |8 |0 |0 |3 |
|75 |0 |0 |734 |0 |0 |0 |0 |
|57 |11 |0 |0 |386 |0 |0 |0 |
|378 |0 |0 |0 |0 |0 |0 |1 |
|373 |0 |0 |0 |0 |0 |0 |0 |
|23 |28 |0 |0 |0 |0 |0 |330 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/npm-shrinkwrap.js | 13 |
| bin/diff.js | 12 |
| analyze-dependency.js | 11 |
| trim-and-sort-shrinkwrap.js | 9 |
| index.js | 9 |
| bin/cli.js | 8 |
| verify-git.js | 5 |
| sync/force-install.js | 4 |
| set-resolved.js | 4 |
| trim-nested.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 734}, "macro avg": {"f1-score": 0.6941323675224497, "precision": 0.7073437906294112, "recall": 0.6823255505446509, "support": 6274}, "micro avg": {"f1-score": 0.9864520242269684, "precision": 0.9864520242269684, "recall": 0.9864520242269684, "support": 6274}, "weighted avg": {"f1-score": 0.9861015314366198, "precision": 0.986426920486698, "recall": 0.9864520242269684, "support": 6274}, "\u2205": {"f1-score": 0.9917765010701813, "precision": 0.9836871508379889, "recall": 1.0, "support": 4402}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9759797724399494, "precision": 0.9796954314720813, "recall": 0.9722921914357683, "support": 397}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.953757225433526, "precision": 0.9880239520958084, "recall": 0.9217877094972067, "support": 358}, "\u2423": {"f1-score": 0.9374130737134909, "precision": 1.0, "recall": 0.8821989528795812, "support": 382}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9513933895009721, "precision": 1.0, "recall": 0.907292954264524, "support": 809}, "macro avg": {"f1-score": 0.5621253601717766, "precision": 0.7073437906294112, "recall": 0.5194932171488534, "support": 10140}, "micro avg": {"f1-score": 0.7541123431217253, "precision": 0.9864520242269684, "recall": 0.6103550295857988, "support": 10140}, "weighted avg": {"f1-score": 0.6657925181154914, "precision": 0.91654958758553, "recall": 0.6103550295857988, "support": 10140}, "\u2205": {"f1-score": 0.9361973628243301, "precision": 0.9836871508379889, "recall": 0.8930817610062893, "support": 4929}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 379}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 373}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9103773584905659, "precision": 0.9796954314720813, "recall": 0.8502202643171806, "support": 454}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9230769230769232, "precision": 0.9880239520958084, "recall": 0.8661417322834646, "support": 381}, "\u2423": {"f1-score": 0.21383248730964466, "precision": 1.0, "recall": 0.1197158081705151, "support": 2815}},
  "ppcr": 0.6187376725838264
}
```
</details>
