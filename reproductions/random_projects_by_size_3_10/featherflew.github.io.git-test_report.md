# Test report for javascript / file:///tmp/top-repos-quality-repos-zhrgqjyr/featherflew.github.io.git HEAD 6c55e8fb2447c369eb92e28d939573551d4ed62f

### Classification report

PPCR: 0.558

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.908| 0.920| 0.729| 0.914| 0.809| 289| 365| 0.792 |
| `␣` | 0.703| 0.933| 0.413| 0.802| 0.520| 104| 235| 0.443 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 86| 0.430 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 11| 0.091 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 41| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 23| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 11| 0.000 |
| `weighted avg` | 0.778| 0.842| 0.470| 0.806| 0.541| 431| 772| 0.558 |
| `micro avg` | 0.842| 0.842| 0.470| 0.842| 0.603| 431| 772| 0.558 |
| `macro avg` | 0.230| 0.265| 0.163| 0.245| 0.190| 431| 772| 0.558 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|76 |266 |23 |0 |0 |0 |0 |0 |
|131 |7 |97 |0 |0 |0 |0 |0 |
|49 |19 |18 |0 |0 |0 |0 |0 |
|41 |0 |0 |0 |0 |0 |0 |0 |
|10 |1 |0 |0 |0 |0 |0 |0 |
|11 |0 |0 |0 |0 |0 |0 |0 |
|23 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "macro avg": {"f1-score": 0.2451060342344316, "precision": 0.23010691143945336, "recall": 0.2647296475151147, "support": 431}, "micro avg": {"f1-score": 0.8422273781902552, "precision": 0.8422273781902552, "recall": 0.8422273781902552, "support": 431}, "weighted avg": {"f1-score": 0.8063659446224783, "precision": 0.7783527841251884, "recall": 0.8422273781902552, "support": 431}, "\u2205": {"f1-score": 0.9140893470790378, "precision": 0.9078498293515358, "recall": 0.9204152249134948, "support": 289}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8016528925619835, "precision": 0.7028985507246377, "recall": 0.9326923076923077, "support": 104}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "macro avg": {"f1-score": 0.18980255384339578, "precision": 0.23010691143945336, "recall": 0.16307615439063997, "support": 772}, "micro avg": {"f1-score": 0.6034912718204488, "precision": 0.8422273781902552, "recall": 0.47020725388601037, "support": 772}, "weighted avg": {"f1-score": 0.5405849534340795, "precision": 0.6431947501730575, "recall": 0.47020725388601037, "support": 772}, "\u2205": {"f1-score": 0.8085106382978723, "precision": 0.9078498293515358, "recall": 0.7287671232876712, "support": 365}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.520107238605898, "precision": 0.7028985507246377, "recall": 0.4127659574468085, "support": 235}},
  "ppcr": 0.5582901554404145
}
```
</details>
