# Test report for javascript / file:///tmp/top-repos-quality-repos-gnqspq34/scheduler.git HEAD 1f7eb76b892bdd1132e468bc2f8280c66377e62f

### Classification report

PPCR: 0.509

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.966| 0.922| 0.555| 0.943| 0.705| 850| 1413| 0.602 |
| `␣` | 0.854| 0.925| 0.396| 0.888| 0.541| 279| 652| 0.428 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 103| 194| 0.531 |
| `⏎` | 1.000| 0.250| 0.007| 0.400| 0.015| 4| 135| 0.030 |
| `"` | 0.037| 1.000| 0.500| 0.071| 0.069| 3| 6| 0.500 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 33| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.858| 0.844| 0.430| 0.849| 0.555| 1239| 2433| 0.509 |
| `macro avg` | 0.286| 0.310| 0.146| 0.230| 0.133| 1239| 2433| 0.509 |
| `micro avg` | 0.844| 0.844| 0.430| 0.844| 0.570| 1239| 2433| 0.509 |

### Confusion matrix

|refusal|  ␣| ∅| ⏎| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎⏎⇥⁺| '| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|373 |258 |3 |0 |0 |9 |9 |0 |0 |
|563 |43 |784 |0 |0 |0 |23 |0 |0 |
|131 |1 |1 |1 |0 |1 |0 |0 |0 |
|3 |0 |0 |0 |3 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|33 |0 |0 |0 |0 |0 |0 |0 |0 |
|91 |0 |24 |0 |78 |0 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.07142857142857142, "precision": 0.037037037037037035, "recall": 1.0, "support": 3}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "macro avg": {"f1-score": 0.23029941322795047, "precision": 0.28568589141779366, "recall": 0.30970841239721697, "support": 1239}, "micro avg": {"f1-score": 0.844229217110573, "precision": 0.8442292171105731, "recall": 0.8442292171105731, "support": 1239}, "weighted avg": {"f1-score": 0.848689468668494, "precision": 0.8580724452469801, "recall": 0.8442292171105731, "support": 1239}, "\u2205": {"f1-score": 0.9434416365824307, "precision": 0.9655172413793104, "recall": 0.9223529411764706, "support": 850}, "\u23ce": {"f1-score": 0.4, "precision": 1.0, "recall": 0.25, "support": 4}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8881239242685025, "precision": 0.8543046357615894, "recall": 0.9247311827956989, "support": 279}},
  "cl_report_full": {"\"": {"f1-score": 0.06896551724137931, "precision": 0.037037037037037035, "recall": 0.5, "support": 6}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "macro avg": {"f1-score": 0.13292710038625702, "precision": 0.28568589141779366, "recall": 0.14579607703518455, "support": 2433}, "micro avg": {"f1-score": 0.5697167755991286, "precision": 0.8442292171105731, "recall": 0.4299219071105631, "support": 2433}, "weighted avg": {"f1-score": 0.5552080826794289, "precision": 0.845254708922213, "recall": 0.4299219071105631, "support": 2433}, "\u2205": {"f1-score": 0.7047191011235956, "precision": 0.9655172413793104, "recall": 0.5548478414720452, "support": 1413}, "\u23ce": {"f1-score": 0.014705882352941178, "precision": 1.0, "recall": 0.007407407407407408, "support": 135}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.5408805031446541, "precision": 0.8543046357615894, "recall": 0.39570552147239263, "support": 652}},
  "ppcr": 0.5092478421701603
}
```
</details>
