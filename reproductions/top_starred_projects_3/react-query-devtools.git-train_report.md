# Train report for javascript / file:///tmp/top-repos-quality-repos-dcsc9gu1/react-query-devtools.git HEAD d8cd2cb107a7cfaabb42b0fbc9fb144b343ec51c

### Classification report

PPCR: 0.135

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.911| 1.000| 0.953| 543| 596| 0.911 |
| `⏎` | 1.000| 1.000| 0.541| 1.000| 0.702| 205| 379| 0.541 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2736| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1381| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 235| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 225| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.135| 1.000| 0.150| 748| 5552| 0.135 |
| `micro avg` | 1.000| 1.000| 0.135| 1.000| 0.237| 748| 5552| 0.135 |
| `macro avg` | 0.333| 0.333| 0.242| 0.333| 0.276| 748| 5552| 0.135 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|2736 |0 |0 |0 |0 |0 |0 |
|1381 |0 |0 |0 |0 |0 |0 |
|53 |0 |0 |543 |0 |0 |0 |
|174 |0 |0 |0 |205 |0 |0 |
|235 |0 |0 |0 |0 |0 |0 |
|225 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 543}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 748}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 748}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 748}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 205}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9534679543459175, "precision": 1.0, "recall": 0.9110738255033557, "support": 596}, "macro avg": {"f1-score": 0.27592045814441085, "precision": 0.3333333333333333, "recall": 0.2419951538547809, "support": 5552}, "micro avg": {"f1-score": 0.23746031746031748, "precision": 1.0, "recall": 0.13472622478386168, "support": 5552}, "weighted avg": {"f1-score": 0.1502783983993974, "precision": 0.17561239193083575, "recall": 0.13472622478386168, "support": 5552}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2736}, "\u23ce": {"f1-score": 0.7020547945205479, "precision": 1.0, "recall": 0.5408970976253298, "support": 379}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 225}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 235}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1381}},
  "ppcr": 0.13472622478386168
}
```
</details>
