# Test report for javascript / file:///tmp/top-repos-quality-repos-zy_h3jma/sdow.git HEAD a2b07edf2c844b1b67c8080a6f286105155c45db

### Classification report

PPCR: 0.911

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.809| 0.980| 0.967| 0.886| 0.881| 1198| 1214| 0.987 |
| `␣` | 0.895| 0.815| 0.759| 0.853| 0.821| 502| 539| 0.931 |
| `'` | 1.000| 0.671| 0.671| 0.803| 0.803| 158| 158| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 63| 110| 0.573 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 69| 0.812 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 59| 0.441 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 58| 0.121 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 4| 1.000 |
| `weighted avg` | 0.783| 0.839| 0.764| 0.803| 0.741| 2014| 2211| 0.911 |
| `macro avg` | 0.338| 0.308| 0.300| 0.318| 0.313| 2014| 2211| 0.911 |
| `micro avg` | 0.839| 0.839| 0.764| 0.839| 0.800| 2014| 2211| 0.911 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|16 |1174 |24 |0 |0 |0 |0 |0 |0 |
|37 |93 |409 |0 |0 |0 |0 |0 |0 |
|0 |49 |3 |106 |0 |0 |0 |0 |0 |
|47 |49 |14 |0 |0 |0 |0 |0 |0 |
|51 |1 |6 |0 |0 |0 |0 |0 |0 |
|13 |55 |1 |0 |0 |0 |0 |0 |0 |
|33 |26 |0 |0 |0 |0 |0 |0 |0 |
|0 |4 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u0027": {"f1-score": 0.8030303030303031, "precision": 1.0, "recall": 0.6708860759493671, "support": 158}, "macro avg": {"f1-score": 0.31779679557917734, "precision": 0.33800804395067463, "recall": 0.30819921535303807, "support": 2014}, "micro avg": {"f1-score": 0.8386295928500497, "precision": 0.8386295928500497, "recall": 0.8386295928500497, "support": 2014}, "weighted avg": {"f1-score": 0.8028523183173822, "precision": 0.7828063246584981, "recall": 0.8386295928500497, "support": 2014}, "\u2205": {"f1-score": 0.8863722159305398, "precision": 0.8090971743625086, "recall": 0.9799666110183639, "support": 1198}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u2423": {"f1-score": 0.8529718456725757, "precision": 0.8949671772428884, "recall": 0.8147410358565738, "support": 502}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u0027": {"f1-score": 0.8030303030303031, "precision": 1.0, "recall": 0.6708860759493671, "support": 158}, "macro avg": {"f1-score": 0.31317076253162063, "precision": 0.33800804395067463, "recall": 0.29959372034312975, "support": 2211}, "micro avg": {"f1-score": 0.7995266272189349, "precision": 0.8386295928500497, "recall": 0.7639077340569878, "support": 2211}, "weighted avg": {"f1-score": 0.7413600071585623, "precision": 0.7338902208095895, "recall": 0.7639077340569878, "support": 2211}, "\u2205": {"f1-score": 0.8810506566604127, "precision": 0.8090971743625086, "recall": 0.9670510708401977, "support": 1214}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 110}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u2423": {"f1-score": 0.8212851405622491, "precision": 0.8949671772428884, "recall": 0.7588126159554731, "support": 539}},
  "ppcr": 0.9109000452284034
}
```
</details>
