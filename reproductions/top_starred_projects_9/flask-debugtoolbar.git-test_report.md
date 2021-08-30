# Test report for javascript / file:///tmp/top-repos-quality-repos-gqx56fkx/flask-debugtoolbar.git HEAD d474a6a689be916d65c2adf173e6517290902abe

### Classification report

PPCR: 0.902

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 0.980| 0.943| 0.984| 0.964| 858| 892| 0.962 |
| `␣` | 0.936| 0.975| 0.903| 0.955| 0.919| 447| 483| 0.925 |
| `⏎` | 0.821| 0.914| 0.646| 0.865| 0.723| 70| 99| 0.707 |
| `"` | 0.733| 0.926| 0.829| 0.818| 0.778| 68| 76| 0.895 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 26| 0.962 |
| `⏎␣⁺␣⁺` | 0.917| 0.579| 0.297| 0.710| 0.449| 19| 37| 0.514 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 33| 0.182 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 9| 0.111 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `weighted avg` | 0.930| 0.947| 0.854| 0.937| 0.876| 1494| 1657| 0.902 |
| `micro avg` | 0.947| 0.947| 0.854| 0.947| 0.898| 1494| 1657| 0.902 |
| `macro avg` | 0.439| 0.438| 0.362| 0.433| 0.383| 1494| 1657| 0.902 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|34 |841 |16 |0 |0 |1 |0 |0 |0 |0 |0 |
|36 |4 |436 |7 |0 |0 |0 |0 |0 |0 |0 |
|29 |0 |6 |64 |0 |0 |0 |0 |0 |0 |0 |
|8 |5 |0 |0 |63 |0 |0 |0 |0 |0 |0 |
|18 |0 |7 |1 |0 |11 |0 |0 |0 |0 |0 |
|27 |0 |1 |5 |0 |0 |0 |0 |0 |0 |0 |
|8 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |2 |0 |0 |23 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8181818181818182, "precision": 0.7325581395348837, "recall": 0.9264705882352942, "support": 68}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "macro avg": {"f1-score": 0.43314429330670867, "precision": 0.43924491461888715, "recall": 0.43752816500099734, "support": 1494}, "micro avg": {"f1-score": 0.9471218206157965, "precision": 0.9471218206157965, "recall": 0.9471218206157965, "support": 1494}, "weighted avg": {"f1-score": 0.9374419463793509, "precision": 0.9302625359892028, "recall": 0.9471218206157965, "support": 1494}, "\u2205": {"f1-score": 0.983625730994152, "precision": 0.9870892018779343, "recall": 0.9801864801864801, "support": 858}, "\u23ce": {"f1-score": 0.8648648648648648, "precision": 0.8205128205128205, "recall": 0.9142857142857143, "support": 70}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7096774193548387, "precision": 0.9166666666666666, "recall": 0.5789473684210527, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.955093099671413, "precision": 0.9356223175965666, "recall": 0.9753914988814317, "support": 447}},
  "cl_report_full": {"\"": {"f1-score": 0.7777777777777778, "precision": 0.7325581395348837, "recall": 0.8289473684210527, "support": 76}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "macro avg": {"f1-score": 0.38332327126646765, "precision": 0.43924491461888715, "recall": 0.36182259356777835, "support": 1657}, "micro avg": {"f1-score": 0.8981275785464933, "precision": 0.9471218206157965, "recall": 0.8539529269764635, "support": 1657}, "weighted avg": {"f1-score": 0.8759299298934118, "precision": 0.9071882932868716, "recall": 0.8539529269764635, "support": 1657}, "\u2205": {"f1-score": 0.9644495412844036, "precision": 0.9870892018779343, "recall": 0.9428251121076233, "support": 892}, "\u23ce": {"f1-score": 0.7231638418079097, "precision": 0.8205128205128205, "recall": 0.6464646464646465, "support": 99}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.4489795918367347, "precision": 0.9166666666666666, "recall": 0.2972972972972973, "support": 37}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9188619599578505, "precision": 0.9356223175965666, "recall": 0.9026915113871635, "support": 483}},
  "ppcr": 0.9016294508147255
}
```
</details>