# Train report for javascript / file:///tmp/top-repos-quality-repos-994dmf85/react-antd-admin-template.git HEAD 096a1d5bd8bb140b6fe74ab5905d2f3d86399308

### Classification report

PPCR: 0.303

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 0.996| 0.609| 0.994| 0.755| 2204| 3607| 0.611 |
| `␣` | 0.976| 0.994| 0.131| 0.985| 0.231| 322| 2438| 0.132 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 311| 0.026 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 664| 0.006 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 333| 0.009 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 676| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 267| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 91| 0.000 |
| `micro avg` | 0.990| 0.990| 0.300| 0.990| 0.460| 2541| 8387| 0.303 |
| `macro avg` | 0.246| 0.249| 0.093| 0.247| 0.123| 2541| 8387| 0.303 |
| `weighted avg` | 0.984| 0.990| 0.300| 0.987| 0.392| 2541| 8387| 0.303 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1403 |2196 |8 |0 |0 |0 |0 |0 |0 |
|2116 |2 |320 |0 |0 |0 |0 |0 |0 |
|660 |4 |0 |0 |0 |0 |0 |0 |0 |
|676 |0 |0 |0 |0 |0 |0 |0 |0 |
|267 |0 |0 |0 |0 |0 |0 |0 |0 |
|330 |3 |0 |0 |0 |0 |0 |0 |0 |
|303 |8 |0 |0 |0 |0 |0 |0 |0 |
|91 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/store/actions/auth.js | 9 |
| src/router/index.js | 5 |
| src/store/actions/user.js | 4 |
| src/App.js | 2 |
| src/lib/Export2Excel.js | 2 |
| src/mock/table.js | 1 |
| src/views/guide/steps.js | 1 |
| src/lib/Export2Zip.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24736942930286826, "precision": 0.24599098453704826, "recall": 0.24876988197630506, "support": 2541}, "micro avg": {"f1-score": 0.9901613537977174, "precision": 0.9901613537977174, "recall": 0.9901613537977174, "support": 2541}, "weighted avg": {"f1-score": 0.9872379471161038, "precision": 0.9843429666987898, "recall": 0.9901613537977174, "support": 2541}, "\u2205": {"f1-score": 0.9943400498075616, "precision": 0.9923181201988251, "recall": 0.9963702359346642, "support": 2204}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.9846153846153846, "precision": 0.975609756097561, "recall": 0.9937888198757764, "support": 322}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 676}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 267}, "macro avg": {"f1-score": 0.1232525288667248, "precision": 0.24599098453704826, "recall": 0.09250891473670397, "support": 8387}, "micro avg": {"f1-score": 0.4604685212298682, "precision": 0.9901613537977174, "recall": 0.2999880767855014, "support": 8387}, "weighted avg": {"f1-score": 0.39180762118670437, "precision": 0.7103646172556356, "recall": 0.2999880767855014, "support": 8387}, "\u2205": {"f1-score": 0.7546391752577319, "precision": 0.9923181201988251, "recall": 0.6088161907402273, "support": 3607}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 664}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 333}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 311}, "\u2423": {"f1-score": 0.23138105567606654, "precision": 0.975609756097561, "recall": 0.13125512715340443, "support": 2438}},
  "ppcr": 0.30296888041015857
}
```
</details>
