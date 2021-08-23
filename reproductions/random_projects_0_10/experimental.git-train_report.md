# Train report for javascript / file:///tmp/top-repos-quality-repos-xdvx_51k/experimental.git HEAD 799e5e5de3ad73b1e44e0ba1ce0ebbda83fa1a11

### Classification report

PPCR: 0.592

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.998| 0.924| 0.985| 0.947| 7824| 8449| 0.926 |
| `␣` | 0.984| 0.966| 0.317| 0.975| 0.480| 1451| 4421| 0.328 |
| `'` | 1.000| 1.000| 0.365| 1.000| 0.535| 376| 1030| 0.365 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 103| 309| 0.333 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 1183| 0.027 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 167| 0.126 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 339| 0.044 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 199| 0.060 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 282| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 224| 0.000 |
| `micro avg` | 0.975| 0.975| 0.577| 0.975| 0.725| 9834| 16603| 0.592 |
| `weighted avg` | 0.957| 0.975| 0.577| 0.966| 0.643| 9834| 16603| 0.592 |
| `macro avg` | 0.296| 0.296| 0.161| 0.296| 0.196| 9834| 16603| 0.592 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|625 |7807 |17 |0 |0 |0 |0 |0 |0 |0 |0 |
|2970 |49 |1402 |0 |0 |0 |0 |0 |0 |0 |0 |
|1151 |30 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|654 |0 |0 |0 |376 |0 |0 |0 |0 |0 |0 |
|282 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|324 |13 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|206 |102 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|187 |11 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|224 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|146 |21 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| web/charting/highcharts/efd/src/assets/highchartsplugins/highcharts-contour.js | 37 |
| web/charting/highcharts/efd/src/assets/highchartsplugins/delaunay.js | 18 |
| go/websocket/web/index.js | 18 |
| web/charting/lightningchart/js/src/index.js | 17 |
| web/charting/highcharts/efd/src/assets/highchartsplugins/vertical-gauge.js | 12 |
| web/charting/lightningchart/js/webpack.config.js | 11 |
| web/charting/highcharts/efd/src/assets/highchartsplugins/pattern-fill-v2.js | 7 |
| web/charting/highcharts/efd/src/assets/highchartsplugins/highcharts-coloraxis-bands.js | 6 |
| projects/expc/karma.conf.js | 4 |
| web/charting/lightningchart/efd/karma.conf.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 376}, "macro avg": {"f1-score": 0.29596407671028524, "precision": 0.2955725701656107, "recall": 0.29640573844425744, "support": 9834}, "micro avg": {"f1-score": 0.974679682733374, "precision": 0.974679682733374, "recall": 0.974679682733374, "support": 9834}, "weighted avg": {"f1-score": 0.9655049780661262, "precision": 0.9566260266318621, "recall": 0.974679682733374, "support": 9834}, "\u2205": {"f1-score": 0.9846755376174561, "precision": 0.9718660525333002, "recall": 0.9978271983640081, "support": 7824}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.9749652294853963, "precision": 0.983859649122807, "recall": 0.9662301860785665, "support": 1451}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 282}, "\u0027": {"f1-score": 0.5348506401137979, "precision": 1.0, "recall": 0.3650485436893204, "support": 1030}, "macro avg": {"f1-score": 0.19618313295672424, "precision": 0.2955725701656107, "recall": 0.16061860428731217, "support": 16603}, "micro avg": {"f1-score": 0.7251200968339827, "precision": 0.974679682733374, "recall": 0.5773053062699512, "support": 16603}, "weighted avg": {"f1-score": 0.6429831453486745, "precision": 0.818583375692693, "recall": 0.5773053062699512, "support": 16603}, "\u2205": {"f1-score": 0.9473364882902561, "precision": 0.9718660525333002, "recall": 0.9240146762930525, "support": 8449}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1183}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 224}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 339}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 199}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 309}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 167}, "\u2423": {"f1-score": 0.4796442011631885, "precision": 0.983859649122807, "recall": 0.3171228228907487, "support": 4421}},
  "ppcr": 0.5923025959164007
}
```
</details>
