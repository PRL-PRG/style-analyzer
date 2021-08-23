# Test report for javascript / file:///tmp/top-repos-quality-repos-0bw21y39/fardelcms.git HEAD f53cd614e912700f27f70494db17f81aaaeafb06

### Classification report

PPCR: 0.447

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.886| 1.000| 0.758| 0.939| 0.817| 729| 962| 0.758 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 60| 134| 0.448 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 595| 0.057 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 65| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 85| 0.000 |
| `micro avg` | 0.886| 0.886| 0.396| 0.886| 0.547| 823| 1841| 0.447 |
| `weighted avg` | 0.785| 0.886| 0.396| 0.832| 0.427| 823| 1841| 0.447 |
| `macro avg` | 0.127| 0.143| 0.108| 0.134| 0.117| 823| 1841| 0.447 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⇥⁻| ⏎⇥⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|233 |729 |0 |0 |0 |0 |
|561 |34 |0 |0 |0 |0 |
|65 |0 |0 |0 |0 |0 |
|74 |60 |0 |0 |0 |0 |
|85 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 60}, "macro avg": {"f1-score": 0.13420471281296023, "precision": 0.1265405311577851, "recall": 0.14285714285714285, "support": 823}, "micro avg": {"f1-score": 0.8857837181044959, "precision": 0.8857837181044957, "recall": 0.8857837181044957, "support": 823}, "weighted avg": {"f1-score": 0.8321344465182698, "precision": 0.7846127952590247, "recall": 0.8857837181044957, "support": 823}, "\u2205": {"f1-score": 0.9394329896907216, "precision": 0.8857837181044957, "recall": 1.0, "support": 729}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 134}, "macro avg": {"f1-score": 0.11668667466986796, "precision": 0.1265405311577851, "recall": 0.10825660825660825, "support": 1841}, "micro avg": {"f1-score": 0.5472972972972973, "precision": 0.8857837181044957, "recall": 0.3959804454101032, "support": 1841}, "weighted avg": {"f1-score": 0.4268158974616463, "precision": 0.4628592812691607, "recall": 0.3959804454101032, "support": 1841}, "\u2205": {"f1-score": 0.8168067226890757, "precision": 0.8857837181044957, "recall": 0.7577962577962578, "support": 962}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 85}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 595}},
  "ppcr": 0.44703965236284626
}
```
</details>
