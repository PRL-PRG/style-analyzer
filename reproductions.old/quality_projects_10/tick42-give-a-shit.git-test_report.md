# Test report for javascript / file:///tmp/top-repos-quality-repos-sp_21c7_/tick42-give-a-shit.git HEAD c84bf2fb0aca5dc6f99939ba40b374228868e9d1

### Classification report

PPCR: 0.547

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.916| 1.000| 0.773| 0.956| 0.838| 197| 255| 0.773 |
| `␣` | 1.000| 0.962| 0.238| 0.980| 0.385| 26| 105| 0.248 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 22| 0.364 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 14| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 25| 0.040 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 18| 0.056 |
| `macro avg` | 0.319| 0.327| 0.168| 0.323| 0.204| 240| 439| 0.547 |
| `micro avg` | 0.925| 0.925| 0.506| 0.925| 0.654| 240| 439| 0.547 |
| `weighted avg` | 0.860| 0.925| 0.506| 0.891| 0.579| 240| 439| 0.547 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|58 |197 |0 |0 |0 |0 |0 |
|79 |1 |25 |0 |0 |0 |0 |
|24 |1 |0 |0 |0 |0 |0 |
|14 |8 |0 |0 |0 |0 |0 |
|17 |1 |0 |0 |0 |0 |0 |
|7 |7 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "macro avg": {"f1-score": 0.3227838060790659, "precision": 0.31937984496124033, "recall": 0.3269230769230769, "support": 240}, "micro avg": {"f1-score": 0.925, "precision": 0.925, "recall": 0.925, "support": 240}, "weighted avg": {"f1-score": 0.8911808331746939, "precision": 0.8604457364341086, "recall": 0.925, "support": 240}, "\u2205": {"f1-score": 0.9563106796116505, "precision": 0.9162790697674419, "recall": 1.0, "support": 197}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.9803921568627451, "precision": 1.0, "recall": 0.9615384615384616, "support": 26}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "macro avg": {"f1-score": 0.2038188761593017, "precision": 0.31937984496124033, "recall": 0.1684407096171802, "support": 439}, "micro avg": {"f1-score": 0.6539027982326953, "precision": 0.925, "recall": 0.5056947608200456, "support": 439}, "weighted avg": {"f1-score": 0.5789306898210111, "precision": 0.7714149494093341, "recall": 0.5056947608200456, "support": 439}, "\u2205": {"f1-score": 0.8382978723404256, "precision": 0.9162790697674419, "recall": 0.7725490196078432, "support": 255}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.3846153846153846, "precision": 1.0, "recall": 0.23809523809523808, "support": 105}},
  "ppcr": 0.5466970387243736
}
```
</details>
