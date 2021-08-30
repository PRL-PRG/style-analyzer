# Train report for javascript / file:///tmp/top-repos-quality-repos-zlgsufzr/artillery.git HEAD d3cb703f2720a3902a0d1af2454717b6e5a41074

### Classification report

PPCR: 0.696

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.999| 0.974| 0.986| 0.973| 24053| 24672| 0.975 |
| `'` | 1.000| 1.000| 0.912| 1.000| 0.954| 2838| 3113| 0.912 |
| `␣` | 0.986| 0.855| 0.208| 0.916| 0.343| 2638| 10865| 0.243 |
| `⏎␣⁺␣⁺` | 0.991| 0.910| 0.717| 0.949| 0.832| 1177| 1494| 0.788 |
| `⏎` | 0.917| 0.774| 0.139| 0.839| 0.241| 398| 2221| 0.179 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 84| 1402| 0.060 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 43| 1134| 0.038 |
| `micro avg` | 0.976| 0.976| 0.679| 0.976| 0.801| 31231| 44901| 0.696 |
| `macro avg` | 0.695| 0.648| 0.421| 0.670| 0.478| 31231| 44901| 0.696 |
| `weighted avg` | 0.972| 0.976| 0.679| 0.974| 0.724| 31231| 44901| 0.696 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|619 |24020 |33 |0 |0 |0 |0 |0 |
|8227 |375 |2256 |0 |0 |7 |0 |0 |
|275 |0 |0 |2838 |0 |0 |0 |0 |
|1823 |90 |0 |0 |308 |0 |0 |0 |
|317 |106 |0 |0 |0 |1071 |0 |0 |
|1318 |81 |0 |0 |0 |3 |0 |0 |
|1091 |15 |0 |0 |28 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| core/lib/ssms.js | 75 |
| core/lib/engine_http.js | 53 |
| test/core/unit/engine_http.test.js | 48 |
| lib/console-reporter.js | 43 |
| core/lib/engine_util.js | 40 |
| core/lib/engine_socketio.js | 36 |
| core/lib/runner.js | 27 |
| test/core/unit/interpolation.test.js | 24 |
| lib/util.js | 23 |
| core/lib/engine_ws.js | 22 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2838}, "macro avg": {"f1-score": 0.6698960694348376, "precision": 0.6951401326079336, "recall": 0.648233033077603, "support": 31231}, "micro avg": {"f1-score": 0.976369632736704, "precision": 0.976369632736704, "recall": 0.976369632736704, "support": 31231}, "weighted avg": {"f1-score": 0.9737730167579556, "precision": 0.9724966665913956, "recall": 0.976369632736704, "support": 31231}, "\u2205": {"f1-score": 0.9856380796060731, "precision": 0.9729817312755701, "recall": 0.9986280297675966, "support": 24053}, "\u23ce": {"f1-score": 0.8392370572207085, "precision": 0.9166666666666666, "recall": 0.7738693467336684, "support": 398}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9486271036315324, "precision": 0.9907493061979649, "recall": 0.9099405267629567, "support": 1177}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u2423": {"f1-score": 0.915770245585549, "precision": 0.9855832241153342, "recall": 0.8551933282789992, "support": 2638}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9537892791127541, "precision": 1.0, "recall": 0.911660777385159, "support": 3113}, "macro avg": {"f1-score": 0.4775474572927701, "precision": 0.6951401326079336, "recall": 0.4212024298763521, "support": 44901}, "micro avg": {"f1-score": 0.8010560605264541, "precision": 0.976369632736704, "recall": 0.6791162780338968, "support": 44901}, "weighted avg": {"f1-score": 0.7235143879995853, "precision": 0.9207559549713903, "recall": 0.6791162780338968, "support": 44901}, "\u2205": {"f1-score": 0.9732774164792641, "precision": 0.9729817312755701, "recall": 0.9735732814526589, "support": 24672}, "\u23ce": {"f1-score": 0.2409073132577239, "precision": 0.9166666666666666, "recall": 0.13867627194957227, "support": 2221}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1134}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8318446601941748, "precision": 0.9907493061979649, "recall": 0.7168674698795181, "support": 1494}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1402}, "\u2423": {"f1-score": 0.3430135320054736, "precision": 0.9855832241153342, "recall": 0.20763920846755637, "support": 10865}},
  "ppcr": 0.695552437584909
}
```
</details>
