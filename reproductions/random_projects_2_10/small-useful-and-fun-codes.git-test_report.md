# Test report for javascript / file:///tmp/top-repos-quality-repos-f2fbyuei/small-useful-and-fun-codes.git HEAD 4b69d0c4c85ae59e03c342bd2119ad7befece072

### Classification report

PPCR: 0.648

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.930| 0.987| 0.839| 0.958| 0.882| 1505| 1771| 0.850 |
| `␣` | 0.956| 0.898| 0.497| 0.926| 0.654| 635| 1148| 0.553 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 53| 70| 0.757 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 196| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 106| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 94| 0.000 |
| `macro avg` | 0.314| 0.314| 0.223| 0.314| 0.256| 2193| 3385| 0.648 |
| `weighted avg` | 0.916| 0.938| 0.607| 0.926| 0.683| 2193| 3385| 0.648 |
| `micro avg` | 0.938| 0.938| 0.607| 0.938| 0.737| 2193| 3385| 0.648 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|266 |1486 |19 |0 |0 |0 |0 |
|513 |65 |570 |0 |0 |0 |0 |
|196 |0 |0 |0 |0 |0 |0 |
|17 |46 |7 |0 |0 |0 |0 |
|106 |0 |0 |0 |0 |0 |0 |
|94 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "macro avg": {"f1-score": 0.31402798575310115, "precision": 0.3144784194077542, "recall": 0.3141688684263304, "support": 2193}, "micro avg": {"f1-score": 0.9375284997720018, "precision": 0.9375284997720018, "recall": 0.9375284997720018, "support": 2193}, "weighted avg": {"f1-score": 0.9256663372352208, "precision": 0.9155007512021198, "recall": 0.9375284997720018, "support": 2193}, "\u2205": {"f1-score": 0.9580915538362347, "precision": 0.9304946775203506, "recall": 0.987375415282392, "support": 1505}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.926076360682372, "precision": 0.9563758389261745, "recall": 0.8976377952755905, "support": 635}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 70}, "macro avg": {"f1-score": 0.2560154212701664, "precision": 0.3144784194077542, "recall": 0.22259827482521014, "support": 3385}, "micro avg": {"f1-score": 0.7371817855862317, "precision": 0.9375284997720018, "recall": 0.6073855243722305, "support": 3385}, "weighted avg": {"f1-score": 0.6833629624474676, "precision": 0.8111744570090957, "recall": 0.6073855243722305, "support": 3385}, "\u2205": {"f1-score": 0.8824228028503562, "precision": 0.9304946775203506, "recall": 0.8390739695087521, "support": 1771}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 196}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u2423": {"f1-score": 0.6536697247706421, "precision": 0.9563758389261745, "recall": 0.4965156794425087, "support": 1148}},
  "ppcr": 0.6478581979320531
}
```
</details>
