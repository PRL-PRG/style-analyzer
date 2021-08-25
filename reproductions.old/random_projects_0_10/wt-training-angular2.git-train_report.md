# Train report for javascript / file:///tmp/top-repos-quality-repos-h4rh65t4/wt-training-angular2.git HEAD 303f2d12f8596da821dc63ab70b1b9be4a0afde9

### Classification report

PPCR: 0.451

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.921| 1.000| 0.587| 0.959| 0.717| 1004| 1710| 0.587 |
| `␣` | 0.975| 0.806| 0.302| 0.883| 0.461| 345| 922| 0.374 |
| `'` | 1.000| 1.000| 0.720| 1.000| 0.837| 321| 446| 0.720 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 97| 0.144 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 336| 0.021 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 100| 0.050 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 146| 0.000 |
| `micro avg` | 0.945| 0.945| 0.427| 0.945| 0.588| 1696| 3757| 0.451 |
| `weighted avg` | 0.933| 0.945| 0.427| 0.936| 0.539| 1696| 3757| 0.451 |
| `macro avg` | 0.414| 0.401| 0.230| 0.406| 0.288| 1696| 3757| 0.451 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|706 |1004 |0 |0 |0 |0 |0 |0 |
|577 |67 |278 |0 |0 |0 |0 |0 |
|125 |0 |0 |321 |0 |0 |0 |0 |
|329 |3 |4 |0 |0 |0 |0 |0 |
|146 |0 |0 |0 |0 |0 |0 |0 |
|95 |2 |3 |0 |0 |0 |0 |0 |
|83 |14 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| config/html-elements-plugin/index.js | 25 |
| config/webpack.prod.js | 11 |
| config/github-deploy/index.js | 11 |
| webpack.config.js | 10 |
| config/webpack.dev.js | 6 |
| config/helpers.js | 5 |
| config/karma.conf.js | 5 |
| config/webpack.test.js | 5 |
| config/webpack.github-deploy.js | 4 |
| config/protractor.conf.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 321}, "macro avg": {"f1-score": 0.4059242799316479, "precision": 0.4137913591317744, "recall": 0.40082815734989646, "support": 1696}, "micro avg": {"f1-score": 0.9451650943396226, "precision": 0.9451650943396226, "recall": 0.9451650943396226, "support": 1696}, "weighted avg": {"f1-score": 0.9364635545789922, "precision": 0.9329667670344287, "recall": 0.9451650943396226, "support": 1696}, "\u2205": {"f1-score": 0.9589302769818528, "precision": 0.9211009174311927, "recall": 1.0, "support": 1004}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.8825396825396826, "precision": 0.9754385964912281, "recall": 0.8057971014492754, "support": 345}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8370273794002607, "precision": 1.0, "recall": 0.7197309417040358, "support": 446}, "macro avg": {"f1-score": 0.28783092383803327, "precision": 0.4137913591317744, "recall": 0.22976912611512665, "support": 3757}, "micro avg": {"f1-score": 0.5879332477535302, "precision": 0.9451650943396226, "recall": 0.42667021559755125, "support": 3757}, "weighted avg": {"f1-score": 0.5388193562109966, "precision": 0.7773321678925345, "recall": 0.42667021559755125, "support": 3757}, "\u2205": {"f1-score": 0.7171428571428571, "precision": 0.9211009174311927, "recall": 0.5871345029239766, "support": 1710}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 336}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u2423": {"f1-score": 0.4606462303231152, "precision": 0.9754385964912281, "recall": 0.30151843817787416, "support": 922}},
  "ppcr": 0.45142400851743414
}
```
</details>
