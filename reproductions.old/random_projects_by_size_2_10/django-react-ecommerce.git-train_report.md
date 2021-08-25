# Train report for javascript / file:///tmp/top-repos-quality-repos-f2h598wj/django-react-ecommerce.git HEAD bfbe7a7138296aa2a243ee4fc46bce31e511ac91

### Classification report

PPCR: 0.402

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 1.000| 0.560| 0.974| 0.704| 4508| 8050| 0.560 |
| `␣` | 0.951| 0.847| 0.199| 0.896| 0.329| 686| 2924| 0.235 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 50| 205| 0.244 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 220| 0.155 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 182| 0.176 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 569| 0.053 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 188| 0.112 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 699| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 163| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 122| 0.000 |
| `macro avg` | 0.190| 0.185| 0.076| 0.187| 0.103| 5361| 13322| 0.402 |
| `micro avg` | 0.949| 0.949| 0.382| 0.949| 0.545| 5361| 13322| 0.402 |
| `weighted avg` | 0.920| 0.949| 0.382| 0.934| 0.498| 5361| 13322| 0.402 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3542 |4508 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2238 |105 |581 |0 |0 |0 |0 |0 |0 |0 |0 |
|699 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|539 |29 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|167 |10 |11 |0 |0 |0 |0 |0 |0 |0 |0 |
|150 |32 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|186 |18 |16 |0 |0 |0 |0 |0 |0 |0 |0 |
|155 |48 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|163 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|122 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/containers/Profile.js | 64 |
| src/containers/Checkout.js | 37 |
| src/registerServiceWorker.js | 24 |
| src/containers/Home.js | 18 |
| src/store/actions/auth.js | 17 |
| src/containers/OrderSummary.js | 17 |
| src/containers/ProductDetail.js | 16 |
| src/containers/Layout.js | 15 |
| src/containers/Signup.js | 14 |
| src/containers/Login.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.18697740918979325, "precision": 0.18999527952450684, "recall": 0.18469387755102043, "support": 5361}, "micro avg": {"f1-score": 0.9492631971647081, "precision": 0.9492631971647081, "recall": 0.9492631971647081, "support": 5361}, "weighted avg": {"f1-score": 0.9335496451987352, "precision": 0.9197251959397228, "recall": 0.9492631971647081, "support": 5361}, "\u2205": {"f1-score": 0.9738604450205228, "precision": 0.9490526315789474, "recall": 1.0, "support": 4508}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.8959136468774095, "precision": 0.9509001636661211, "recall": 0.8469387755102041, "support": 686}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 699}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "macro avg": {"f1-score": 0.10330878712871287, "precision": 0.18999527952450684, "recall": 0.07587004103967168, "support": 13322}, "micro avg": {"f1-score": 0.5447733233420757, "precision": 0.9492631971647081, "recall": 0.38199969974478304, "support": 13322}, "weighted avg": {"f1-score": 0.49777624873469184, "precision": 0.782187791830826, "recall": 0.38199969974478304, "support": 13322}, "\u2205": {"f1-score": 0.7043750000000001, "precision": 0.9490526315789474, "recall": 0.56, "support": 8050}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 569}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 220}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 188}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 205}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "\u2423": {"f1-score": 0.32871287128712867, "precision": 0.9509001636661211, "recall": 0.19870041039671682, "support": 2924}},
  "ppcr": 0.40241705449632187
}
```
</details>
