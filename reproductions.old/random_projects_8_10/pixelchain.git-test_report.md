# Test report for javascript / file:///tmp/top-repos-quality-repos-usr5ditx/pixelchain.git HEAD 175f3c9ac7a8c450c677425ac4912b2fe42ebca1

### Classification report

PPCR: 0.186

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.392| 1.000| 0.563| 758| 1936| 0.392 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 194| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1941| 0.000 |
| `macro avg` | 0.333| 0.333| 0.131| 0.333| 0.188| 758| 4071| 0.186 |
| `micro avg` | 1.000| 1.000| 0.186| 1.000| 0.314| 758| 4071| 0.186 |
| `weighted avg` | 1.000| 1.000| 0.186| 1.000| 0.268| 758| 4071| 0.186 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|1178 |758 |0 |0 |
|1941 |0 |0 |0 |
|194 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 758}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 758}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 758}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 758}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"macro avg": {"f1-score": 0.18757733234347937, "precision": 0.3333333333333333, "recall": 0.13050964187327824, "support": 4071}, "micro avg": {"f1-score": 0.31393663284323875, "precision": 1.0, "recall": 0.18619503807418325, "support": 4071}, "weighted avg": {"f1-score": 0.26761217053572295, "precision": 0.4755588307541145, "recall": 0.18619503807418325, "support": 4071}, "\u2205": {"f1-score": 0.5627319970304381, "precision": 1.0, "recall": 0.3915289256198347, "support": 1936}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1941}},
  "ppcr": 0.18619503807418325
}
```
</details>
