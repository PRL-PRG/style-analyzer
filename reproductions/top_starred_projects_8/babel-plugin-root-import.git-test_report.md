# Test report for javascript / file:///tmp/top-repos-quality-repos-z2pbtfn7/babel-plugin-root-import.git HEAD 4ad4ac78df8168541f195db847318cc731c3bd53

### Classification report

PPCR: 0.354

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 0.977| 0.336| 0.985| 0.502| 131| 381| 0.344 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 74| 148| 0.500 |
| `␣` | 1.000| 1.000| 0.297| 1.000| 0.458| 44| 148| 0.297 |
| `⏎␣⁺␣⁺` | 1.000| 1.000| 0.679| 1.000| 0.809| 19| 28| 0.679 |
| `⏎␣⁻␣⁻` | 0.857| 0.947| 0.643| 0.900| 0.735| 19| 28| 0.679 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 62| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 16| 0.000 |
| `weighted avg` | 0.987| 0.986| 0.349| 0.986| 0.494| 287| 811| 0.354 |
| `macro avg` | 0.693| 0.703| 0.351| 0.698| 0.453| 287| 811| 0.354 |
| `micro avg` | 0.986| 0.986| 0.349| 0.986| 0.515| 287| 811| 0.354 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|250 |128 |0 |0 |0 |3 |0 |0 |
|104 |0 |44 |0 |0 |0 |0 |0 |
|74 |0 |0 |74 |0 |0 |0 |0 |
|9 |0 |0 |0 |19 |0 |0 |0 |
|9 |1 |0 |0 |0 |18 |0 |0 |
|62 |0 |0 |0 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 74}, "macro avg": {"f1-score": 0.6978021978021979, "precision": 0.6927701313083372, "recall": 0.7034953796705504, "support": 287}, "micro avg": {"f1-score": 0.9860627177700348, "precision": 0.9860627177700348, "recall": 0.9860627177700348, "support": 287}, "weighted avg": {"f1-score": 0.9863575448941303, "precision": 0.9870042174555586, "recall": 0.9860627177700348, "support": 287}, "\u2205": {"f1-score": 0.9846153846153846, "precision": 0.9922480620155039, "recall": 0.9770992366412213, "support": 131}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9, "precision": 0.8571428571428571, "recall": 0.9473684210526315, "support": 19}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 44}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 148}, "macro avg": {"f1-score": 0.45288075716608833, "precision": 0.6927701313083372, "recall": 0.35066912485360185, "support": 811}, "micro avg": {"f1-score": 0.5154826958105647, "precision": 0.9860627177700348, "recall": 0.34895191122071517, "support": 811}, "weighted avg": {"f1-score": 0.4943980089609105, "precision": 0.8952484730307114, "recall": 0.34895191122071517, "support": 811}, "\u2205": {"f1-score": 0.5019607843137255, "precision": 0.9922480620155039, "recall": 0.3359580052493438, "support": 381}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8085106382978724, "precision": 1.0, "recall": 0.6785714285714286, "support": 28}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7346938775510204, "precision": 0.8571428571428571, "recall": 0.6428571428571429, "support": 28}, "\u2423": {"f1-score": 0.4583333333333333, "precision": 1.0, "recall": 0.2972972972972973, "support": 148}},
  "ppcr": 0.35388409371146734
}
```
</details>
