# Test report for javascript / file:///tmp/top-repos-quality-repos-p_foqebb/skinnefy.git HEAD c50d891bdd5dd30330ba389e69f30ebd74cf0522

### Classification report

PPCR: 0.530

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.777| 1.000| 0.873| 0.874| 0.822| 542| 621| 0.873 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 72| 361| 0.199 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 39| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 97| 0.381 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 40| 0.200 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 128| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 32| 0.000 |
| `macro avg` | 0.111| 0.143| 0.125| 0.125| 0.117| 698| 1318| 0.530 |
| `weighted avg` | 0.603| 0.777| 0.411| 0.679| 0.387| 698| 1318| 0.530 |
| `micro avg` | 0.777| 0.777| 0.411| 0.777| 0.538| 698| 1318| 0.530 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|79 |542 |0 |0 |0 |0 |0 |0 |
|289 |72 |0 |0 |0 |0 |0 |0 |
|60 |37 |0 |0 |0 |0 |0 |0 |
|128 |0 |0 |0 |0 |0 |0 |0 |
|0 |39 |0 |0 |0 |0 |0 |0 |
|32 |0 |0 |0 |0 |0 |0 |0 |
|32 |8 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.12488479262672811, "precision": 0.11092918542775276, "recall": 0.14285714285714285, "support": 698}, "micro avg": {"f1-score": 0.7765042979942693, "precision": 0.7765042979942693, "recall": 0.7765042979942693, "support": 698}, "weighted avg": {"f1-score": 0.678815047601442, "precision": 0.602958924803573, "recall": 0.7765042979942693, "support": 698}, "\u2205": {"f1-score": 0.8741935483870967, "precision": 0.7765042979942693, "recall": 1.0, "support": 542}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "macro avg": {"f1-score": 0.11740496046788694, "precision": 0.11092918542775276, "recall": 0.12468368990108121, "support": 1318}, "micro avg": {"f1-score": 0.5376984126984127, "precision": 0.7765042979942693, "recall": 0.41122913505311076, "support": 1318}, "weighted avg": {"f1-score": 0.3872225820591081, "precision": 0.36586431642977335, "recall": 0.41122913505311076, "support": 1318}, "\u2205": {"f1-score": 0.8218347232752086, "precision": 0.7765042979942693, "recall": 0.8727858293075684, "support": 621}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 361}},
  "ppcr": 0.5295902883156297
}
```
</details>
