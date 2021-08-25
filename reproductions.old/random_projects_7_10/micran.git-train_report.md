# Train report for javascript / file:///tmp/top-repos-quality-repos-n93mvyi0/micran.git HEAD 4c7ac3b5bba8bf184dc99348195a31751b39e714

### Classification report

PPCR: 0.534

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 1.000| 0.715| 0.988| 0.826| 13782| 19283| 0.715 |
| `"` | 0.954| 1.000| 0.686| 0.977| 0.798| 1382| 2016| 0.686 |
| `⏎` | 0.956| 0.983| 0.193| 0.969| 0.321| 286| 1455| 0.197 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 273| 4522| 0.060 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 66| 1425| 0.046 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 392| 0.120 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 460| 0.015 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 129| 0.039 |
| `macro avg` | 0.361| 0.373| 0.199| 0.367| 0.243| 15848| 29682| 0.534 |
| `micro avg` | 0.975| 0.975| 0.520| 0.975| 0.678| 15848| 29682| 0.534 |
| `weighted avg` | 0.950| 0.975| 0.520| 0.962| 0.606| 15848| 29682| 0.534 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5501 |13782 |0 |0 |0 |0 |0 |0 |0 |
|4249 |273 |0 |0 |0 |0 |0 |0 |0 |
|634 |0 |0 |1382 |0 |0 |0 |0 |0 |
|1169 |5 |0 |0 |281 |0 |0 |0 |0 |
|1359 |0 |0 |66 |0 |0 |0 |0 |0 |
|453 |3 |0 |0 |4 |0 |0 |0 |0 |
|345 |43 |0 |0 |4 |0 |0 |0 |0 |
|124 |0 |0 |0 |5 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/ Components/ PersonCabinet/MainPage/Main.js | 106 |
| client/src/ Components/ PersonCabinet/ListReports/ListReports.js | 48 |
| client/src/ Components/ PersonCabinet/SendReport/SendReport.js | 27 |
| client/src/App.js | 26 |
| client/src/ Components/ PersonCabinet/Payroll/Payroll.js | 20 |
| client/src/ Components/ PersonCabinet/WorkCalendar/Interval/Interval.js | 16 |
| client/src/ Components/ PersonCabinet/SendReport/Reports/ProjectList/ProjectList.js | 15 |
| client/src/ Components/ PersonCabinet/Employees/Employees.js | 14 |
| client/src/ Components/ PersonCabinet/SendReport/Reports/Reports.js | 13 |
| client/src/ Components/ PersonCabinet/SystemTime/SystemTime.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.976678445229682, "precision": 0.9544198895027625, "recall": 1.0, "support": 1382}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "macro avg": {"f1-score": 0.366753257787368, "precision": 0.36090415663058645, "recall": 0.3728146853146853, "support": 15848}, "micro avg": {"f1-score": 0.9745709237758707, "precision": 0.9745709237758707, "recall": 0.9745709237758707, "support": 15848}, "weighted avg": {"f1-score": 0.9621892888103444, "precision": 0.9501390691815741, "recall": 0.9745709237758707, "support": 15848}, "\u2205": {"f1-score": 0.9883820998278829, "precision": 0.9770310506167589, "recall": 1.0, "support": 13782}, "\u23ce": {"f1-score": 0.9689655172413794, "precision": 0.95578231292517, "recall": 0.9825174825174825, "support": 286}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 273}},
  "cl_report_full": {"\"": {"f1-score": 0.7979214780600462, "precision": 0.9544198895027625, "recall": 0.685515873015873, "support": 2016}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1425}, "macro avg": {"f1-score": 0.2430986620352718, "precision": 0.36090415663058645, "recall": 0.1991707292028152, "support": 29682}, "micro avg": {"f1-score": 0.6784537667471997, "precision": 0.9745709237758707, "recall": 0.520349033084024, "support": 29682}, "weighted avg": {"f1-score": 0.6062614881382364, "precision": 0.7464073684922395, "recall": 0.520349033084024, "support": 29682}, "\u2205": {"f1-score": 0.8255413459522597, "precision": 0.9770310506167589, "recall": 0.7147228128403257, "support": 19283}, "\u23ce": {"f1-score": 0.3213264722698685, "precision": 0.95578231292517, "recall": 0.19312714776632303, "support": 1455}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 129}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 460}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 392}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4522}},
  "ppcr": 0.5339262852907486
}
```
</details>
