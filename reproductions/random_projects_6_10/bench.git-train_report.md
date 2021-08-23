# Train report for javascript / file:///tmp/top-repos-quality-repos-h2llj9ax/bench.git HEAD 363e374742ed1f4a994de6ca86a1f9bce03986a8

### Classification report

PPCR: 0.553

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.999| 0.945| 0.987| 0.959| 6526| 6901| 0.946 |
| `␣` | 0.993| 0.842| 0.173| 0.912| 0.294| 673| 3283| 0.205 |
| `⏎` | 0.964| 0.724| 0.173| 0.827| 0.294| 185| 773| 0.239 |
| `⏎⏎` | 0.984| 0.939| 0.308| 0.961| 0.470| 132| 402| 0.328 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 344| 0.017 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 81| 0.074 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 320| 0.006 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 904| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 537| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 82| 0.000 |
| `micro avg` | 0.976| 0.976| 0.539| 0.976| 0.695| 7530| 13627| 0.553 |
| `weighted avg` | 0.974| 0.976| 0.539| 0.974| 0.587| 7530| 13627| 0.553 |
| `macro avg` | 0.392| 0.351| 0.160| 0.369| 0.202| 7530| 13627| 0.553 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|375 |6522 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|2610 |106 |567 |0 |0 |0 |0 |0 |0 |0 |0 |
|904 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|588 |49 |0 |0 |134 |0 |2 |0 |0 |0 |0 |
|537 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|270 |3 |0 |0 |5 |0 |124 |0 |0 |0 |0 |
|338 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|318 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|82 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|75 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| integrated_interface/resources/js/gui.js | 38 |
| integrated_interface/resources/js/nprogress.js | 31 |
| integrated_interface/bench_cli.js | 24 |
| integrated_interface/resources/modules/benchmark_executor.js | 19 |
| integrated_interface/resources/website_template/js/dynamic_content.js | 13 |
| integrated_interface/resources/modules/validator_executor.js | 11 |
| integrated_interface/resources/website_template/js/custom.js | 11 |
| integrated_interface/resources/modules/exporter.js | 9 |
| integrated_interface/resources/js/project.js | 8 |
| integrated_interface/resources/website_template/js/jquery.metisMenu.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.36866632047296993, "precision": 0.3915455855237847, "recall": 0.3505601616124162, "support": 7530}, "micro avg": {"f1-score": 0.9756972111553784, "precision": 0.9756972111553784, "recall": 0.9756972111553784, "support": 7530}, "weighted avg": {"f1-score": 0.9737738474878601, "precision": 0.9740839643117658, "recall": 0.9756972111553784, "support": 7530}, "\u2205": {"f1-score": 0.9866868381240546, "precision": 0.9743053480729011, "recall": 0.9993870671161508, "support": 6526}, "\u23ce": {"f1-score": 0.8271604938271606, "precision": 0.9640287769784173, "recall": 0.7243243243243244, "support": 185}, "\u23ce\u23ce": {"f1-score": 0.9612403100775193, "precision": 0.9841269841269841, "recall": 0.9393939393939394, "support": 132}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9115755627009645, "precision": 0.9929947460595446, "recall": 0.8424962852897474, "support": 673}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 537}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 904}, "macro avg": {"f1-score": 0.2017266763254956, "precision": 0.3915455855237847, "recall": 0.1599596605843146, "support": 13627}, "micro avg": {"f1-score": 0.6945219076428605, "precision": 0.9756972111553784, "recall": 0.5391502164819842, "support": 13627}, "weighted avg": {"f1-score": 0.5873094575879949, "precision": 0.8163569568201321, "recall": 0.5391502164819842, "support": 13627}, "\u2205": {"f1-score": 0.959470393527032, "precision": 0.9743053480729011, "recall": 0.9450804231270831, "support": 6901}, "\u23ce": {"f1-score": 0.293859649122807, "precision": 0.9640287769784173, "recall": 0.17335058214747737, "support": 773}, "\u23ce\u23ce": {"f1-score": 0.4696969696969696, "precision": 0.9841269841269841, "recall": 0.30845771144278605, "support": 402}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 344}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 320}, "\u2423": {"f1-score": 0.29423975090814736, "precision": 0.9929947460595446, "recall": 0.17270788912579957, "support": 3283}},
  "ppcr": 0.5525794378806781
}
```
</details>
