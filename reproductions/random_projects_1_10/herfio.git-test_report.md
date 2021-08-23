# Test report for javascript / file:///tmp/top-repos-quality-repos-6coxyta1/herfio.git HEAD 94974b504cd60dd0b9e1bd0def1d183b89c4f41b

### Classification report

PPCR: 0.586

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.876| 1.000| 0.738| 0.934| 0.801| 1287| 1745| 0.738 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 146| 543| 0.269 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 90| 180| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 194| 0.191 |
| `micro avg` | 0.883| 0.883| 0.517| 0.883| 0.652| 1560| 2662| 0.586 |
| `weighted avg` | 0.780| 0.883| 0.517| 0.828| 0.570| 1560| 2662| 0.586 |
| `macro avg` | 0.469| 0.500| 0.309| 0.483| 0.367| 1560| 2662| 0.586 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|458 |1287 |0 |0 |0 |
|397 |146 |0 |0 |0 |
|90 |0 |0 |90 |0 |
|157 |37 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 90}, "macro avg": {"f1-score": 0.4834058759521218, "precision": 0.4688775510204082, "recall": 0.5, "support": 1560}, "micro avg": {"f1-score": 0.8826923076923077, "precision": 0.8826923076923077, "recall": 0.8826923076923077, "support": 1560}, "weighted avg": {"f1-score": 0.8279316983343098, "precision": 0.7799882260596547, "recall": 0.8826923076923077, "support": 1560}, "\u2205": {"f1-score": 0.9336235038084874, "precision": 0.8755102040816326, "recall": 1.0, "support": 1287}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 180}, "macro avg": {"f1-score": 0.36682218766200103, "precision": 0.4688775510204082, "recall": 0.3093839541547278, "support": 2662}, "micro avg": {"f1-score": 0.6522974893415443, "precision": 0.8826923076923077, "recall": 0.5172802404207363, "support": 2662}, "weighted avg": {"f1-score": 0.5699044089208993, "precision": 0.6415346754780048, "recall": 0.5172802404207363, "support": 2662}, "\u2205": {"f1-score": 0.8006220839813375, "precision": 0.8755102040816326, "recall": 0.7375358166189112, "support": 1745}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 543}},
  "ppcr": 0.5860255447032306
}
```
</details>
