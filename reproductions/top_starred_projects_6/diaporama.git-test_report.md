# Test report for javascript / file:///tmp/top-repos-quality-repos-2wtpzdq7/diaporama.git HEAD 1aeab0bd934ec4c7f3abb823e45332b44bd68d9f

### Classification report

PPCR: 0.933

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.934| 0.977| 0.952| 0.955| 0.943| 1982| 2035| 0.974 |
| `␣` | 0.916| 0.894| 0.792| 0.905| 0.850| 856| 966| 0.886 |
| `⏎` | 0.898| 0.812| 0.749| 0.853| 0.817| 261| 283| 0.922 |
| `"` | 1.000| 0.902| 0.696| 0.948| 0.821| 122| 158| 0.772 |
| `⏎␣⁺␣⁺` | 0.971| 0.927| 0.895| 0.949| 0.932| 110| 114| 0.965 |
| `⏎␣⁻␣⁻` | 0.949| 0.877| 0.877| 0.912| 0.912| 106| 106| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 45| 0.444 |
| `micro avg` | 0.931| 0.931| 0.868| 0.931| 0.899| 3457| 3707| 0.933 |
| `weighted avg` | 0.926| 0.931| 0.868| 0.928| 0.891| 3457| 3707| 0.933 |
| `macro avg` | 0.810| 0.770| 0.709| 0.789| 0.753| 3457| 3707| 0.933 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|53 |1937 |45 |0 |0 |0 |0 |0 |
|110 |83 |765 |2 |3 |0 |3 |0 |
|22 |32 |17 |212 |0 |0 |0 |0 |
|4 |0 |8 |0 |102 |0 |0 |0 |
|36 |10 |0 |0 |0 |110 |2 |0 |
|0 |11 |0 |2 |0 |0 |93 |0 |
|25 |0 |0 |20 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9482758620689655, "precision": 1.0, "recall": 0.9016393442622951, "support": 122}, "macro avg": {"f1-score": 0.7888786147194635, "precision": 0.8098965014119787, "recall": 0.7699311926047301, "support": 3457}, "micro avg": {"f1-score": 0.9311541799247903, "precision": 0.9311541799247903, "recall": 0.9311541799247903, "support": 3457}, "weighted avg": {"f1-score": 0.9277998368674506, "precision": 0.925691703593628, "recall": 0.9311541799247903, "support": 3457}, "\u2205": {"f1-score": 0.955363748458693, "precision": 0.9343945972021225, "recall": 0.9772956609485368, "support": 1982}, "\u23ce": {"f1-score": 0.8531187122736418, "precision": 0.8983050847457628, "recall": 0.8122605363984674, "support": 261}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9488372093023255, "precision": 0.9714285714285714, "recall": 0.9272727272727272, "support": 110}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.911764705882353, "precision": 0.9489795918367347, "recall": 0.8773584905660378, "support": 106}, "\u2423": {"f1-score": 0.9047900650502662, "precision": 0.9161676646706587, "recall": 0.8936915887850467, "support": 856}},
  "cl_report_full": {"\"": {"f1-score": 0.8208955223880596, "precision": 1.0, "recall": 0.6962025316455697, "support": 158}, "macro avg": {"f1-score": 0.7533841108935031, "precision": 0.8098965014119787, "recall": 0.7087403842531405, "support": 3707}, "micro avg": {"f1-score": 0.8986599664991625, "precision": 0.9311541799247903, "recall": 0.8683571621257081, "support": 3707}, "weighted avg": {"f1-score": 0.8911424752782572, "precision": 0.9198991103962173, "recall": 0.8683571621257081, "support": 3707}, "\u2205": {"f1-score": 0.9430379746835442, "precision": 0.9343945972021225, "recall": 0.9518427518427518, "support": 2035}, "\u23ce": {"f1-score": 0.8169556840077071, "precision": 0.8983050847457628, "recall": 0.7491166077738516, "support": 283}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9315068493150684, "precision": 0.9714285714285714, "recall": 0.8947368421052632, "support": 114}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.911764705882353, "precision": 0.9489795918367347, "recall": 0.8773584905660378, "support": 106}, "\u2423": {"f1-score": 0.8495280399777901, "precision": 0.9161676646706587, "recall": 0.7919254658385093, "support": 966}},
  "ppcr": 0.9325600215807931
}
```
</details>