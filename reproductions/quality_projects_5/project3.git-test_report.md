# Test report for javascript / file:///tmp/top-repos-quality-repos-vwwtzv2c/project3.git HEAD 5cdbd162477789cd187071a8c056d2f1531018d3

### Classification report

PPCR: 0.795

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.891| 0.978| 0.893| 0.932| 0.892| 1294| 1417| 0.913 |
| `␣` | 0.867| 0.743| 0.613| 0.800| 0.718| 448| 543| 0.825 |
| `"` | 0.815| 1.000| 0.500| 0.898| 0.620| 101| 202| 0.500 |
| `⏎` | 0.946| 0.522| 0.278| 0.673| 0.429| 67| 126| 0.532 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 46| 0.500 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 59| 0.271 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 31| 0.258 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 49| 0.163 |
| `micro avg` | 0.882| 0.882| 0.701| 0.882| 0.781| 1965| 2473| 0.795 |
| `weighted avg` | 0.858| 0.882| 0.701| 0.865| 0.741| 1965| 2473| 0.795 |
| `macro avg` | 0.440| 0.405| 0.285| 0.413| 0.332| 1965| 2473| 0.795 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|123 |1265 |29 |0 |0 |0 |0 |0 |0 |
|95 |113 |333 |0 |2 |0 |0 |0 |0 |
|101 |0 |0 |101 |0 |0 |0 |0 |0 |
|59 |17 |15 |0 |35 |0 |0 |0 |0 |
|43 |15 |1 |0 |0 |0 |0 |0 |0 |
|41 |6 |2 |0 |0 |0 |0 |0 |0 |
|23 |0 |0 |23 |0 |0 |0 |0 |0 |
|23 |4 |4 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8977777777777779, "precision": 0.8145161290322581, "recall": 1.0, "support": 101}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "macro avg": {"f1-score": 0.41294235748949737, "precision": 0.43981183067509244, "recall": 0.4054100628557093, "support": 1965}, "micro avg": {"f1-score": 0.8824427480916031, "precision": 0.8824427480916031, "recall": 0.8824427480916031, "support": 1965}, "weighted avg": {"f1-score": 0.8654749518869068, "precision": 0.8584722791538917, "recall": 0.8824427480916031, "support": 1965}, "\u2205": {"f1-score": 0.9322033898305085, "precision": 0.8908450704225352, "recall": 0.9775888717156105, "support": 1294}, "\u23ce": {"f1-score": 0.673076923076923, "precision": 0.9459459459459459, "recall": 0.5223880597014925, "support": 67}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.8004807692307693, "precision": 0.8671875, "recall": 0.7433035714285714, "support": 448}},
  "cl_report_full": {"\"": {"f1-score": 0.6196319018404909, "precision": 0.8145161290322581, "recall": 0.5, "support": 202}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "macro avg": {"f1-score": 0.33241418194890815, "precision": 0.43981183067509244, "recall": 0.28547107104687314, "support": 2473}, "micro avg": {"f1-score": 0.7814330779630464, "precision": 0.8824427480916031, "recall": 0.7011726647796199, "support": 2473}, "weighted avg": {"f1-score": 0.7412272130309052, "precision": 0.8155809642306663, "recall": 0.7011726647796199, "support": 2473}, "\u2205": {"f1-score": 0.8917870990482906, "precision": 0.8908450704225352, "recall": 0.8927311220889202, "support": 1417}, "\u23ce": {"f1-score": 0.4294478527607362, "precision": 0.9459459459459459, "recall": 0.2777777777777778, "support": 126}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u2423": {"f1-score": 0.7184466019417476, "precision": 0.8671875, "recall": 0.6132596685082873, "support": 543}},
  "ppcr": 0.7945814799838253
}
```
</details>