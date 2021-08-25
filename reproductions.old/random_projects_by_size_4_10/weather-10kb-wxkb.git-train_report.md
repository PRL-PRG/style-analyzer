# Train report for javascript / file:///tmp/top-repos-quality-repos-av4ezdeu/weather-10kb-wxkb.git HEAD f3f17b417d411d50c633c25aea7ccbf624d80203

### Classification report

PPCR: 0.582

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.952| 1.000| 0.789| 0.975| 0.863| 1707| 2163| 0.789 |
| `␣` | 1.000| 0.937| 0.365| 0.968| 0.535| 350| 899| 0.389 |
| `'` | 1.000| 1.000| 0.473| 1.000| 0.642| 202| 427| 0.473 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 239| 0.109 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 116| 0.198 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 144| 0.104 |
| `macro avg` | 0.492| 0.490| 0.271| 0.490| 0.340| 2323| 3988| 0.582 |
| `weighted avg` | 0.937| 0.963| 0.561| 0.950| 0.657| 2323| 3988| 0.582 |
| `micro avg` | 0.963| 0.963| 0.561| 0.963| 0.709| 2323| 3988| 0.582 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|456 |1707 |0 |0 |0 |0 |0 |
|549 |22 |328 |0 |0 |0 |0 |
|225 |0 |0 |202 |0 |0 |0 |
|213 |26 |0 |0 |0 |0 |0 |
|129 |15 |0 |0 |0 |0 |0 |
|93 |23 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/integration/forecast.test.js | 23 |
| public/sw.js | 12 |
| src/router.js | 12 |
| src/modules/WeatherRequest.js | 11 |
| test/unit/TenonRequest.test.js | 10 |
| test/integration/app.test.js | 7 |
| src/modules/TenonRequest.js | 3 |
| test/unit/routerHelper.test.js | 3 |
| public/sw-registration.js | 2 |
| src/app.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 202}, "macro avg": {"f1-score": 0.4904966989745751, "precision": 0.49200594906116385, "recall": 0.4895238095238095, "support": 2323}, "micro avg": {"f1-score": 0.9629789065863108, "precision": 0.9629789065863108, "recall": 0.9629789065863108, "support": 2323}, "weighted avg": {"f1-score": 0.9495047952110112, "precision": 0.9372040164806026, "recall": 0.9629789065863108, "support": 2323}, "\u2205": {"f1-score": 0.9754285714285714, "precision": 0.9520356943669828, "recall": 1.0, "support": 1707}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.967551622418879, "precision": 1.0, "recall": 0.9371428571428572, "support": 350}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6422893481717011, "precision": 1.0, "recall": 0.47306791569086654, "support": 427}, "macro avg": {"f1-score": 0.33998659952144106, "precision": 0.49200594906116385, "recall": 0.27118324015552037, "support": 3988}, "micro avg": {"f1-score": 0.7089209317065441, "precision": 0.9629789065863108, "recall": 0.5609327983951855, "support": 3988}, "weighted avg": {"f1-score": 0.657359628653735, "precision": 0.8488598813730651, "recall": 0.5609327983951855, "support": 3988}, "\u2205": {"f1-score": 0.8629929221435795, "precision": 0.9520356943669828, "recall": 0.7891816920943134, "support": 2163}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 239}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 144}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u2423": {"f1-score": 0.534637326813366, "precision": 1.0, "recall": 0.36484983314794217, "support": 899}},
  "ppcr": 0.5824974924774323
}
```
</details>
