# Test report for javascript / file:///tmp/top-repos-quality-repos-xao_ipwo/localtunnel.git HEAD 485b81619de7ea1196ea91a1d2c5f1993151be5f

### Classification report

PPCR: 0.378

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.869| 1.000| 0.688| 0.930| 0.768| 53| 77| 0.688 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 12| 24| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 53| 0.132 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 17| 0.059 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 10| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 9| 0.000 |
| `weighted avg` | 0.795| 0.890| 0.337| 0.839| 0.389| 73| 193| 0.378 |
| `macro avg` | 0.267| 0.286| 0.170| 0.276| 0.205| 73| 193| 0.378 |
| `micro avg` | 0.890| 0.890| 0.337| 0.890| 0.489| 73| 193| 0.378 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|24 |53 |0 |0 |0 |0 |0 |0 |
|46 |7 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |12 |0 |0 |0 |0 |
|16 |1 |0 |0 |0 |0 |0 |0 |
|10 |0 |0 |0 |0 |0 |0 |0 |
|9 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 12}, "macro avg": {"f1-score": 0.2756892230576441, "precision": 0.2669789227166276, "recall": 0.2857142857142857, "support": 73}, "micro avg": {"f1-score": 0.8904109589041096, "precision": 0.8904109589041096, "recall": 0.8904109589041096, "support": 73}, "weighted avg": {"f1-score": 0.8394616678683009, "precision": 0.7951942510666966, "recall": 0.8904109589041096, "support": 73}, "\u2205": {"f1-score": 0.9298245614035088, "precision": 0.8688524590163934, "recall": 1.0, "support": 53}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 24}, "macro avg": {"f1-score": 0.2049689440993789, "precision": 0.2669789227166276, "recall": 0.16975881261595546, "support": 193}, "micro avg": {"f1-score": 0.48872180451127817, "precision": 0.8904109589041096, "recall": 0.33678756476683935, "support": 193}, "weighted avg": {"f1-score": 0.3893519561462792, "precision": 0.47099294997027097, "recall": 0.33678756476683935, "support": 193}, "\u2205": {"f1-score": 0.7681159420289856, "precision": 0.8688524590163934, "recall": 0.6883116883116883, "support": 77}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}},
  "ppcr": 0.37823834196891193
}
```
</details>
