# Train report for javascript / file:///tmp/top-repos-quality-repos-beafqjhu/angular-starter.git HEAD 610df4725415c9826df81eb36f68765013f0cdcb

### Classification report

PPCR: 0.404

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.999| 0.481| 0.977| 0.640| 1113| 2310| 0.482 |
| `'` | 1.000| 1.000| 0.753| 1.000| 0.859| 542| 720| 0.753 |
| `␣` | 0.980| 0.938| 0.335| 0.959| 0.500| 421| 1178| 0.357 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 169| 0.148 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 497| 0.008 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 173| 0.017 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 165| 0.000 |
| `micro avg` | 0.972| 0.972| 0.393| 0.972| 0.560| 2108| 5212| 0.404 |
| `macro avg` | 0.419| 0.420| 0.224| 0.419| 0.286| 2108| 5212| 0.404 |
| `weighted avg` | 0.958| 0.972| 0.393| 0.965| 0.515| 2108| 5212| 0.404 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1197 |1112 |1 |0 |0 |0 |0 |0 |
|757 |26 |395 |0 |0 |0 |0 |0 |
|178 |0 |0 |542 |0 |0 |0 |0 |
|493 |0 |4 |0 |0 |0 |0 |0 |
|170 |0 |3 |0 |0 |0 |0 |0 |
|165 |0 |0 |0 |0 |0 |0 |0 |
|144 |25 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| config/webpack.prod.js | 10 |
| config/github-deploy/index.js | 8 |
| config/html-elements-plugin/index.js | 7 |
| webpack.config.js | 7 |
| config/webpack.dev.js | 7 |
| config/webpack.common.js | 6 |
| config/karma.conf.js | 5 |
| config/webpack.test.js | 3 |
| config/webpack.github-deploy.js | 2 |
| config/protractor.conf.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 542}, "macro avg": {"f1-score": 0.41941296627170327, "precision": 0.4194709681076974, "recall": 0.4196205439554928, "support": 2108}, "micro avg": {"f1-score": 0.972011385199241, "precision": 0.972011385199241, "recall": 0.972011385199241, "support": 2108}, "weighted avg": {"f1-score": 0.9645160428278428, "precision": 0.9577017482124216, "recall": 0.972011385199241, "support": 2108}, "\u2205": {"f1-score": 0.977152899824253, "precision": 0.9561478933791917, "recall": 0.9991015274034142, "support": 1113}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u2423": {"f1-score": 0.9587378640776699, "precision": 0.9801488833746899, "recall": 0.9382422802850356, "support": 421}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8589540412044374, "precision": 1.0, "recall": 0.7527777777777778, "support": 720}, "macro avg": {"f1-score": 0.2855723347304426, "precision": 0.4194709681076974, "recall": 0.22421102154912487, "support": 5212}, "micro avg": {"f1-score": 0.5598360655737705, "precision": 0.972011385199241, "recall": 0.39313123561013047, "support": 5212}, "weighted avg": {"f1-score": 0.5154116900117467, "precision": 0.7834453220109971, "recall": 0.39313123561013047, "support": 5212}, "\u2205": {"f1-score": 0.6403685574431327, "precision": 0.9561478933791917, "recall": 0.48138528138528136, "support": 2310}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 497}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 173}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 169}, "\u2423": {"f1-score": 0.4996837444655281, "precision": 0.9801488833746899, "recall": 0.33531409168081494, "support": 1178}},
  "ppcr": 0.4044512663085188
}
```
</details>
