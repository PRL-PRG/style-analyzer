# Train report for javascript / file:///tmp/top-repos-quality-repos-7sq0d9_u/leetcode-javascript.git HEAD 87cf78f84c086f2801f9de7093c68f768d1a0204

### Classification report

PPCR: 0.808

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.978| 0.921| 0.977| 0.948| 7890| 8372| 0.942 |
| `␣` | 0.965| 0.971| 0.864| 0.968| 0.912| 5510| 6194| 0.890 |
| `⏎` | 0.954| 0.956| 0.375| 0.955| 0.538| 546| 1392| 0.392 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 180| 0.122 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 451| 0.027 |
| `'` | 1.000| 1.000| 0.125| 1.000| 0.222| 4| 32| 0.125 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 134| 0.022 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 443| 0.002 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 125| 0.008 |
| `macro avg` | 0.433| 0.434| 0.254| 0.433| 0.291| 13989| 17323| 0.808 |
| `weighted avg` | 0.969| 0.972| 0.785| 0.970| 0.828| 13989| 17323| 0.808 |
| `micro avg` | 0.972| 0.972| 0.785| 0.972| 0.868| 13989| 17323| 0.808 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|482 |7714 |174 |2 |0 |0 |0 |0 |0 |0 |
|684 |158 |5351 |1 |0 |0 |0 |0 |0 |0 |
|846 |15 |9 |522 |0 |0 |0 |0 |0 |0 |
|439 |4 |7 |1 |0 |0 |0 |0 |0 |0 |
|442 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|158 |1 |0 |21 |0 |0 |0 |0 |0 |0 |
|28 |0 |0 |0 |0 |0 |0 |4 |0 |0 |
|131 |0 |3 |0 |0 |0 |0 |0 |0 |0 |
|124 |1 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/leetcode/isValidSudoku.js | 27 |
| src/leetcode/Queue.js | 23 |
| src/leetcode/S1314_Matrix Block Sum/matrixBlockSum.js | 21 |
| src/leetcode/S0007_Reverse_Integer/reverse.js | 10 |
| src/leetcode/test.js | 10 |
| src/leetcode/155_MinStack.js | 9 |
| src/leetcode/S0304_Range Sum Query 2D - Immutable/NumMatrix.js | 8 |
| src/leetcode/S1315_Sum of Nodes with Even-Valued Grandparent/sumEvenGrandparent.js | 8 |
| src/leetcode/617_mergeTrees.js | 8 |
| src/leetcode/computeArea.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4}, "macro avg": {"f1-score": 0.43341901076313494, "precision": 0.4329646247629413, "recall": 0.43387562381786504, "support": 13989}, "micro avg": {"f1-score": 0.9715490742726428, "precision": 0.9715490742726428, "recall": 0.9715490742726428, "support": 13989}, "weighted avg": {"f1-score": 0.970198646568168, "precision": 0.9688555675377467, "recall": 0.9715490742726428, "support": 13989}, "\u2205": {"f1-score": 0.9774455144450076, "precision": 0.9771978718013681, "recall": 0.9776932826362484, "support": 7890}, "\u23ce": {"f1-score": 0.9551692589204025, "precision": 0.9542961608775137, "recall": 0.9560439560439561, "support": 546}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9681563235028043, "precision": 0.9651875901875901, "recall": 0.9711433756805807, "support": 5510}},
  "cl_report_full": {"\u0027": {"f1-score": 0.2222222222222222, "precision": 1.0, "recall": 0.125, "support": 32}, "macro avg": {"f1-score": 0.2912072481456029, "precision": 0.4329646247629413, "recall": 0.2539228034658395, "support": 17323}, "micro avg": {"f1-score": 0.8681016862544711, "precision": 0.9715490742726428, "recall": 0.7845638746175605, "support": 17323}, "weighted avg": {"f1-score": 0.828066548124417, "precision": 0.8959102218024871, "recall": 0.7845638746175605, "support": 17323}, "\u2205": {"f1-score": 0.9484814951432436, "precision": 0.9771978718013681, "recall": 0.9214046822742475, "support": 8372}, "\u23ce": {"f1-score": 0.5384218669417224, "precision": 0.9542961608775137, "recall": 0.375, "support": 1392}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 451}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 134}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 443}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 125}, "\u2423": {"f1-score": 0.9117396490032373, "precision": 0.9651875901875901, "recall": 0.863900548918308, "support": 6194}},
  "ppcr": 0.8075391098539514
}
```
</details>
