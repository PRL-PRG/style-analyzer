# Train report for javascript / file:///tmp/top-repos-quality-repos-kl6i6z45/openglol.git HEAD 9f62f0ded5041f76b8518a002509722b2c152a4b

### Classification report

PPCR: 0.817

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.954| 0.997| 0.947| 0.975| 0.950| 2485| 2615| 0.950 |
| `␣` | 0.990| 0.945| 0.738| 0.967| 0.846| 983| 1258| 0.781 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 64| 321| 0.199 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 131| 0.023 |
| `micro avg` | 0.964| 0.964| 0.788| 0.964| 0.867| 3535| 4325| 0.817 |
| `macro avg` | 0.486| 0.485| 0.421| 0.486| 0.449| 3535| 4325| 0.817 |
| `weighted avg` | 0.946| 0.964| 0.788| 0.954| 0.821| 3535| 4325| 0.817 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|130 |2477 |8 |0 |0 |
|275 |54 |929 |0 |0 |
|257 |63 |1 |0 |0 |
|128 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| bubbles.js | 47 |
| webgl-utils.js | 46 |
| phong-shading.js | 19 |
| sphere-animate.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.485504411667395, "precision": 0.48604948879016546, "recall": 0.4854617020536239, "support": 3535}, "micro avg": {"f1-score": 0.9635077793493635, "precision": 0.9635077793493635, "recall": 0.9635077793493635, "support": 3535}, "weighted avg": {"f1-score": 0.9542213780455095, "precision": 0.945896303375819, "recall": 0.9635077793493635, "support": 3535}, "\u2205": {"f1-score": 0.9748130657221566, "precision": 0.9537928378898729, "recall": 0.9967806841046277, "support": 2485}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9672045809474232, "precision": 0.990405117270789, "recall": 0.9450661241098678, "support": 983}},
  "cl_report_full": {"macro avg": {"f1-score": 0.4491456593792942, "precision": 0.48604948879016546, "recall": 0.4214253253365839, "support": 4325}, "micro avg": {"f1-score": 0.8666666666666667, "precision": 0.9635077793493635, "recall": 0.7875144508670521, "support": 4325}, "weighted avg": {"f1-score": 0.8207925770710687, "precision": 0.8647625222216578, "recall": 0.7875144508670521, "support": 4325}, "\u2205": {"f1-score": 0.9504988488104373, "precision": 0.9537928378898729, "recall": 0.947227533460803, "support": 2615}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 321}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 131}, "\u2423": {"f1-score": 0.8460837887067395, "precision": 0.990405117270789, "recall": 0.7384737678855325, "support": 1258}},
  "ppcr": 0.8173410404624277
}
```
</details>
