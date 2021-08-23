# Test report for javascript / file:///tmp/top-repos-quality-repos-skeutftm/tads.git HEAD a39116a8182e7cf99e23ee1692854f3260646cd0

### Classification report

PPCR: 0.678

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.851| 0.985| 0.838| 0.913| 0.844| 330| 388| 0.851 |
| `␣` | 0.939| 0.704| 0.467| 0.805| 0.624| 152| 229| 0.664 |
| `'` | 1.000| 0.886| 0.500| 0.939| 0.667| 35| 62| 0.565 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 51| 0.098 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 24| 0.167 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 23| 0.043 |
| `weighted avg` | 0.870| 0.879| 0.596| 0.866| 0.659| 527| 777| 0.678 |
| `micro avg` | 0.879| 0.879| 0.596| 0.879| 0.710| 527| 777| 0.678 |
| `macro avg` | 0.465| 0.429| 0.301| 0.443| 0.356| 527| 777| 0.678 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|58 |325 |5 |0 |0 |0 |0 |
|77 |45 |107 |0 |0 |0 |0 |
|27 |4 |0 |31 |0 |0 |0 |
|46 |5 |0 |0 |0 |0 |0 |
|22 |1 |0 |0 |0 |0 |0 |
|20 |2 |2 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9393939393939393, "precision": 1.0, "recall": 0.8857142857142857, "support": 35}, "macro avg": {"f1-score": 0.44280442765067246, "precision": 0.46489697192370105, "recall": 0.4290850231639705, "support": 527}, "micro avg": {"f1-score": 0.8785578747628083, "precision": 0.8785578747628083, "recall": 0.8785578747628083, "support": 527}, "weighted avg": {"f1-score": 0.8660883246837235, "precision": 0.8698782333402877, "recall": 0.8785578747628083, "support": 527}, "\u2205": {"f1-score": 0.9129213483146067, "precision": 0.8507853403141361, "recall": 0.9848484848484849, "support": 330}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.8045112781954887, "precision": 0.9385964912280702, "recall": 0.7039473684210527, "support": 152}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 62}, "macro avg": {"f1-score": 0.35578820272697825, "precision": 0.46489697192370105, "recall": 0.3008129623793874, "support": 777}, "micro avg": {"f1-score": 0.7101226993865031, "precision": 0.8785578747628083, "recall": 0.5958815958815958, "support": 777}, "weighted avg": {"f1-score": 0.6586106003015624, "precision": 0.7812655193476356, "recall": 0.5958815958815958, "support": 777}, "\u2205": {"f1-score": 0.8441558441558441, "precision": 0.8507853403141361, "recall": 0.8376288659793815, "support": 388}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u2423": {"f1-score": 0.6239067055393586, "precision": 0.9385964912280702, "recall": 0.4672489082969432, "support": 229}},
  "ppcr": 0.6782496782496783
}
```
</details>
