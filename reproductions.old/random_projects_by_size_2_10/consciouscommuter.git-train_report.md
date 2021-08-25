# Train report for javascript / file:///tmp/top-repos-quality-repos-7n3puaqx/consciouscommuter.git HEAD a079735bfcde0fadcb9a29b4d283328900b786d0

### Classification report

PPCR: 0.258

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 1.000| 0.408| 0.987| 0.575| 1722| 4223| 0.408 |
| `⏎` | 0.972| 0.995| 0.411| 0.984| 0.578| 213| 516| 0.413 |
| `"` | 1.000| 1.000| 0.319| 1.000| 0.484| 141| 442| 0.319 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 157| 0.178 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 2120| 0.011 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 615| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 172| 0.000 |
| `macro avg` | 0.421| 0.428| 0.163| 0.424| 0.234| 2127| 8245| 0.258 |
| `micro avg` | 0.976| 0.976| 0.252| 0.976| 0.400| 2127| 8245| 0.258 |
| `weighted avg` | 0.952| 0.976| 0.252| 0.964| 0.357| 2127| 8245| 0.258 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2501 |1722 |0 |0 |0 |0 |0 |0 |
|2097 |18 |0 |0 |5 |0 |0 |0 |
|615 |0 |0 |0 |0 |0 |0 |0 |
|303 |1 |0 |0 |212 |0 |0 |0 |
|301 |0 |0 |0 |0 |141 |0 |0 |
|172 |0 |0 |0 |0 |0 |0 |0 |
|129 |27 |0 |0 |1 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| conscious_commuter/frontend/src/Viz/stackedArea.js | 9 |
| conscious_commuter/frontend/src/Frontpage/register.js | 8 |
| conscious_commuter/frontend/src/Viz/progresschart.js | 7 |
| conscious_commuter/frontend/src/App.js | 6 |
| conscious_commuter/frontend/src/Frontpage/login.js | 6 |
| conscious_commuter/frontend/src/Frontpage/header.js | 4 |
| conscious_commuter/frontend/src/Frontpage/goals.js | 3 |
| conscious_commuter/frontend/src/DataPage/datapage.js | 3 |
| conscious_commuter/frontend/src/TripPage/tripmodal.js | 2 |
| conscious_commuter/frontend/src/Viz/piechart.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 141}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4243683121338205, "precision": 0.4209227092389532, "recall": 0.42790073775989274, "support": 2127}, "micro avg": {"f1-score": 0.9755524212505877, "precision": 0.9755524212505877, "recall": 0.9755524212505877, "support": 2127}, "weighted avg": {"f1-score": 0.9637253197511632, "precision": 0.9522023729469132, "recall": 0.9755524212505877, "support": 2127}, "\u2205": {"f1-score": 0.9868194842406877, "precision": 0.9739819004524887, "recall": 1.0, "support": 1722}, "\u23ce": {"f1-score": 0.9837587006960558, "precision": 0.9724770642201835, "recall": 0.9953051643192489, "support": 213}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}},
  "cl_report_full": {"\"": {"f1-score": 0.483704974271012, "precision": 1.0, "recall": 0.3190045248868778, "support": 442}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 615}, "macro avg": {"f1-score": 0.233746277637213, "precision": 0.4209227092389532, "recall": 0.16251774690806206, "support": 8245}, "micro avg": {"f1-score": 0.40011569610489783, "precision": 0.9755524212505877, "recall": 0.25166767738023044, "support": 8245}, "weighted avg": {"f1-score": 0.35652054681773593, "precision": 0.6133321686778016, "recall": 0.25166767738023044, "support": 8245}, "\u2205": {"f1-score": 0.5748622934401603, "precision": 0.9739819004524887, "recall": 0.4077669902912621, "support": 4223}, "\u23ce": {"f1-score": 0.5776566757493189, "precision": 0.9724770642201835, "recall": 0.4108527131782946, "support": 516}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2120}},
  "ppcr": 0.25797453001819287
}
```
</details>
