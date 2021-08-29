# Train report for javascript / file:///tmp/top-repos-quality-repos-z2pbtfn7/babel-plugin-root-import.git HEAD 4ad4ac78df8168541f195db847318cc731c3bd53

### Classification report

PPCR: 0.282

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.998| 1.000| 0.360| 0.999| 0.529| 503| 1398| 0.360 |
| `␣` | 1.000| 1.000| 0.195| 1.000| 0.327| 123| 630| 0.195 |
| `'` | 1.000| 1.000| 0.502| 1.000| 0.669| 120| 239| 0.502 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 111| 0.009 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 111| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 91| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 65| 0.000 |
| `weighted avg` | 0.997| 0.999| 0.282| 0.998| 0.418| 747| 2645| 0.282 |
| `macro avg` | 0.428| 0.429| 0.151| 0.428| 0.218| 747| 2645| 0.282 |
| `micro avg` | 0.999| 0.999| 0.282| 0.999| 0.440| 747| 2645| 0.282 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|895 |503 |0 |0 |0 |0 |0 |0 |
|507 |0 |123 |0 |0 |0 |0 |0 |
|119 |0 |0 |120 |0 |0 |0 |0 |
|111 |0 |0 |0 |0 |0 |0 |0 |
|110 |1 |0 |0 |0 |0 |0 |0 |
|91 |0 |0 |0 |0 |0 |0 |0 |
|65 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/helper.spec.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 120}, "macro avg": {"f1-score": 0.4284295644772308, "precision": 0.4282879818594104, "recall": 0.42857142857142855, "support": 747}, "micro avg": {"f1-score": 0.998661311914324, "precision": 0.998661311914324, "recall": 0.998661311914324, "support": 747}, "weighted avg": {"f1-score": 0.9979926325626904, "precision": 0.997325279955802, "recall": 0.998661311914324, "support": 747}, "\u2205": {"f1-score": 0.9990069513406157, "precision": 0.998015873015873, "recall": 1.0, "support": 503}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 123}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6685236768802229, "precision": 1.0, "recall": 0.502092050209205, "support": 239}, "macro avg": {"f1-score": 0.21773340478852868, "precision": 0.4282879818594104, "recall": 0.15101855133203818, "support": 2645}, "micro avg": {"f1-score": 0.43985849056603776, "precision": 0.998661311914324, "recall": 0.2820415879017013, "support": 2645}, "weighted avg": {"f1-score": 0.41777684663515635, "precision": 0.8560401476280495, "recall": 0.2820415879017013, "support": 2645}, "\u2205": {"f1-score": 0.5289169295478443, "precision": 0.998015873015873, "recall": 0.3597997138769671, "support": 1398}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 111}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 111}, "\u2423": {"f1-score": 0.32669322709163345, "precision": 1.0, "recall": 0.19523809523809524, "support": 630}},
  "ppcr": 0.2824196597353497
}
```
</details>
