# Test report for javascript / file:///tmp/top-repos-quality-repos-0ypvuiim/tech-interview-handbook.git HEAD a5abb22e662530cccbad3932ca7cf93f8a45f7b1

### Classification report

PPCR: 0.646

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.888| 0.875| 0.612| 0.881| 0.725| 993| 1419| 0.700 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 350| 571| 0.613 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 50| 58| 0.862 |
| `⏎` | 0.060| 0.519| 0.132| 0.107| 0.082| 27| 106| 0.255 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 39| 0.282 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 36| 0.278 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.613| 0.613| 0.396| 0.609| 0.465| 1441| 2229| 0.646 |
| `micro avg` | 0.613| 0.613| 0.396| 0.613| 0.481| 1441| 2229| 0.646 |
| `macro avg` | 0.135| 0.199| 0.106| 0.141| 0.115| 1441| 2229| 0.646 |

### Confusion matrix

|refusal|  "| ␣| ∅| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |
|221 |58 |0 |80 |212 |0 |0 |0 |
|426 |124 |0 |869 |0 |0 |0 |0 |
|79 |13 |0 |0 |14 |0 |0 |0 |
|8 |29 |0 |21 |0 |0 |0 |0 |
|28 |0 |0 |9 |2 |0 |0 |0 |
|26 |4 |0 |0 |6 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "macro avg": {"f1-score": 0.14123120512572845, "precision": 0.1353527870381803, "recall": 0.1990920570980994, "support": 1441}, "micro avg": {"f1-score": 0.6127689104788342, "precision": 0.6127689104788342, "recall": 0.6127689104788342, "support": 1441}, "weighted avg": {"f1-score": 0.6093448458854215, "precision": 0.6127983004215958, "recall": 0.6127689104788342, "support": 1441}, "\u2205": {"f1-score": 0.8813387423935091, "precision": 0.8876404494382022, "recall": 0.8751258811681772, "support": 993}, "\u23ce": {"f1-score": 0.10727969348659004, "precision": 0.05982905982905983, "recall": 0.5185185185185185, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 350}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "macro avg": {"f1-score": 0.11530336905404363, "precision": 0.1353527870381803, "recall": 0.10635408178190099, "support": 2229}, "micro avg": {"f1-score": 0.48119891008174387, "precision": 0.6127689104788342, "recall": 0.39614176760879316, "support": 2229}, "weighted avg": {"f1-score": 0.4653113293176804, "precision": 0.5679244854619512, "recall": 0.39614176760879316, "support": 2229}, "\u2205": {"f1-score": 0.7247706422018348, "precision": 0.8876404494382022, "recall": 0.6124031007751938, "support": 1419}, "\u23ce": {"f1-score": 0.0823529411764706, "precision": 0.05982905982905983, "recall": 0.1320754716981132, "support": 106}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 571}},
  "ppcr": 0.6464782413638402
}
```
</details>
