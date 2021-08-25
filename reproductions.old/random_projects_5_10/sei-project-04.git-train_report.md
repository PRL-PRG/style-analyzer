# Train report for javascript / file:///tmp/top-repos-quality-repos-kli0n8xe/sei-project-04.git HEAD dfb3484a5f1667d1855d6413dc40b0fae7362bbc

### Classification report

PPCR: 0.641

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.966| 0.994| 0.772| 0.980| 0.858| 6110| 7867| 0.777 |
| `'` | 1.000| 1.000| 0.988| 1.000| 0.994| 1377| 1394| 0.988 |
| `␣` | 0.906| 0.903| 0.274| 0.905| 0.421| 837| 2757| 0.304 |
| `⏎⇥⁻` | 0.956| 0.652| 0.586| 0.775| 0.726| 299| 333| 0.898 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 722| 0.058 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 320| 0.106 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 187| 0.005 |
| `weighted avg` | 0.957| 0.965| 0.618| 0.960| 0.702| 8700| 13580| 0.641 |
| `macro avg` | 0.547| 0.507| 0.374| 0.523| 0.428| 8700| 13580| 0.641 |
| `micro avg` | 0.965| 0.965| 0.618| 0.965| 0.754| 8700| 13580| 0.641 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁻| ⏎⇥⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1757 |6071 |35 |0 |0 |4 |0 |0 |
|1920 |76 |756 |0 |0 |5 |0 |0 |
|17 |0 |0 |1377 |0 |0 |0 |0 |
|680 |31 |11 |0 |0 |0 |0 |0 |
|34 |104 |0 |0 |0 |195 |0 |0 |
|286 |3 |31 |0 |0 |0 |0 |0 |
|186 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/components/common/payment/Payment.js | 32 |
| frontend/src/components/events/EventDisplay.js | 31 |
| frontend/src/App.js | 21 |
| frontend/src/components/events/EventForm.js | 20 |
| frontend/src/components/user/UserForm.js | 20 |
| frontend/src/components/auth/Register.js | 20 |
| frontend/src/components/groups/GroupCard.js | 19 |
| frontend/src/components/common/NavBar.js | 13 |
| frontend/src/serviceWorker.js | 12 |
| frontend/src/components/sports/SportDisplay.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1377}, "macro avg": {"f1-score": 0.5228262647229859, "precision": 0.546901121328324, "recall": 0.5070023915388123, "support": 8700}, "micro avg": {"f1-score": 0.9654022988505748, "precision": 0.9654022988505747, "recall": 0.9654022988505747, "support": 8700}, "weighted avg": {"f1-score": 0.9599393447540707, "precision": 0.9567226298451188, "recall": 0.9654022988505747, "support": 8700}, "\u2205": {"f1-score": 0.9795885437676484, "precision": 0.9659506762132061, "recall": 0.9936170212765958, "support": 6110}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u21e5\u207b": {"f1-score": 0.7753479125248509, "precision": 0.9558823529411765, "recall": 0.6521739130434783, "support": 299}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9048473967684021, "precision": 0.9064748201438849, "recall": 0.9032258064516129, "support": 837}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9938650306748467, "precision": 1.0, "recall": 0.9878048780487805, "support": 1394}, "macro avg": {"f1-score": 0.4284493214793602, "precision": 0.546901121328324, "recall": 0.37418659306337876, "support": 13580}, "micro avg": {"f1-score": 0.7539497307001797, "precision": 0.9654022988505747, "recall": 0.6184830633284242, "support": 13580}, "weighted avg": {"f1-score": 0.7023408160447967, "precision": 0.8697049979702058, "recall": 0.6184830633284242, "support": 13580}, "\u2205": {"f1-score": 0.8579706048615038, "precision": 0.9659506762132061, "recall": 0.7717045887886107, "support": 7867}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 722}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 320}, "\u23ce\u21e5\u207b": {"f1-score": 0.7262569832402236, "precision": 0.9558823529411765, "recall": 0.5855855855855856, "support": 333}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u2423": {"f1-score": 0.42105263157894735, "precision": 0.9064748201438849, "recall": 0.27421109902067464, "support": 2757}},
  "ppcr": 0.6406480117820325
}
```
</details>
