# Train report for javascript / file:///tmp/top-repos-quality-repos-h6ifyxti/coba.git HEAD 0c54dc3736be85f23cb7467610a1c499997f040c

### Classification report

PPCR: 0.280

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.999| 1.000| 0.486| 1.000| 0.654| 3996| 8230| 0.486 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 3742| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 413| 0.002 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 595| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 868| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 427| 0.000 |
| `micro avg` | 0.999| 0.999| 0.280| 0.999| 0.437| 3998| 14275| 0.280 |
| `weighted avg` | 0.999| 0.999| 0.280| 0.999| 0.377| 3998| 14275| 0.280 |
| `macro avg` | 0.167| 0.167| 0.081| 0.167| 0.109| 3998| 14275| 0.280 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|4234 |3996 |0 |0 |0 |0 |0 |
|3741 |1 |0 |0 |0 |0 |0 |
|595 |0 |0 |0 |0 |0 |0 |
|868 |0 |0 |0 |0 |0 |0 |
|427 |0 |0 |0 |0 |0 |0 |
|412 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| extension/modules/logger.jsm | 1 |
| extension/modules/cobaUtils.jsm | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16662496872654492, "precision": 0.16658329164582292, "recall": 0.16666666666666666, "support": 3998}, "micro avg": {"f1-score": 0.9994997498749375, "precision": 0.9994997498749375, "recall": 0.9994997498749375, "support": 3998}, "weighted avg": {"f1-score": 0.9992496873906056, "precision": 0.9989997500000627, "recall": 0.9994997498749375, "support": 3998}, "\u2205": {"f1-score": 0.9997498123592695, "precision": 0.9994997498749375, "recall": 1.0, "support": 3996}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 595}, "macro avg": {"f1-score": 0.10893032384690875, "precision": 0.16658329164582292, "recall": 0.08092345078979345, "support": 14275}, "micro avg": {"f1-score": 0.4373666064685602, "precision": 0.9994997498749375, "recall": 0.27992994746059546, "support": 14275}, "weighted avg": {"f1-score": 0.37681116578356244, "precision": 0.5762439888946225, "recall": 0.27992994746059546, "support": 14275}, "\u2205": {"f1-score": 0.6535819430814525, "precision": 0.9994997498749375, "recall": 0.48554070473876065, "support": 8230}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 868}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 427}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 413}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3742}},
  "ppcr": 0.28007005253940453
}
```
</details>
