# Train report for javascript / file:///tmp/top-repos-quality-repos-fs5ydjt5/birthregistration.git HEAD 4adad63c0430c2858ddd58916c2741d306f38da0

### Classification report

PPCR: 0.885

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 0.982| 0.963| 0.985| 0.975| 49818| 50786| 0.981 |
| `␣` | 0.947| 0.983| 0.960| 0.965| 0.954| 25990| 26603| 0.977 |
| `'` | 0.991| 1.000| 0.900| 0.995| 0.943| 15741| 17483| 0.900 |
| `⏎` | 0.944| 0.922| 0.700| 0.933| 0.804| 4606| 6064| 0.760 |
| `⏎␣⁻␣⁻` | 0.866| 0.928| 0.614| 0.896| 0.718| 1841| 2785| 0.661 |
| `⏎⏎` | 0.982| 0.896| 0.252| 0.937| 0.402| 1071| 3804| 0.282 |
| `⏎␣⁺␣⁺` | 0.823| 0.469| 0.057| 0.597| 0.107| 318| 2608| 0.122 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 299| 966| 0.310 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 163| 1043| 0.156 |
| `"` | 1.000| 0.051| 0.009| 0.098| 0.018| 156| 879| 0.177 |
| `weighted avg` | 0.968| 0.973| 0.861| 0.970| 0.886| 100003| 113021| 0.885 |
| `micro avg` | 0.973| 0.973| 0.861| 0.973| 0.913| 100003| 113021| 0.885 |
| `macro avg` | 0.754| 0.623| 0.446| 0.641| 0.492| 100003| 113021| 0.885 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|968 |48914 |899 |0 |1 |3 |1 |0 |0 |0 |0 |
|613 |286 |25536 |0 |158 |0 |8 |2 |0 |0 |0 |
|1742 |0 |0 |15741 |0 |0 |0 |0 |0 |0 |0 |
|1458 |27 |311 |0 |4247 |15 |6 |0 |0 |0 |0 |
|2733 |0 |54 |0 |57 |960 |0 |0 |0 |0 |0 |
|944 |111 |3 |0 |18 |0 |1709 |0 |0 |0 |0 |
|2290 |123 |43 |0 |3 |0 |0 |149 |0 |0 |0 |
|723 |0 |0 |148 |0 |0 |0 |0 |8 |0 |0 |
|880 |31 |98 |0 |4 |0 |0 |30 |0 |0 |0 |
|667 |32 |8 |0 |9 |0 |250 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| br/static/js/select2/select2.full.js | 460 |
| base/static/base/js/app.js | 336 |
| br/static/js/select2.js | 330 |
| br/static/js/select2/select2.js | 330 |
| base/static/base/js/charts.js | 312 |
| base/static/base/js/widgets.js | 243 |
| base/static/base/js/tables-dynamic.js | 125 |
| base/static/base/js/settings.js | 105 |
| base/static/base/js/maps-vector.js | 100 |
| base/static/base/js/index.js | 87 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.09756097560975609, "precision": 1.0, "recall": 0.05128205128205128, "support": 156}, "\u0027": {"f1-score": 0.9953208978817577, "precision": 0.9906853798225187, "recall": 1.0, "support": 15741}, "macro avg": {"f1-score": 0.6405592836973528, "precision": 0.7540791981623352, "recall": 0.6230937767190261, "support": 100003}, "micro avg": {"f1-score": 0.9726108216753497, "precision": 0.9726108216753497, "recall": 0.9726108216753497, "support": 100003}, "weighted avg": {"f1-score": 0.9695116100202765, "precision": 0.96833256062243, "recall": 0.9726108216753497, "support": 100003}, "\u2205": {"f1-score": 0.9847597189506957, "precision": 0.9876827396817704, "recall": 0.9818539483720744, "support": 49818}, "\u23ce": {"f1-score": 0.9330989783587829, "precision": 0.9444073826995775, "recall": 0.9220581849761181, "support": 4606}, "\u23ce\u23ce": {"f1-score": 0.9370424597364568, "precision": 0.9815950920245399, "recall": 0.896358543417367, "support": 1071}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.597194388777555, "precision": 0.8232044198895028, "recall": 0.46855345911949686, "support": 318}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8959370904325032, "precision": 0.8657548125633232, "recall": 0.9282998370450842, "support": 1841}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 299}, "\u2423": {"f1-score": 0.964678327226021, "precision": 0.9474621549421193, "recall": 0.9825317429780684, "support": 25990}},
  "cl_report_full": {"\"": {"f1-score": 0.018038331454340473, "precision": 1.0, "recall": 0.009101251422070534, "support": 879}, "\u0027": {"f1-score": 0.9433656957928801, "precision": 0.9906853798225187, "recall": 0.9003603500543385, "support": 17483}, "macro avg": {"f1-score": 0.4921149169669838, "precision": 0.7540791981623352, "recall": 0.44559979443992725, "support": 113021}, "micro avg": {"f1-score": 0.9131741024485504, "precision": 0.9726108216753497, "recall": 0.8605834313977049, "support": 113021}, "weighted avg": {"f1-score": 0.8855971390053774, "precision": 0.9518926765791574, "recall": 0.8605834313977049, "support": 113021}, "\u2205": {"f1-score": 0.9752567042169276, "precision": 0.9876827396817704, "recall": 0.9631394478793368, "support": 50786}, "\u23ce": {"f1-score": 0.8042798977369566, "precision": 0.9444073826995775, "recall": 0.7003627968337731, "support": 6064}, "\u23ce\u23ce": {"f1-score": 0.4015056461731493, "precision": 0.9815950920245399, "recall": 0.25236593059936907, "support": 3804}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.10684833273574759, "precision": 0.8232044198895028, "recall": 0.0571319018404908, "support": 2608}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1043}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7182181130489599, "precision": 0.8657548125633232, "recall": 0.6136445242369838, "support": 2785}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 966}, "\u2423": {"f1-score": 0.9536364485108767, "precision": 0.9474621549421193, "recall": 0.9598917415329098, "support": 26603}},
  "ppcr": 0.8848178657063731
}
```
</details>