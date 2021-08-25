# Train report for javascript / file:///tmp/top-repos-quality-repos-4kd0tek2/instagram-helper.git HEAD d7033052d53d07169abe8643ac495a74e86c1438

### Classification report

PPCR: 0.189

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 1.000| 0.438| 0.991| 0.606| 678| 1548| 0.438 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 281| 0.043 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1014| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 814| 0.000 |
| `macro avg` | 0.246| 0.250| 0.109| 0.248| 0.151| 690| 3657| 0.189 |
| `weighted avg` | 0.966| 0.983| 0.185| 0.974| 0.256| 690| 3657| 0.189 |
| `micro avg` | 0.983| 0.983| 0.185| 0.983| 0.312| 690| 3657| 0.189 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|870 |678 |0 |0 |0 |
|1014 |0 |0 |0 |0 |
|814 |0 |0 |0 |0 |
|269 |12 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| InstagramHelper.js | 6 |
| InstagramHelperQuick.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24780701754385964, "precision": 0.24565217391304348, "recall": 0.25, "support": 690}, "micro avg": {"f1-score": 0.9826086956521739, "precision": 0.9826086956521739, "recall": 0.9826086956521739, "support": 690}, "weighted avg": {"f1-score": 0.9739893211289092, "precision": 0.9655198487712665, "recall": 0.9826086956521739, "support": 690}, "\u2205": {"f1-score": 0.9912280701754386, "precision": 0.9826086956521739, "recall": 1.0, "support": 678}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 814}, "macro avg": {"f1-score": 0.15147453083109919, "precision": 0.24565217391304348, "recall": 0.10949612403100775, "support": 3657}, "micro avg": {"f1-score": 0.3119392684610076, "precision": 0.9826086956521739, "recall": 0.18539786710418377, "support": 3657}, "weighted avg": {"f1-score": 0.25647533358112284, "precision": 0.41593608445982094, "recall": 0.18539786710418377, "support": 3657}, "\u2205": {"f1-score": 0.6058981233243967, "precision": 0.9826086956521739, "recall": 0.437984496124031, "support": 1548}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 281}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1014}},
  "ppcr": 0.18867924528301888
}
```
</details>
