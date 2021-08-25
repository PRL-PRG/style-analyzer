# Test report for javascript / file:///tmp/top-repos-quality-repos-zk_qm53x/prepaid-mate.git HEAD 17bf38f67a33d0ce5f099e4a0e02f09d7fdfaa9c

### Classification report

PPCR: 0.922

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.899| 0.960| 0.955| 0.929| 0.926| 727| 731| 0.995 |
| `␣` | 0.853| 0.857| 0.795| 0.855| 0.823| 217| 234| 0.927 |
| `'` | 1.000| 0.667| 0.610| 0.800| 0.758| 141| 154| 0.916 |
| `⏎` | 1.000| 0.909| 0.455| 0.952| 0.625| 33| 66| 0.500 |
| `⏎⏎` | 1.000| 1.000| 0.034| 1.000| 0.067| 1| 29| 0.034 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.594| 0.549| 0.356| 0.567| 0.400| 1119| 1214| 0.922 |
| `weighted avg` | 0.906| 0.902| 0.831| 0.899| 0.848| 1119| 1214| 0.922 |
| `micro avg` | 0.902| 0.902| 0.831| 0.902| 0.865| 1119| 1214| 0.922 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|4 |698 |29 |0 |0 |0 |
|17 |31 |186 |0 |0 |0 |
|33 |2 |1 |30 |0 |0 |
|13 |45 |2 |0 |94 |0 |
|28 |0 |0 |0 |0 |1 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 141}, "macro avg": {"f1-score": 0.567045301842948, "precision": 0.5940869431570983, "recall": 0.5491263092707384, "support": 1119}, "micro avg": {"f1-score": 0.9016979445933869, "precision": 0.9016979445933869, "recall": 0.9016979445933869, "support": 1119}, "weighted avg": {"f1-score": 0.8990573400640942, "precision": 0.9062306047567332, "recall": 0.9016979445933869, "support": 1119}, "\u2205": {"f1-score": 0.9288090485695276, "precision": 0.8994845360824743, "recall": 0.9601100412654745, "support": 727}, "\u23ce": {"f1-score": 0.9523809523809523, "precision": 1.0, "recall": 0.9090909090909091, "support": 33}, "\u23ce\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8551724137931034, "precision": 0.8532110091743119, "recall": 0.8571428571428571, "support": 217}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.7580645161290321, "precision": 1.0, "recall": 0.6103896103896104, "support": 154}, "macro avg": {"f1-score": 0.39988547020208226, "precision": 0.5940869431570983, "recall": 0.3561432474470825, "support": 1214}, "micro avg": {"f1-score": 0.8649807115302186, "precision": 0.9016979445933869, "recall": 0.8311367380560132, "support": 1214}, "weighted avg": {"f1-score": 0.8481603012398987, "precision": 0.9111816902990755, "recall": 0.8311367380560132, "support": 1214}, "\u2205": {"f1-score": 0.9263437292634372, "precision": 0.8994845360824743, "recall": 0.9548563611491108, "support": 731}, "\u23ce": {"f1-score": 0.625, "precision": 1.0, "recall": 0.45454545454545453, "support": 66}, "\u23ce\u23ce": {"f1-score": 0.06666666666666667, "precision": 1.0, "recall": 0.034482758620689655, "support": 29}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8230088495575222, "precision": 0.8532110091743119, "recall": 0.7948717948717948, "support": 234}},
  "ppcr": 0.9217462932454695
}
```
</details>
