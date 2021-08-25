# Test report for javascript / file:///tmp/top-repos-quality-repos-mewp_bos/tecnologie-per-iot.git HEAD 17a93fceceefa94f249d80f047eaeabed377410d

### Classification report

PPCR: 0.680

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.983| 0.898| 0.969| 0.926| 1843| 2017| 0.914 |
| `␣` | 0.925| 0.897| 0.544| 0.911| 0.685| 660| 1088| 0.607 |
| `"` | 1.000| 0.956| 0.500| 0.978| 0.667| 319| 610| 0.523 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 325| 0.058 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 140| 0.000 |
| `micro avg` | 0.954| 0.954| 0.648| 0.954| 0.772| 2841| 4180| 0.680 |
| `weighted avg` | 0.947| 0.954| 0.648| 0.950| 0.723| 2841| 4180| 0.680 |
| `macro avg` | 0.412| 0.405| 0.277| 0.408| 0.325| 2841| 4180| 0.680 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|174 |1812 |31 |0 |0 |0 |
|428 |68 |592 |0 |0 |0 |
|306 |3 |16 |0 |0 |0 |
|291 |13 |1 |0 |305 |0 |
|140 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9775641025641025, "precision": 1.0, "recall": 0.9561128526645768, "support": 319}, "macro avg": {"f1-score": 0.40822520663788386, "precision": 0.4115280289330922, "recall": 0.40518030687357304, "support": 2841}, "micro avg": {"f1-score": 0.9535374868004224, "precision": 0.9535374868004224, "recall": 0.9535374868004224, "support": 2841}, "weighted avg": {"f1-score": 0.9501111223258172, "precision": 0.9471482229024368, "recall": 0.9535374868004224, "support": 2841}, "\u2205": {"f1-score": 0.9692431131318534, "precision": 0.9556962025316456, "recall": 0.9831795984807379, "support": 1843}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9107692307692308, "precision": 0.925, "recall": 0.896969696969697, "support": 660}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 610}, "macro avg": {"f1-score": 0.3254279250956991, "precision": 0.4115280289330922, "recall": 0.2774973648358699, "support": 4180}, "micro avg": {"f1-score": 0.7716849451645065, "precision": 0.9535374868004224, "recall": 0.6480861244019138, "support": 4180}, "weighted avg": {"f1-score": 0.722531061576349, "precision": 0.8478562776330931, "recall": 0.6480861244019138, "support": 4180}, "\u2205": {"f1-score": 0.9261436238180424, "precision": 0.9556962025316456, "recall": 0.8983639067922657, "support": 2017}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 325}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u2423": {"f1-score": 0.6851851851851851, "precision": 0.925, "recall": 0.5441176470588235, "support": 1088}},
  "ppcr": 0.679665071770335
}
```
</details>
