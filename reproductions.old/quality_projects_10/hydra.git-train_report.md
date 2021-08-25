# Train report for javascript / file:///tmp/top-repos-quality-repos-otox_8nd/hydra.git HEAD f8181b840b1ba664f40f5b38ee9c8dd1dce7f776

### Classification report

PPCR: 0.691

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.994| 0.943| 0.988| 0.962| 27360| 28837| 0.949 |
| `␣` | 0.971| 0.961| 0.610| 0.966| 0.749| 8343| 13149| 0.634 |
| `⏎⏎` | 0.945| 0.953| 0.442| 0.949| 0.602| 727| 1569| 0.463 |
| `⏎` | 0.978| 0.493| 0.068| 0.656| 0.128| 363| 2617| 0.139 |
| `"` | 1.000| 1.000| 0.090| 1.000| 0.165| 273| 3027| 0.090 |
| `'` | 1.000| 1.000| 0.097| 1.000| 0.177| 192| 1983| 0.097 |
| `⏎␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 812| 0.048 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 613| 0.028 |
| `⏎␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 745| 0.020 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 639| 0.008 |
| `macro avg` | 0.588| 0.540| 0.225| 0.556| 0.278| 37334| 53991| 0.691 |
| `micro avg` | 0.979| 0.979| 0.677| 0.979| 0.801| 37334| 53991| 0.691 |
| `weighted avg` | 0.977| 0.979| 0.677| 0.977| 0.736| 37334| 53991| 0.691 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1477 |27200 |160 |0 |0 |0 |0 |0 |0 |0 |0 |
|4806 |310 |8021 |0 |4 |0 |8 |0 |0 |0 |0 |
|2754 |0 |0 |273 |0 |0 |0 |0 |0 |0 |0 |
|2254 |108 |44 |0 |179 |0 |32 |0 |0 |0 |0 |
|1791 |0 |0 |0 |0 |192 |0 |0 |0 |0 |0 |
|842 |13 |21 |0 |0 |0 |693 |0 |0 |0 |0 |
|634 |1 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|773 |33 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|596 |17 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|730 |13 |2 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/js/processView.js | 138 |
| src/js/process_graph.js | 67 |
| src/js/fields.js | 57 |
| src/js/layout_examples.js | 53 |
| src/js/ipcsInfoView.js | 37 |
| src/js/listview.js | 32 |
| src/js/debuggee_tracker/tracker.js | 28 |
| src/js/layout/splitted.js | 26 |
| src/js/layout/tabbed.js | 21 |
| src/js/breakpoints_view.js | 21 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 273}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 192}, "macro avg": {"f1-score": 0.5559422860134365, "precision": 0.5876999112007883, "recall": 0.5401902227081601, "support": 37334}, "micro avg": {"f1-score": 0.9792146568811271, "precision": 0.9792146568811271, "recall": 0.9792146568811271, "support": 37334}, "weighted avg": {"f1-score": 0.9773858878037228, "precision": 0.9771774009094003, "recall": 0.9792146568811271, "support": 37334}, "\u2205": {"f1-score": 0.9881028062846244, "precision": 0.9821267376782813, "recall": 0.9941520467836257, "support": 27360}, "\u23ce": {"f1-score": 0.6556776556776557, "precision": 0.9781420765027322, "recall": 0.4931129476584022, "support": 363}, "\u23ce\u23ce": {"f1-score": 0.9493150684931507, "precision": 0.9454297407912687, "recall": 0.953232462173315, "support": 727}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.966327329678935, "precision": 0.9713005570356018, "recall": 0.9614047704662592, "support": 8343}},
  "cl_report_full": {"\"": {"f1-score": 0.16545454545454547, "precision": 1.0, "recall": 0.09018830525272548, "support": 3027}, "\u0027": {"f1-score": 0.17655172413793102, "precision": 1.0, "recall": 0.09682299546142209, "support": 1983}, "macro avg": {"f1-score": 0.27836164457749857, "precision": 0.5876999112007883, "recall": 0.22503338493194, "support": 53991}, "micro avg": {"f1-score": 0.8006131946345469, "precision": 0.9792146568811271, "recall": 0.6771128521420237, "support": 53991}, "weighted avg": {"f1-score": 0.7359242066679116, "precision": 0.9287917770813445, "recall": 0.6771128521420237, "support": 53991}, "\u2205": {"f1-score": 0.9622868463878864, "precision": 0.9821267376782813, "recall": 0.9432326524950584, "support": 28837}, "\u23ce": {"f1-score": 0.12785714285714284, "precision": 0.9781420765027322, "recall": 0.06839893007260221, "support": 2617}, "\u23ce\u23ce": {"f1-score": 0.6020851433536055, "precision": 0.9454297407912687, "recall": 0.4416826003824092, "support": 1569}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 812}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 639}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 745}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 613}, "\u2423": {"f1-score": 0.7493810435838745, "precision": 0.9713005570356018, "recall": 0.6100083656551829, "support": 13149}},
  "ppcr": 0.6914856179733659
}
```
</details>
