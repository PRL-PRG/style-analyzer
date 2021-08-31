# Test report for javascript / file:///tmp/top-repos-quality-repos-sl35_1xu/emitter.git HEAD 6b64b870f651dfd918153813fc124b988d000597

### Classification report

PPCR: 0.427

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.928| 0.895| 0.304| 0.911| 0.458| 86| 253| 0.340 |
| `'` | 0.783| 0.900| 0.871| 0.837| 0.824| 60| 62| 0.968 |
| `␣` | 0.977| 0.875| 0.477| 0.923| 0.641| 48| 88| 0.545 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 34| 0.029 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 8| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `macro avg` | 0.448| 0.445| 0.275| 0.445| 0.321| 195| 457| 0.427 |
| `weighted avg` | 0.890| 0.887| 0.379| 0.887| 0.489| 195| 457| 0.427 |
| `micro avg` | 0.887| 0.887| 0.379| 0.887| 0.531| 195| 457| 0.427 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|167 |77 |0 |9 |0 |0 |0 |
|40 |0 |42 |6 |0 |0 |0 |
|2 |6 |0 |54 |0 |0 |0 |
|33 |0 |1 |0 |0 |0 |0 |
|8 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8372093023255814, "precision": 0.782608695652174, "recall": 0.9, "support": 60}, "macro avg": {"f1-score": 0.44525480482546675, "precision": 0.44784395417869655, "recall": 0.44505813953488377, "support": 195}, "micro avg": {"f1-score": 0.8871794871794871, "precision": 0.8871794871794871, "recall": 0.8871794871794871, "support": 195}, "weighted avg": {"f1-score": 0.886703150525911, "precision": 0.8903762830742742, "recall": 0.8871794871794871, "support": 195}, "\u2205": {"f1-score": 0.9112426035502958, "precision": 0.927710843373494, "recall": 0.8953488372093024, "support": 86}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.923076923076923, "precision": 0.9767441860465116, "recall": 0.875, "support": 48}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8244274809160306, "precision": 0.782608695652174, "recall": 0.8709677419354839, "support": 62}, "macro avg": {"f1-score": 0.32066369804919426, "precision": 0.44784395417869655, "recall": 0.2754313825491946, "support": 457}, "micro avg": {"f1-score": 0.5306748466257669, "precision": 0.8871794871794871, "recall": 0.3785557986870897, "support": 457}, "weighted avg": {"f1-score": 0.489059776950017, "precision": 0.8078469822232424, "recall": 0.3785557986870897, "support": 457}, "\u2205": {"f1-score": 0.45833333333333337, "precision": 0.927710843373494, "recall": 0.30434782608695654, "support": 253}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.6412213740458016, "precision": 0.9767441860465116, "recall": 0.4772727272727273, "support": 88}},
  "ppcr": 0.42669584245076586
}
```
</details>
