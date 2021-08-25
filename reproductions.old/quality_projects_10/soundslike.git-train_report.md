# Train report for javascript / file:///tmp/top-repos-quality-repos-d64ll3vb/soundslike.git HEAD 02fb51f5c813c9a8fa7071a4f94c74976e1e3989

### Classification report

PPCR: 0.069

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.913| 1.000| 0.304| 0.954| 0.456| 345| 1135| 0.304 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 3187| 0.008 |
| `⏎` | 1.000| 1.000| 0.045| 1.000| 0.085| 11| 247| 0.045 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 190| 0.047 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 32| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 579| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 179| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 90| 0.000 |
| `macro avg` | 0.239| 0.250| 0.044| 0.244| 0.068| 389| 5639| 0.069 |
| `micro avg` | 0.915| 0.915| 0.063| 0.915| 0.118| 389| 5639| 0.069 |
| `weighted avg` | 0.838| 0.915| 0.063| 0.875| 0.096| 389| 5639| 0.069 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3163 |0 |24 |0 |0 |0 |0 |0 |0 |
|790 |0 |345 |0 |0 |0 |0 |0 |0 |
|236 |0 |0 |11 |0 |0 |0 |0 |0 |
|32 |0 |0 |0 |0 |0 |0 |0 |0 |
|579 |0 |0 |0 |0 |0 |0 |0 |0 |
|181 |0 |9 |0 |0 |0 |0 |0 |0 |
|179 |0 |0 |0 |0 |0 |0 |0 |0 |
|90 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/components/login.js | 10 |
| client/actions/index.js | 4 |
| client/reducers/auth.js | 4 |
| client/containers/login.js | 4 |
| client/containers/app.js | 4 |
| client/index.js | 3 |
| test/client/reducers/test_auth.js | 2 |
| client/containers/songs/single.js | 1 |
| client/components/alert.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24429460580912865, "precision": 0.23908730158730157, "recall": 0.25, "support": 389}, "micro avg": {"f1-score": 0.9151670951156813, "precision": 0.9151670951156813, "recall": 0.9151670951156813, "support": 389}, "weighted avg": {"f1-score": 0.8746866633244089, "precision": 0.837740237483168, "recall": 0.9151670951156813, "support": 389}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9543568464730291, "precision": 0.9126984126984127, "recall": 1.0, "support": 345}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 579}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "macro avg": {"f1-score": 0.0676648631754766, "precision": 0.23908730158730157, "recall": 0.04356239633308958, "support": 5639}, "micro avg": {"f1-score": 0.1181154611811546, "precision": 0.9151670951156813, "recall": 0.06313176095052314, "support": 5639}, "weighted avg": {"f1-score": 0.0955268713248394, "precision": 0.22750712864208164, "recall": 0.06313176095052314, "support": 5639}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3187}, "\u23ce": {"f1-score": 0.08527131782945735, "precision": 1.0, "recall": 0.044534412955465584, "support": 247}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 90}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 190}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 179}, "\u2423": {"f1-score": 0.4560475875743555, "precision": 0.9126984126984127, "recall": 0.3039647577092511, "support": 1135}},
  "ppcr": 0.06898386238694804
}
```
</details>
