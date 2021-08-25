# Train report for javascript / file:///tmp/top-repos-quality-repos-g7bm6_qh/structurefinder.git HEAD eed4e45d323f7deb639935e28d425574af000962

### Classification report

PPCR: 0.139

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.998| 1.000| 0.304| 0.999| 0.466| 631| 2077| 0.304 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 123| 0.008 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1132| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 553| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 328| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 220| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 122| 0.000 |
| `macro avg` | 0.143| 0.143| 0.043| 0.143| 0.067| 632| 4555| 0.139 |
| `weighted avg` | 0.997| 0.998| 0.139| 0.998| 0.212| 632| 4555| 0.139 |
| `micro avg` | 0.998| 0.998| 0.139| 0.998| 0.243| 632| 4555| 0.139 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1446 |631 |0 |0 |0 |0 |0 |0 |
|1132 |0 |0 |0 |0 |0 |0 |0 |
|553 |0 |0 |0 |0 |0 |0 |0 |
|328 |0 |0 |0 |0 |0 |0 |0 |
|220 |0 |0 |0 |0 |0 |0 |0 |
|122 |1 |0 |0 |0 |0 |0 |0 |
|122 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| cgi_ui/static/strf_web.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1427440334803755, "precision": 0.14263110307414104, "recall": 0.14285714285714285, "support": 632}, "micro avg": {"f1-score": 0.9984177215189873, "precision": 0.9984177215189873, "recall": 0.9984177215189873, "support": 632}, "weighted avg": {"f1-score": 0.997627208675346, "precision": 0.9968379466431662, "recall": 0.9984177215189873, "support": 632}, "\u2205": {"f1-score": 0.9992082343626286, "precision": 0.9984177215189873, "recall": 1.0, "support": 631}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 220}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 553}, "macro avg": {"f1-score": 0.06655065126825924, "precision": 0.14263110307414104, "recall": 0.04340050897585803, "support": 4555}, "micro avg": {"f1-score": 0.24330055909003276, "precision": 0.9984177215189873, "recall": 0.1385290889132821, "support": 4555}, "weighted avg": {"f1-score": 0.21242149698994972, "precision": 0.4552609456849476, "recall": 0.1385290889132821, "support": 4555}, "\u2205": {"f1-score": 0.46585455887781463, "precision": 0.9984177215189873, "recall": 0.30380356283100624, "support": 2077}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 328}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1132}},
  "ppcr": 0.13874862788144895
}
```
</details>
