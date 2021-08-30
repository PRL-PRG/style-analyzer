# Train report for javascript / file:///tmp/top-repos-quality-repos-ax5dqr3q/jso.git HEAD b1be102c5c8eedb15c38dea2ba56d83b749eed94

### Classification report

PPCR: 0.501

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 0.998| 0.876| 0.984| 0.921| 2509| 2858| 0.878 |
| `␣` | 0.979| 0.856| 0.153| 0.914| 0.264| 278| 1558| 0.178 |
| `⏎` | 0.746| 0.688| 0.115| 0.715| 0.200| 64| 381| 0.168 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 139| 0.151 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 147| 0.034 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 164| 0.024 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 212| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 288| 0.000 |
| `micro avg` | 0.967| 0.967| 0.485| 0.967| 0.646| 2881| 5747| 0.501 |
| `weighted avg` | 0.957| 0.967| 0.485| 0.961| 0.543| 2881| 5747| 0.501 |
| `macro avg` | 0.337| 0.318| 0.143| 0.327| 0.173| 2881| 5747| 0.501 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|349 |2504 |5 |0 |0 |0 |0 |0 |0 |
|1280 |40 |238 |0 |0 |0 |0 |0 |0 |
|317 |20 |0 |44 |0 |0 |0 |0 |0 |
|212 |0 |0 |0 |0 |0 |0 |0 |0 |
|288 |0 |0 |0 |0 |0 |0 |0 |0 |
|118 |11 |0 |10 |0 |0 |0 |0 |0 |
|160 |3 |0 |1 |0 |0 |0 |0 |0 |
|142 |1 |0 |4 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/utils.js | 22 |
| src/JSO.js | 21 |
| src/Authentication/Authentication.js | 13 |
| test/tests.js | 10 |
| src/HTTP/Fetcher.js | 8 |
| src/Loaders/IFramePassive.js | 6 |
| src/store.js | 6 |
| src/Loaders/HTTPRedirect.js | 2 |
| src/Loaders/Popup.js | 2 |
| webpack.config.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.32666894039834543, "precision": 0.33701319262683727, "recall": 0.3177027852608308, "support": 2881}, "micro avg": {"f1-score": 0.9670253384241583, "precision": 0.9670253384241583, "recall": 0.9670253384241583, "support": 2881}, "weighted avg": {"f1-score": 0.9612382561738315, "precision": 0.9566276715347592, "recall": 0.9670253384241583, "support": 2881}, "\u2205": {"f1-score": 0.9842767295597484, "precision": 0.9709189608375339, "recall": 0.9980071741729772, "support": 2509}, "\u23ce": {"f1-score": 0.7154471544715446, "precision": 0.7457627118644068, "recall": 0.6875, "support": 64}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.9136276391554703, "precision": 0.9794238683127572, "recall": 0.8561151079136691, "support": 278}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 288}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 212}, "macro avg": {"f1-score": 0.1731742256488614, "precision": 0.33701319262683727, "recall": 0.14304783397611553, "support": 5747}, "micro avg": {"f1-score": 0.645804357904497, "precision": 0.9670253384241583, "recall": 0.48477466504263095, "support": 5747}, "weighted avg": {"f1-score": 0.5429734816542148, "precision": 0.7978013520315445, "recall": 0.48477466504263095, "support": 5747}, "\u2205": {"f1-score": 0.9210961927533566, "precision": 0.9709189608375339, "recall": 0.8761371588523443, "support": 2858}, "\u23ce": {"f1-score": 0.2, "precision": 0.7457627118644068, "recall": 0.11548556430446194, "support": 381}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 147}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 139}, "\u2423": {"f1-score": 0.2642976124375347, "precision": 0.9794238683127572, "recall": 0.1527599486521181, "support": 1558}},
  "ppcr": 0.5013050287106317
}
```
</details>
