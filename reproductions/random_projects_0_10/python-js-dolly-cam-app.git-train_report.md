# Train report for javascript / file:///tmp/top-repos-quality-repos-v9ta4xr5/python-js-dolly-cam-app.git HEAD beeec04d8f47555a4ebd98f4b59d153dce57a8d2

### Classification report

PPCR: 0.148

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 0.992| 0.227| 0.994| 0.370| 1154| 5042| 0.229 |
| `␣` | 0.961| 0.984| 0.123| 0.973| 0.218| 253| 2025| 0.125 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 127| 0.008 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 636| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 690| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 429| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 165| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 151| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 126| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 97| 0.000 |
| `micro avg` | 0.990| 0.990| 0.147| 0.990| 0.256| 1408| 9488| 0.148 |
| `weighted avg` | 0.989| 0.990| 0.147| 0.990| 0.243| 1408| 9488| 0.148 |
| `macro avg` | 0.196| 0.198| 0.035| 0.197| 0.059| 1408| 9488| 0.148 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3888 |1145 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|1772 |4 |249 |0 |0 |0 |0 |0 |0 |0 |0 |
|636 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|690 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|429 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|165 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|151 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|126 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|126 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|97 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/contexts/AppSettingsContext.js | 8 |
| sandbox/sockets/static/js/app.js | 2 |
| client/src/components/KeyboardControlsCanvas.js | 1 |
| client/src/components/Status/StepsTakenIndicator.js | 1 |
| client/src/actions/socketActions.js | 1 |
| api/v1/static/assets/js/app.v0.2.6.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1967011438884064, "precision": 0.1957908673313373, "recall": 0.19763907631815097, "support": 1408}, "micro avg": {"f1-score": 0.9900568181818182, "precision": 0.9900568181818182, "recall": 0.9900568181818182, "support": 1408}, "weighted avg": {"f1-score": 0.9897499426294106, "precision": 0.9894987597949413, "recall": 0.9900568181818182, "support": 1408}, "\u2205": {"f1-score": 0.9943551888840643, "precision": 0.9965187119234117, "recall": 0.9922010398613518, "support": 1154}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9726562499999999, "precision": 0.9613899613899614, "recall": 0.9841897233201581, "support": 253}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 636}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 690}, "macro avg": {"f1-score": 0.0587930307284655, "precision": 0.1957908673313373, "recall": 0.035005538660437506, "support": 9488}, "micro avg": {"f1-score": 0.25587371512481644, "precision": 0.9900568181818182, "recall": 0.14692242833052277, "support": 9488}, "weighted avg": {"f1-score": 0.24309890046879948, "precision": 0.7347451535974403, "recall": 0.14692242833052277, "support": 9488}, "\u2205": {"f1-score": 0.36989177838798254, "precision": 0.9965187119234117, "recall": 0.22709242364141213, "support": 5042}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 429}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 127}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 126}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u2423": {"f1-score": 0.2180385288966725, "precision": 0.9613899613899614, "recall": 0.12296296296296297, "support": 2025}},
  "ppcr": 0.14839797639123103
}
```
</details>
