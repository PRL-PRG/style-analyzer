# Test report for javascript / file:///tmp/top-repos-quality-repos-m2i4ish9/colesnyder.github.io.git HEAD 64e939b7b93e9b5710650a620c219fc0745de868

### Classification report

PPCR: 0.936

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.990| 0.974| 0.992| 0.983| 11138| 11327| 0.983 |
| `␣` | 0.948| 0.975| 0.905| 0.962| 0.926| 5915| 6378| 0.927 |
| `'` | 0.992| 0.979| 0.922| 0.986| 0.955| 875| 930| 0.941 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.924| 0.874| 0.839| 0.899| 0.879| 826| 861| 0.959 |
| `⏎` | 0.866| 0.829| 0.568| 0.847| 0.686| 783| 1143| 0.685 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.926| 0.977| 0.959| 0.951| 0.942| 743| 757| 0.982 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 58| 292| 0.199 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 61| 0.426 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 4| 0.500 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.970| 0.970| 0.908| 0.970| 0.938| 20366| 21753| 0.936 |
| `weighted avg` | 0.966| 0.970| 0.908| 0.968| 0.928| 20366| 21753| 0.936 |
| `macro avg` | 0.514| 0.511| 0.470| 0.512| 0.488| 20366| 21753| 0.936 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|189 |11030 |96 |0 |6 |4 |2 |0 |0 |0 |
|463 |39 |5769 |5 |10 |51 |41 |0 |0 |0 |
|55 |15 |1 |857 |0 |1 |1 |0 |0 |0 |
|360 |1 |131 |0 |649 |2 |0 |0 |0 |0 |
|35 |10 |83 |0 |11 |722 |0 |0 |0 |0 |
|14 |10 |3 |0 |4 |0 |726 |0 |0 |0 |
|234 |0 |0 |0 |57 |1 |0 |0 |0 |0 |
|2 |0 |0 |2 |0 |0 |0 |0 |0 |0 |
|35 |0 |0 |0 |12 |0 |14 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u0027": {"f1-score": 0.9856239217941346, "precision": 0.9918981481481481, "recall": 0.9794285714285714, "support": 875}, "macro avg": {"f1-score": 0.5123425658900728, "precision": 0.5136809138508119, "recall": 0.5113749243809224, "support": 20366}, "micro avg": {"f1-score": 0.9699008150839634, "precision": 0.9699008150839634, "recall": 0.9699008150839634, "support": 20366}, "weighted avg": {"f1-score": 0.9677469206213335, "precision": 0.9658477036951314, "recall": 0.9699008150839634, "support": 20366}, "\u2205": {"f1-score": 0.9917726925324821, "precision": 0.9932462854570013, "recall": 0.990303465613216, "support": 11138}, "\u23ce": {"f1-score": 0.8472584856396866, "precision": 0.8664886515353805, "recall": 0.8288633461047255, "support": 783}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8985687616677038, "precision": 0.9244558258642765, "recall": 0.87409200968523, "support": 826}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9508840864440078, "precision": 0.9260204081632653, "recall": 0.9771197846567967, "support": 743}, "\u2423": {"f1-score": 0.9616602767127855, "precision": 0.9483807331908598, "recall": 0.975316990701606, "support": 5915}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u0027": {"f1-score": 0.955406911928651, "precision": 0.9918981481481481, "recall": 0.921505376344086, "support": 930}, "macro avg": {"f1-score": 0.4884054101734682, "precision": 0.5136809138508119, "recall": 0.4695648253872376, "support": 21753}, "micro avg": {"f1-score": 0.9379614900638666, "precision": 0.9699008150839634, "recall": 0.9080586585758286, "support": 21753}, "weighted avg": {"f1-score": 0.9280499077169723, "precision": 0.9520106979816213, "recall": 0.9080586585758286, "support": 21753}, "\u2205": {"f1-score": 0.9834165477888729, "precision": 0.9932462854570013, "recall": 0.9737794649951443, "support": 11327}, "\u23ce": {"f1-score": 0.6860465116279071, "precision": 0.8664886515353805, "recall": 0.5678040244969379, "support": 1143}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 292}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.879415347137637, "precision": 0.9244558258642765, "recall": 0.8385598141695703, "support": 861}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9422452952628164, "precision": 0.9260204081632653, "recall": 0.9590488771466315, "support": 757}, "\u2423": {"f1-score": 0.9259288981622663, "precision": 0.9483807331908598, "recall": 0.9045155221072436, "support": 6378}},
  "ppcr": 0.9362386797223371
}
```
</details>