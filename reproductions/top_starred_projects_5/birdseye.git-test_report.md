# Test report for javascript / file:///tmp/top-repos-quality-repos-nbgsvwuk/birdseye.git HEAD 0f0e0b40e16584d232587e0cffec41660a158aa0

### Classification report

PPCR: 0.995

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.878| 0.955| 0.955| 0.915| 0.915| 1515| 1515| 1.000 |
| `␣` | 0.794| 0.965| 0.952| 0.871| 0.866| 782| 793| 0.986 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 243| 244| 0.996 |
| `⏎` | 0.737| 0.645| 0.641| 0.688| 0.685| 152| 153| 0.993 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 46| 46| 1.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.753| 0.840| 0.836| 0.793| 0.792| 2738| 2751| 0.995 |
| `macro avg` | 0.344| 0.366| 0.364| 0.353| 0.352| 2738| 2751| 0.995 |
| `micro avg` | 0.840| 0.840| 0.836| 0.840| 0.838| 2738| 2751| 0.995 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|0 |1447 |63 |1 |4 |0 |0 |
|11 |26 |755 |0 |1 |0 |0 |
|1 |2 |52 |98 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |
|1 |171 |65 |6 |1 |0 |0 |
|0 |2 |16 |28 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 243}, "macro avg": {"f1-score": 0.3534278376648782, "precision": 0.34411103464612347, "recall": 0.36647507134806706, "support": 2738}, "micro avg": {"f1-score": 0.8400292184075968, "precision": 0.8400292184075968, "recall": 0.8400292184075968, "support": 2738}, "weighted avg": {"f1-score": 0.7933024919248834, "precision": 0.7534887454726371, "recall": 0.8400292184075968, "support": 2738}, "\u2205": {"f1-score": 0.9149541574454633, "precision": 0.8780339805825242, "recall": 0.9551155115511551, "support": 1515}, "\u23ce": {"f1-score": 0.687719298245614, "precision": 0.7368421052631579, "recall": 0.6447368421052632, "support": 152}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8713214079630698, "precision": 0.7939011566771819, "recall": 0.9654731457800512, "support": 782}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 244}, "macro avg": {"f1-score": 0.3522992186905061, "precision": 0.34411103464612347, "recall": 0.363959870506745, "support": 2751}, "micro avg": {"f1-score": 0.8380397157952268, "precision": 0.8400292184075968, "recall": 0.8360596146855689, "support": 2751}, "weighted avg": {"f1-score": 0.7915697804526447, "precision": 0.7533703889250427, "recall": 0.8360596146855689, "support": 2751}, "\u2205": {"f1-score": 0.9149541574454633, "precision": 0.8780339805825242, "recall": 0.9551155115511551, "support": 1515}, "\u23ce": {"f1-score": 0.6853146853146852, "precision": 0.7368421052631579, "recall": 0.6405228758169934, "support": 153}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8658256880733944, "precision": 0.7939011566771819, "recall": 0.9520807061790668, "support": 793}},
  "ppcr": 0.995274445656125
}
```
</details>
