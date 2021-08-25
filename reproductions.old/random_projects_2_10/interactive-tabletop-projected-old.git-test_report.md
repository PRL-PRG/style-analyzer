# Test report for javascript / file:///tmp/top-repos-quality-repos-syesqkv9/interactive-tabletop-projected-old.git HEAD 9456d214c4116020ae10bd093fee88e3d5a897a4

### Classification report

PPCR: 0.947

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.905| 0.958| 0.910| 0.931| 0.908| 190| 200| 0.950 |
| `␣` | 0.774| 0.964| 0.930| 0.858| 0.845| 110| 114| 0.965 |
| `⏎` | 0.190| 0.148| 0.148| 0.167| 0.167| 27| 27| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 20| 0.850 |
| `⏎␣⁺␣⁺` | 0.875| 0.583| 0.583| 0.700| 0.700| 12| 12| 1.000 |
| `⏎␣⁻␣⁻` | 1.000| 0.750| 0.750| 0.857| 0.857| 12| 12| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 12| 0.667 |
| `macro avg` | 0.535| 0.486| 0.474| 0.502| 0.497| 376| 397| 0.947 |
| `weighted avg` | 0.757| 0.819| 0.776| 0.783| 0.758| 376| 397| 0.947 |
| `micro avg` | 0.819| 0.819| 0.776| 0.819| 0.797| 376| 397| 0.947 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|10 |182 |8 |0 |0 |0 |0 |0 |
|4 |4 |106 |0 |0 |0 |0 |0 |
|0 |4 |18 |4 |1 |0 |0 |0 |
|0 |0 |5 |0 |7 |0 |0 |0 |
|0 |3 |0 |0 |0 |9 |0 |0 |
|4 |8 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |17 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "macro avg": {"f1-score": 0.5018650586444753, "precision": 0.534953065004191, "recall": 0.4861446545657072, "support": 376}, "micro avg": {"f1-score": 0.8191489361702128, "precision": 0.8191489361702128, "recall": 0.8191489361702128, "support": 376}, "weighted avg": {"f1-score": 0.783187407307768, "precision": 0.7574259233749386, "recall": 0.8191489361702128, "support": 376}, "\u2205": {"f1-score": 0.9309462915601023, "precision": 0.9054726368159204, "recall": 0.9578947368421052, "support": 190}, "\u23ce": {"f1-score": 0.16666666666666666, "precision": 0.19047619047619047, "recall": 0.14814814814814814, "support": 27}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7000000000000001, "precision": 0.875, "recall": 0.5833333333333334, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8571428571428571, "precision": 1.0, "recall": 0.75, "support": 12}, "\u2423": {"f1-score": 0.8582995951417005, "precision": 0.7737226277372263, "recall": 0.9636363636363636, "support": 110}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "macro avg": {"f1-score": 0.49659453015292226, "precision": 0.534953065004191, "recall": 0.4744722918407129, "support": 397}, "micro avg": {"f1-score": 0.796895213454075, "precision": 0.8191489361702128, "recall": 0.7758186397984886, "support": 397}, "weighted avg": {"f1-score": 0.7582335051352579, "precision": 0.7479641412294333, "recall": 0.7758186397984886, "support": 397}, "\u2205": {"f1-score": 0.9077306733167083, "precision": 0.9054726368159204, "recall": 0.91, "support": 200}, "\u23ce": {"f1-score": 0.16666666666666666, "precision": 0.19047619047619047, "recall": 0.14814814814814814, "support": 27}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7000000000000001, "precision": 0.875, "recall": 0.5833333333333334, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8571428571428571, "precision": 1.0, "recall": 0.75, "support": 12}, "\u2423": {"f1-score": 0.8446215139442232, "precision": 0.7737226277372263, "recall": 0.9298245614035088, "support": 114}},
  "ppcr": 0.947103274559194
}
```
</details>
