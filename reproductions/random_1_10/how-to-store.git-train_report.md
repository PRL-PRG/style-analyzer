# Train report for javascript / file:///tmp/top-repos-quality-repos-o4ptc0_e/how-to-store.git HEAD 389f5c880ffe63d793ae8cea04cf1abe6a9bf16b

### Classification report

PPCR: 0.283

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.993| 0.400| 0.989| 0.569| 1230| 3051| 0.403 |
| `"` | 1.000| 1.000| 0.655| 1.000| 0.792| 211| 322| 0.655 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.920| 0.912| 0.585| 0.916| 0.715| 113| 176| 0.642 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 193| 0.036 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1217| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 293| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 156| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 103| 0.000 |
| `weighted avg` | 0.979| 0.983| 0.279| 0.981| 0.384| 1561| 5511| 0.283 |
| `micro avg` | 0.983| 0.983| 0.279| 0.983| 0.434| 1561| 5511| 0.283 |
| `macro avg` | 0.363| 0.363| 0.205| 0.363| 0.260| 1561| 5511| 0.283 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1821 |1221 |0 |0 |0 |0 |9 |0 |0 |
|1217 |0 |0 |0 |0 |0 |0 |0 |0 |
|111 |0 |0 |211 |0 |0 |0 |0 |0 |
|293 |0 |0 |0 |0 |0 |0 |0 |0 |
|186 |7 |0 |0 |0 |0 |0 |0 |0 |
|63 |10 |0 |0 |0 |0 |103 |0 |0 |
|156 |0 |0 |0 |0 |0 |0 |0 |0 |
|103 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/components/layout/AutocompleteSearchBar.js | 7 |
| frontend/src/components/accounts/Register.js | 4 |
| frontend/src/components/food_items/ImageCard.js | 3 |
| frontend/src/actions/foodItems.js | 3 |
| frontend/src/components/layout/Alerts.js | 3 |
| frontend/src/components/accounts/Login.js | 2 |
| frontend/src/components/food_items/FoodImagesList.js | 1 |
| frontend/src/reducers/messages.js | 1 |
| frontend/src/components/common/PrivateRoute.js | 1 |
| frontend/src/store.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 211}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.36312758869079775, "precision": 0.3632388789522271, "recall": 0.36302341895100365, "support": 1561}, "micro avg": {"f1-score": 0.9833440102498399, "precision": 0.9833440102498399, "recall": 0.9833440102498399, "support": 1561}, "weighted avg": {"f1-score": 0.9811018047161398, "precision": 0.9788786018330193, "recall": 0.9833440102498399, "support": 1561}, "\u2205": {"f1-score": 0.9894651539708265, "precision": 0.9862681744749596, "recall": 0.9926829268292683, "support": 1230}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9155555555555556, "precision": 0.9196428571428571, "recall": 0.911504424778761, "support": 113}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.7917448405253283, "precision": 1.0, "recall": 0.65527950310559, "support": 322}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "macro avg": {"f1-score": 0.2595482632869556, "precision": 0.3632388789522271, "recall": 0.20508792908333598, "support": 5511}, "micro avg": {"f1-score": 0.4341063348416289, "precision": 0.9833440102498399, "recall": 0.2785338414080929, "support": 5511}, "weighted avg": {"f1-score": 0.3843147757944622, "precision": 0.6338162480820622, "recall": 0.2785338414080929, "support": 5511}, "\u2205": {"f1-score": 0.5693634879925391, "precision": 0.9862681744749596, "recall": 0.400196656833825, "support": 3051}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 293}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7152777777777778, "precision": 0.9196428571428571, "recall": 0.5852272727272727, "support": 176}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1217}},
  "ppcr": 0.2832516784612593
}
```
</details>
