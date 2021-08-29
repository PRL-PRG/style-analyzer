# Train report for javascript / file:///tmp/top-repos-quality-repos-lyvhbr25/adele.git HEAD fd31fe1de9a7a28e453beee8ac54b5362ba9c878

### Classification report

PPCR: 0.338

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.460| 1.000| 0.630| 376| 817| 0.460 |
| `␣` | 1.000| 1.000| 0.527| 1.000| 0.690| 205| 389| 0.527 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 174| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 78| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 140| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 61| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 61| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.338| 1.000| 0.456| 581| 1720| 0.338 |
| `micro avg` | 1.000| 1.000| 0.338| 1.000| 0.505| 581| 1720| 0.338 |
| `macro avg` | 0.286| 0.286| 0.141| 0.286| 0.189| 581| 1720| 0.338 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|441 |376 |0 |0 |0 |0 |0 |0 |
|184 |0 |205 |0 |0 |0 |0 |0 |
|174 |0 |0 |0 |0 |0 |0 |0 |
|78 |0 |0 |0 |0 |0 |0 |0 |
|140 |0 |0 |0 |0 |0 |0 |0 |
|61 |0 |0 |0 |0 |0 |0 |0 |
|61 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2857142857142857, "precision": 0.2857142857142857, "recall": 0.2857142857142857, "support": 581}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 581}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 581}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 376}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 205}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 78}, "macro avg": {"f1-score": 0.1886541945217553, "precision": 0.2857142857142857, "recall": 0.14103037230788454, "support": 1720}, "micro avg": {"f1-score": 0.5049978270317252, "precision": 1.0, "recall": 0.3377906976744186, "support": 1720}, "weighted avg": {"f1-score": 0.4555188738657227, "precision": 0.7011627906976744, "recall": 0.3377906976744186, "support": 1720}, "\u2205": {"f1-score": 0.6303436714165969, "precision": 1.0, "recall": 0.4602203182374541, "support": 817}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 174}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u2423": {"f1-score": 0.6902356902356902, "precision": 1.0, "recall": 0.5269922879177378, "support": 389}},
  "ppcr": 0.3377906976744186
}
```
</details>
