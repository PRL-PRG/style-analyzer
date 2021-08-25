# Test report for javascript / file:///tmp/top-repos-quality-repos-61jkviud/freecome.git HEAD b7866f447c40a5687baefc13ba33244f97d62392

### Classification report

PPCR: 0.475

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.677| 1.000| 0.503| 0.808| 0.577| 86| 171| 0.503 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 114| 0.430 |
| `"` | 0.467| 1.000| 0.500| 0.636| 0.483| 7| 14| 0.500 |
| `micro avg` | 0.655| 0.655| 0.311| 0.655| 0.422| 142| 299| 0.475 |
| `weighted avg` | 0.433| 0.655| 0.311| 0.520| 0.353| 142| 299| 0.475 |
| `macro avg` | 0.381| 0.667| 0.334| 0.481| 0.353| 142| 299| 0.475 |

### Confusion matrix

|refusal|  ∅| "| ␣| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|85 |86 |0 |0 |
|7 |0 |7 |0 |
|65 |41 |8 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.6363636363636364, "precision": 0.4666666666666667, "recall": 1.0, "support": 7}, "macro avg": {"f1-score": 0.48129179115094606, "precision": 0.38127734033245847, "recall": 0.6666666666666666, "support": 142}, "micro avg": {"f1-score": 0.6549295774647887, "precision": 0.6549295774647887, "recall": 0.6549295774647887, "support": 142}, "weighted avg": {"f1-score": 0.5204264425649071, "precision": 0.4331189235148423, "recall": 0.6549295774647887, "support": 142}, "\u2205": {"f1-score": 0.8075117370892019, "precision": 0.6771653543307087, "recall": 1.0, "support": 86}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}},
  "cl_report_full": {"\"": {"f1-score": 0.4827586206896552, "precision": 0.4666666666666667, "recall": 0.5, "support": 14}, "macro avg": {"f1-score": 0.3533132762477822, "precision": 0.38127734033245847, "recall": 0.33430799220272905, "support": 299}, "micro avg": {"f1-score": 0.4217687074829932, "precision": 0.6549295774647887, "recall": 0.3110367892976589, "support": 299}, "weighted avg": {"f1-score": 0.3526976831666769, "precision": 0.40912578235412883, "recall": 0.3110367892976589, "support": 299}, "\u2205": {"f1-score": 0.5771812080536913, "precision": 0.6771653543307087, "recall": 0.5029239766081871, "support": 171}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}},
  "ppcr": 0.47491638795986624
}
```
</details>
