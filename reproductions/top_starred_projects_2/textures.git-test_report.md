# Test report for javascript / file:///tmp/top-repos-quality-repos-u2h6bxfm/textures.git HEAD bfde14ecf2c051c499ae25eac5f9eb5ed60a4eb2

### Classification report

PPCR: 0.916

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.806| 0.958| 0.904| 0.875| 0.852| 450| 477| 0.943 |
| `␣` | 0.878| 0.711| 0.688| 0.785| 0.771| 304| 314| 0.968 |
| `'` | 0.800| 1.000| 1.000| 0.889| 0.889| 68| 68| 1.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 32| 0.906 |
| `⏎` | 0.125| 0.056| 0.032| 0.077| 0.051| 18| 31| 0.581 |
| `⏎⇥⁻` | 1.000| 0.882| 0.600| 0.938| 0.750| 17| 25| 0.680 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 23| 0.130 |
| `micro avg` | 0.822| 0.822| 0.754| 0.822| 0.786| 889| 970| 0.916 |
| `macro avg` | 0.516| 0.515| 0.461| 0.509| 0.473| 889| 970| 0.916 |
| `weighted avg` | 0.791| 0.822| 0.754| 0.799| 0.752| 889| 970| 0.916 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|27 |431 |19 |0 |0 |0 |0 |0 |
|10 |64 |216 |17 |7 |0 |0 |0 |
|0 |0 |0 |68 |0 |0 |0 |0 |
|13 |17 |0 |0 |1 |0 |0 |0 |
|3 |18 |11 |0 |0 |0 |0 |0 |
|8 |2 |0 |0 |0 |0 |15 |0 |
|20 |3 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.888888888888889, "precision": 0.8, "recall": 1.0, "support": 68}, "macro avg": {"f1-score": 0.5091276306885444, "precision": 0.5155223224461885, "recall": 0.515173227185611, "support": 889}, "micro avg": {"f1-score": 0.8222722159730033, "precision": 0.8222722159730034, "recall": 0.8222722159730034, "support": 889}, "weighted avg": {"f1-score": 0.799045948533438, "precision": 0.7908888568664499, "recall": 0.8222722159730034, "support": 889}, "\u2205": {"f1-score": 0.8751269035532995, "precision": 0.805607476635514, "recall": 0.9577777777777777, "support": 450}, "\u23ce": {"f1-score": 0.07692307692307691, "precision": 0.125, "recall": 0.05555555555555555, "support": 18}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u21e5\u207b": {"f1-score": 0.9375, "precision": 1.0, "recall": 0.8823529411764706, "support": 17}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.7854545454545453, "precision": 0.8780487804878049, "recall": 0.7105263157894737, "support": 304}},
  "cl_report_full": {"\u0027": {"f1-score": 0.888888888888889, "precision": 0.8, "recall": 1.0, "support": 68}, "macro avg": {"f1-score": 0.47333973824657055, "precision": 0.5155223224461885, "recall": 0.4605314421411277, "support": 970}, "micro avg": {"f1-score": 0.7864443249058634, "precision": 0.8222722159730034, "recall": 0.7536082474226804, "support": 970}, "weighted avg": {"f1-score": 0.7518671942629808, "precision": 0.7662444159054752, "recall": 0.7536082474226804, "support": 970}, "\u2205": {"f1-score": 0.8517786561264822, "precision": 0.805607476635514, "recall": 0.9035639412997903, "support": 477}, "\u23ce": {"f1-score": 0.05128205128205128, "precision": 0.125, "recall": 0.03225806451612903, "support": 31}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u21e5\u207b": {"f1-score": 0.7499999999999999, "precision": 1.0, "recall": 0.6, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.7714285714285715, "precision": 0.8780487804878049, "recall": 0.6878980891719745, "support": 314}},
  "ppcr": 0.9164948453608247
}
```
</details>
