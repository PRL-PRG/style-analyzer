# Train report for javascript / file:///tmp/top-repos-quality-repos-34b8yf9b/travelroute.git HEAD 63478bb0511bccf4a75f93834978f093d5b54f47

### Classification report

PPCR: 0.495

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.994| 0.755| 0.993| 0.858| 2346| 3088| 0.760 |
| `␣` | 0.923| 0.946| 0.148| 0.934| 0.255| 203| 1297| 0.157 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 116| 0.052 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 116| 0.017 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 397| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 152| 0.000 |
| `micro avg` | 0.987| 0.987| 0.489| 0.987| 0.654| 2557| 5166| 0.495 |
| `weighted avg` | 0.984| 0.987| 0.489| 0.986| 0.577| 2557| 5166| 0.495 |
| `macro avg` | 0.319| 0.323| 0.151| 0.321| 0.185| 2557| 5166| 0.495 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|742 |2332 |14 |0 |0 |0 |0 |
|1094 |11 |192 |0 |0 |0 |0 |
|397 |0 |0 |0 |0 |0 |0 |
|152 |0 |0 |0 |0 |0 |0 |
|114 |0 |2 |0 |0 |0 |0 |
|110 |6 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/build/js/location/background_map.js | 12 |
| static/build/js/lib/keydragzoom.js | 10 |
| static/build/js/location/anneal.js | 5 |
| static/build/js/location/page_related.js | 5 |
| static/build/js/location/search_place.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3212839667399962, "precision": 0.3193066334828787, "recall": 0.32330753390811595, "support": 2557}, "micro avg": {"f1-score": 0.9870942510754791, "precision": 0.9870942510754791, "recall": 0.9870942510754791, "support": 2557}, "weighted avg": {"f1-score": 0.9855980202307381, "precision": 0.9841244922765562, "recall": 0.9870942510754791, "support": 2557}, "\u2205": {"f1-score": 0.9933972310969117, "precision": 0.9927628778203491, "recall": 0.9940323955669225, "support": 2346}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.9343065693430657, "precision": 0.9230769230769231, "recall": 0.9458128078817734, "support": 203}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "macro avg": {"f1-score": 0.18549591810837965, "precision": 0.3193066334828787, "recall": 0.15053587859854614, "support": 5166}, "micro avg": {"f1-score": 0.6536320082869351, "precision": 0.9870942510754791, "recall": 0.4885791715060008, "support": 5166}, "weighted avg": {"f1-score": 0.5768284191322058, "precision": 0.8251805141192425, "recall": 0.4885791715060008, "support": 5166}, "\u2205": {"f1-score": 0.8578260069891483, "precision": 0.9927628778203491, "recall": 0.7551813471502591, "support": 3088}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 397}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u2423": {"f1-score": 0.25514950166112954, "precision": 0.9230769230769231, "recall": 0.14803392444101773, "support": 1297}},
  "ppcr": 0.49496709252806814
}
```
</details>
