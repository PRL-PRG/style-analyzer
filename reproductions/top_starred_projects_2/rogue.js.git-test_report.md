# Test report for javascript / file:///tmp/top-repos-quality-repos-svlqhn4t/rogue.js.git HEAD 7caf7d38d5bcf9bb97f10923ba8766aace049616

### Classification report

PPCR: 0.261

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.448| 1.000| 0.619| 143| 319| 0.448 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 32| 64| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 36| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 16| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 15| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 221| 0.000 |
| `micro avg` | 1.000| 1.000| 0.261| 1.000| 0.414| 175| 671| 0.261 |
| `macro avg` | 0.333| 0.333| 0.158| 0.333| 0.214| 175| 671| 0.261 |
| `weighted avg` | 1.000| 1.000| 0.261| 1.000| 0.358| 175| 671| 0.261 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|176 |143 |0 |0 |0 |0 |0 |
|221 |0 |0 |0 |0 |0 |0 |
|32 |0 |0 |32 |0 |0 |0 |
|36 |0 |0 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |0 |0 |
|15 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 32}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 175}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 175}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 175}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 143}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 64}, "macro avg": {"f1-score": 0.21428571428571427, "precision": 0.3333333333333333, "recall": 0.15804597701149425, "support": 671}, "micro avg": {"f1-score": 0.41371158392434987, "precision": 1.0, "recall": 0.2608047690014903, "support": 671}, "weighted avg": {"f1-score": 0.35788801362571854, "precision": 0.5707898658718331, "recall": 0.2608047690014903, "support": 671}, "\u2205": {"f1-score": 0.6190476190476191, "precision": 1.0, "recall": 0.4482758620689655, "support": 319}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 221}},
  "ppcr": 0.2608047690014903
}
```
</details>
