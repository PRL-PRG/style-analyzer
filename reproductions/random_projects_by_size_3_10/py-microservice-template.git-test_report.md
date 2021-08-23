# Test report for javascript / file:///tmp/top-repos-quality-repos-dxxuftfx/py-microservice-template.git HEAD 9a899bbaad60e3025b37ea499d86ba334eb8c01e

### Classification report

PPCR: 0.935

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 0.629| 1.000| 1.000| 0.773| 0.773| 700| 700| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 330| 330| 1.000 |
| `␣` | 0.913| 0.719| 0.596| 0.804| 0.721| 306| 369| 0.829 |
| `∅` | 0.971| 0.935| 0.879| 0.953| 0.922| 248| 264| 0.939 |
| `⏎` | 0.996| 1.000| 0.935| 0.998| 0.965| 245| 262| 0.935 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.857| 0.571| 0.429| 0.686| 0.571| 21| 28| 0.750 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.833| 0.625| 0.172| 0.714| 0.286| 8| 29| 0.276 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 5| 0.000 |
| `weighted avg` | 0.662| 0.761| 0.712| 0.693| 0.668| 1858| 1987| 0.935 |
| `micro avg` | 0.761| 0.761| 0.712| 0.761| 0.736| 1858| 1987| 0.935 |
| `macro avg` | 0.650| 0.606| 0.501| 0.616| 0.530| 1858| 1987| 0.935 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|16 |232 |13 |0 |0 |2 |1 |0 |0 |
|63 |5 |220 |81 |0 |0 |0 |0 |0 |
|0 |0 |0 |700 |0 |0 |0 |0 |0 |
|17 |0 |0 |0 |245 |0 |0 |0 |0 |
|7 |0 |8 |1 |0 |12 |0 |0 |0 |
|21 |2 |0 |0 |1 |0 |5 |0 |0 |
|0 |0 |0 |330 |0 |0 |0 |0 |0 |
|5 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 330}, "\u0027": {"f1-score": 0.7726269315673289, "precision": 0.6294964028776978, "recall": 1.0, "support": 700}, "macro avg": {"f1-score": 0.6159687392709072, "precision": 0.6499352400392538, "recall": 0.6063583363452908, "support": 1858}, "micro avg": {"f1-score": 0.7610333692142088, "precision": 0.7610333692142088, "recall": 0.7610333692142088, "support": 1858}, "weighted avg": {"f1-score": 0.6931562036878718, "precision": 0.6616742276536104, "recall": 0.7610333692142088, "support": 1858}, "\u2205": {"f1-score": 0.9527720739219712, "precision": 0.9707112970711297, "recall": 0.9354838709677419, "support": 248}, "\u23ce": {"f1-score": 0.9979633401221996, "precision": 0.9959349593495935, "recall": 1.0, "support": 245}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.6857142857142857, "precision": 0.8571428571428571, "recall": 0.5714285714285714, "support": 21}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7142857142857143, "precision": 0.8333333333333334, "recall": 0.625, "support": 8}, "\u2423": {"f1-score": 0.8043875685557587, "precision": 0.9128630705394191, "recall": 0.7189542483660131, "support": 306}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 330}, "\u0027": {"f1-score": 0.7726269315673289, "precision": 0.6294964028776978, "recall": 1.0, "support": 700}, "macro avg": {"f1-score": 0.5297641752501745, "precision": 0.6499352400392538, "recall": 0.5013866957923963, "support": 1987}, "micro avg": {"f1-score": 0.7355006501950586, "precision": 0.7610333692142088, "recall": 0.7116255661801711, "support": 1987}, "weighted avg": {"f1-score": 0.6681106448673825, "precision": 0.6758245412614355, "recall": 0.7116255661801711, "support": 1987}, "\u2205": {"f1-score": 0.9224652087475149, "precision": 0.9707112970711297, "recall": 0.8787878787878788, "support": 264}, "\u23ce": {"f1-score": 0.9645669291338583, "precision": 0.9959349593495935, "recall": 0.9351145038167938, "support": 262}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.5714285714285714, "precision": 0.8571428571428571, "recall": 0.42857142857142855, "support": 28}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.28571428571428575, "precision": 0.8333333333333334, "recall": 0.1724137931034483, "support": 29}, "\u2423": {"f1-score": 0.721311475409836, "precision": 0.9128630705394191, "recall": 0.5962059620596206, "support": 369}},
  "ppcr": 0.9350780070457977
}
```
</details>