# Test report for javascript / file:///tmp/top-repos-quality-repos-rhxk0a97/apprend.git HEAD 8b2249d46328e6707f929d458d345408193eabdd

### Classification report

PPCR: 0.974

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 0.941| 0.926| 0.945| 0.937| 3775| 3838| 0.984 |
| `␣` | 0.858| 0.937| 0.930| 0.896| 0.892| 2251| 2269| 0.992 |
| `'` | 0.974| 0.828| 0.817| 0.896| 0.889| 781| 792| 0.986 |
| `⏎` | 0.677| 0.703| 0.650| 0.690| 0.663| 454| 491| 0.925 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.797| 0.818| 0.818| 0.808| 0.808| 303| 303| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.956| 0.911| 0.794| 0.933| 0.867| 237| 272| 0.871 |
| `⏎⏎` | 0.882| 0.411| 0.314| 0.561| 0.463| 146| 191| 0.764 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 12| 1.000 |
| `micro avg` | 0.899| 0.899| 0.876| 0.899| 0.887| 7959| 8168| 0.974 |
| `weighted avg` | 0.902| 0.899| 0.876| 0.898| 0.884| 7959| 8168| 0.974 |
| `macro avg` | 0.762| 0.694| 0.656| 0.716| 0.690| 7959| 8168| 0.974 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|63 |3553 |154 |0 |0 |5 |57 |6 |0 |
|18 |57 |2110 |0 |0 |76 |4 |4 |0 |
|0 |0 |0 |0 |12 |0 |0 |0 |0 |
|11 |64 |35 |12 |647 |21 |2 |0 |0 |
|37 |33 |85 |4 |5 |319 |0 |0 |8 |
|0 |12 |40 |1 |0 |2 |248 |0 |0 |
|35 |5 |12 |0 |0 |4 |0 |216 |0 |
|45 |18 |24 |0 |0 |44 |0 |0 |60 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u0027": {"f1-score": 0.8955017301038062, "precision": 0.9743975903614458, "recall": 0.8284250960307298, "support": 781}, "macro avg": {"f1-score": 0.7159927306903404, "precision": 0.761803575243641, "recall": 0.6938068313733514, "support": 7959}, "micro avg": {"f1-score": 0.8987309963563261, "precision": 0.8987309963563261, "recall": 0.8987309963563261, "support": 7959}, "weighted avg": {"f1-score": 0.8977614091845186, "precision": 0.9021883549171977, "recall": 0.8987309963563261, "support": 7959}, "\u2205": {"f1-score": 0.9453239324198482, "precision": 0.9494922501336184, "recall": 0.9411920529801324, "support": 3775}, "\u23ce": {"f1-score": 0.6897297297297298, "precision": 0.6772823779193206, "recall": 0.7026431718061674, "support": 454}, "\u23ce\u23ce": {"f1-score": 0.5607476635514018, "precision": 0.8823529411764706, "recall": 0.410958904109589, "support": 146}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8078175895765473, "precision": 0.797427652733119, "recall": 0.8184818481848185, "support": 303}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9330453563714902, "precision": 0.9557522123893806, "recall": 0.9113924050632911, "support": 237}, "\u2423": {"f1-score": 0.8957758437699002, "precision": 0.8577235772357723, "recall": 0.9373611728120835, "support": 2251}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u0027": {"f1-score": 0.8887362637362638, "precision": 0.9743975903614458, "recall": 0.8169191919191919, "support": 792}, "macro avg": {"f1-score": 0.6900473910769558, "precision": 0.761803575243641, "recall": 0.6561271206524408, "support": 8168}, "micro avg": {"f1-score": 0.887083772555342, "precision": 0.8987309963563261, "recall": 0.8757345739471106, "support": 8168}, "weighted avg": {"f1-score": 0.8841213666517838, "precision": 0.9016539157931193, "recall": 0.8757345739471106, "support": 8168}, "\u2205": {"f1-score": 0.9374670184696571, "precision": 0.9494922501336184, "recall": 0.9257425742574258, "support": 3838}, "\u23ce": {"f1-score": 0.6632016632016633, "precision": 0.6772823779193206, "recall": 0.6496945010183299, "support": 491}, "\u23ce\u23ce": {"f1-score": 0.4633204633204633, "precision": 0.8823529411764706, "recall": 0.31413612565445026, "support": 191}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8078175895765473, "precision": 0.797427652733119, "recall": 0.8184818481848185, "support": 303}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8674698795180723, "precision": 0.9557522123893806, "recall": 0.7941176470588235, "support": 272}, "\u2423": {"f1-score": 0.8923662507929794, "precision": 0.8577235772357723, "recall": 0.9299250771264874, "support": 2269}},
  "ppcr": 0.9744123408423114
}
```
</details>