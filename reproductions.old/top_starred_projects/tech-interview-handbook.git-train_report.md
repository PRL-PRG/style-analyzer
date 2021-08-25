# Train report for javascript / file:///tmp/top-repos-quality-repos-0ypvuiim/tech-interview-handbook.git HEAD a5abb22e662530cccbad3932ca7cf93f8a45f7b1

### Classification report

PPCR: 0.430

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 0.993| 1.000| 0.996| 1942| 1956| 0.993 |
| `␣` | 0.995| 0.982| 0.409| 0.988| 0.579| 603| 1449| 0.416 |
| `⏎` | 0.925| 1.000| 0.503| 0.961| 0.652| 383| 761| 0.503 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 106| 0.189 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 179| 0.011 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 2266| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 151| 0.000 |
| `weighted avg` | 0.981| 0.988| 0.425| 0.985| 0.478| 2951| 6868| 0.430 |
| `micro avg` | 0.988| 0.988| 0.425| 0.988| 0.594| 2951| 6868| 0.430 |
| `macro avg` | 0.417| 0.426| 0.272| 0.421| 0.318| 2951| 6868| 0.430 |

### Confusion matrix

|refusal|  "| ␣| ∅| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|14 |1942 |0 |0 |0 |0 |0 |0 |
|846 |0 |592 |0 |11 |0 |0 |0 |
|2265 |0 |1 |0 |0 |0 |0 |0 |
|378 |0 |0 |0 |383 |0 |0 |0 |
|151 |0 |0 |0 |0 |0 |0 |0 |
|177 |0 |2 |0 |0 |0 |0 |0 |
|86 |0 |0 |0 |20 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| website/docusaurus.config.js | 15 |
| website/sidebars.js | 9 |
| experimental/utilities/javascript/graphTopoSort.js | 3 |
| experimental/utilities/javascript/mergeSort.js | 2 |
| experimental/utilities/javascript/binToInt.js | 2 |
| experimental/utilities/javascript/isSubsequence.js | 1 |
| experimental/utilities/javascript/intToBin.js | 1 |
| experimental/utilities/javascript/matrixClone.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1942}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4213454281363364, "precision": 0.4171541080200196, "recall": 0.4259654110400379, "support": 2951}, "micro avg": {"f1-score": 0.9884784818705523, "precision": 0.9884784818705523, "recall": 0.9884784818705523, "support": 2951}, "weighted avg": {"f1-score": 0.9847699563698886, "precision": 0.9814574449014549, "recall": 0.9884784818705523, "support": 2951}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce": {"f1-score": 0.9611041405269761, "precision": 0.9251207729468599, "recall": 1.0, "support": 383}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u2423": {"f1-score": 0.9883138564273789, "precision": 0.9949579831932773, "recall": 0.9817578772802653, "support": 603}},
  "cl_report_full": {"\"": {"f1-score": 0.9964084145715751, "precision": 1.0, "recall": 0.9928425357873211, "support": 1956}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "macro avg": {"f1-score": 0.31822566689526777, "precision": 0.4171541080200196, "recall": 0.27209790183617183, "support": 6868}, "micro avg": {"f1-score": 0.5941541908544659, "precision": 0.9884784818705523, "recall": 0.42472335468841, "support": 6868}, "weighted avg": {"f1-score": 0.47822139759725957, "precision": 0.597220592000527, "recall": 0.42472335468841, "support": 6868}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2266}, "\u23ce": {"f1-score": 0.6519148936170212, "precision": 0.9251207729468599, "recall": 0.5032851511169514, "support": 761}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 179}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u2423": {"f1-score": 0.5792563600782779, "precision": 0.9949579831932773, "recall": 0.4085576259489303, "support": 1449}},
  "ppcr": 0.42967384973791495
}
```
</details>
