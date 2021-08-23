# Train report for javascript / file:///tmp/top-repos-quality-repos-_zxoy3vj/streeteats.git HEAD c532ee6e58af60f3453757bf1867a52f70438b6c

### Classification report

PPCR: 0.471

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 1.000| 0.693| 0.993| 0.814| 2802| 4042| 0.693 |
| `␣` | 1.000| 0.932| 0.627| 0.965| 0.770| 2440| 3631| 0.672 |
| `⏎` | 0.927| 1.000| 0.859| 0.962| 0.892| 1492| 1736| 0.859 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.993| 0.996| 0.969| 0.995| 0.981| 847| 871| 0.972 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.976| 0.989| 0.432| 0.983| 0.599| 376| 861| 0.437 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 5224| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 512| 0.000 |
| `macro avg` | 0.697| 0.703| 0.511| 0.700| 0.579| 7957| 16877| 0.471 |
| `micro avg` | 0.978| 0.978| 0.461| 0.978| 0.627| 7957| 16877| 0.471 |
| `weighted avg` | 0.980| 0.978| 0.461| 0.978| 0.534| 7957| 16877| 0.471 |

### Confusion matrix

|refusal|  ∅| "| ␣| ⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1240 |2802 |0 |0 |0 |0 |0 |0 |
|5224 |0 |0 |0 |0 |0 |0 |0 |
|1191 |34 |0 |2275 |116 |0 |6 |9 |
|244 |0 |0 |0 |1492 |0 |0 |0 |
|512 |0 |0 |0 |0 |0 |0 |0 |
|24 |3 |0 |0 |0 |0 |844 |0 |
|485 |2 |0 |0 |2 |0 |0 |372 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| FoodTruckApp/static/FoodTruckApp/assets/js/line-chart.js | 111 |
| FoodTruckApp/static/FoodTruckApp/assets/js/scripts.js | 32 |
| FoodTruckApp/static/FoodTruckApp/assets/js/pie-chart.js | 23 |
| FoodTruckApp/static/FoodTruckApp/assets/js/bar-chart.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.6996539403337758, "precision": 0.6974713775775141, "recall": 0.7025995483821665, "support": 7957}, "micro avg": {"f1-score": 0.9783838129948473, "precision": 0.9783838129948473, "recall": 0.9783838129948473, "support": 7957}, "weighted avg": {"f1-score": 0.9783263733185922, "precision": 0.9795554994067842, "recall": 0.9783838129948473, "support": 7957}, "\u2205": {"f1-score": 0.9930887825624668, "precision": 0.986272439281943, "recall": 1.0, "support": 2802}, "\u23ce": {"f1-score": 0.961960025789813, "precision": 0.9267080745341615, "recall": 1.0, "support": 1492}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9828269484808455, "precision": 0.9763779527559056, "recall": 0.9893617021276596, "support": 376}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.99469652327637, "precision": 0.9929411764705882, "recall": 0.9964580873671782, "support": 847}, "\u2423": {"f1-score": 0.9650053022269353, "precision": 1.0, "recall": 0.9323770491803278, "support": 2440}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5224}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 512}, "macro avg": {"f1-score": 0.5794646970449556, "precision": 0.6974713775775141, "recall": 0.5114677484983845, "support": 16877}, "micro avg": {"f1-score": 0.6269630345494082, "precision": 0.9783838129948473, "recall": 0.4612786632695384, "support": 16877}, "weighted avg": {"f1-score": 0.5336552176555436, "precision": 0.6477331041652922, "recall": 0.4612786632695384, "support": 16877}, "\u2205": {"f1-score": 0.8141798634316432, "precision": 0.986272439281943, "recall": 0.6932211776348343, "support": 4042}, "\u23ce": {"f1-score": 0.8918111177525403, "precision": 0.9267080745341615, "recall": 0.8594470046082949, "support": 1736}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.5990338164251208, "precision": 0.9763779527559056, "recall": 0.43205574912891986, "support": 861}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9808251016850669, "precision": 0.9929411764705882, "recall": 0.9690011481056258, "support": 871}, "\u2423": {"f1-score": 0.7704029800203183, "precision": 1.0, "recall": 0.6265491600110162, "support": 3631}},
  "ppcr": 0.4714700479943118
}
```
</details>
