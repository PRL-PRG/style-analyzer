# Test report for javascript / file:///tmp/top-repos-quality-repos-ry4jnrsx/infinity.git HEAD 2f38017af57363ef3b7f6d584264074b337eba22

### Classification report

PPCR: 0.836

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.965| 0.939| 0.968| 0.955| 1895| 1947| 0.973 |
| `␣` | 0.869| 0.952| 0.743| 0.908| 0.801| 682| 874| 0.780 |
| `⏎` | 0.853| 0.919| 0.636| 0.885| 0.729| 285| 412| 0.692 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 97| 0.495 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 91| 0.176 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 88| 0.102 |
| `macro avg` | 0.449| 0.473| 0.386| 0.460| 0.414| 2935| 3509| 0.836 |
| `weighted avg` | 0.912| 0.933| 0.781| 0.922| 0.815| 2935| 3509| 0.836 |
| `micro avg` | 0.933| 0.933| 0.781| 0.933| 0.850| 2935| 3509| 0.836 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|52 |1828 |67 |0 |0 |0 |0 |
|192 |33 |649 |0 |0 |0 |0 |
|127 |3 |20 |262 |0 |0 |0 |
|49 |3 |0 |45 |0 |0 |0 |
|75 |5 |11 |0 |0 |0 |0 |
|79 |9 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.46028049597794346, "precision": 0.44900871019712923, "recall": 0.47259249138535625, "support": 2935}, "micro avg": {"f1-score": 0.9332197614991482, "precision": 0.9332197614991482, "recall": 0.9332197614991482, "support": 2935}, "weighted avg": {"f1-score": 0.9221534623780363, "precision": 0.912217283741067, "recall": 0.9332197614991482, "support": 2935}, "\u2205": {"f1-score": 0.9682203389830508, "precision": 0.9718234981392876, "recall": 0.9646437994722955, "support": 1895}, "\u23ce": {"f1-score": 0.8851351351351351, "precision": 0.8534201954397395, "recall": 0.9192982456140351, "support": 285}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.9083275017494752, "precision": 0.8688085676037484, "recall": 0.9516129032258065, "support": 682}},
  "cl_report_full": {"macro avg": {"f1-score": 0.41409969840873395, "precision": 0.44900871019712923, "recall": 0.38622759797828493, "support": 3509}, "micro avg": {"f1-score": 0.8500931098696461, "precision": 0.9332197614991482, "recall": 0.780564263322884, "support": 3509}, "weighted avg": {"f1-score": 0.8149403601222202, "precision": 0.8558244968606559, "recall": 0.780564263322884, "support": 3509}, "\u2205": {"f1-score": 0.955067920585162, "precision": 0.9718234981392876, "recall": 0.9388803287108372, "support": 1947}, "\u23ce": {"f1-score": 0.7287899860917942, "precision": 0.8534201954397395, "recall": 0.6359223300970874, "support": 412}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "\u2423": {"f1-score": 0.8007402837754473, "precision": 0.8688085676037484, "recall": 0.7425629290617849, "support": 874}},
  "ppcr": 0.8364206326588772
}
```
</details>
