# Test report for javascript / file:///tmp/top-repos-quality-repos-2j1qajhf/brickpop.git HEAD ff32b9b01266029e9d4237df5188fe4aec1b1a1e

### Classification report

PPCR: 0.755

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.828| 0.966| 0.896| 0.892| 0.861| 179| 193| 0.927 |
| `␣` | 0.845| 1.000| 0.652| 0.916| 0.736| 60| 92| 0.652 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 42| 0.952 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 6| 0.167 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 20| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `macro avg` | 0.239| 0.281| 0.221| 0.258| 0.228| 280| 371| 0.755 |
| `micro avg` | 0.832| 0.832| 0.628| 0.832| 0.716| 280| 371| 0.755 |
| `weighted avg` | 0.710| 0.832| 0.628| 0.766| 0.630| 280| 371| 0.755 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|14 |173 |6 |0 |0 |0 |0 |0 |
|32 |0 |60 |0 |0 |0 |0 |0 |
|20 |0 |0 |0 |0 |0 |0 |0 |
|2 |36 |4 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |0 |0 |
|5 |0 |1 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "macro avg": {"f1-score": 0.25825473023867607, "precision": 0.23897451695820857, "recall": 0.28092577813248204, "support": 280}, "micro avg": {"f1-score": 0.8321428571428572, "precision": 0.8321428571428572, "recall": 0.8321428571428572, "support": 280}, "weighted avg": {"f1-score": 0.7663769407188388, "precision": 0.7102560338105186, "recall": 0.8321428571428572, "support": 280}, "\u2205": {"f1-score": 0.8917525773195876, "precision": 0.8277511961722488, "recall": 0.9664804469273743, "support": 179}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.916030534351145, "precision": 0.8450704225352113, "recall": 1.0, "support": 60}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "macro avg": {"f1-score": 0.22812754806162003, "precision": 0.23897451695820857, "recall": 0.22122099571975667, "support": 371}, "micro avg": {"f1-score": 0.7158218125960061, "precision": 0.8321428571428572, "recall": 0.628032345013477, "support": 371}, "weighted avg": {"f1-score": 0.6303085962544198, "precision": 0.6401683550794702, "recall": 0.628032345013477, "support": 371}, "\u2205": {"f1-score": 0.8606965174129353, "precision": 0.8277511961722488, "recall": 0.8963730569948186, "support": 193}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.7361963190184049, "precision": 0.8450704225352113, "recall": 0.6521739130434783, "support": 92}},
  "ppcr": 0.7547169811320755
}
```
</details>
