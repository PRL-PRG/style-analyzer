# Train report for javascript / file:///tmp/top-repos-quality-repos-pced0zst/otus_web.git HEAD b90ad69e1b5c1828fa2ace165710422d113d1d17

### Classification report

PPCR: 0.401

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 0.982| 0.391| 0.964| 0.553| 5032| 12636| 0.398 |
| `␣` | 0.963| 0.949| 0.443| 0.956| 0.606| 3226| 6915| 0.467 |
| `'` | 0.982| 1.000| 0.729| 0.991| 0.837| 1289| 1767| 0.729 |
| `⏎` | 0.913| 0.628| 0.104| 0.744| 0.186| 183| 1110| 0.165 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 362| 0.113 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 50| 0.480 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 365| 0.038 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 563| 0.016 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 182| 0.044 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 201| 0.025 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 187| 0.016 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 194| 0.015 |
| `micro avg` | 0.956| 0.956| 0.383| 0.956| 0.547| 9837| 24532| 0.401 |
| `weighted avg` | 0.946| 0.956| 0.383| 0.950| 0.525| 9837| 24532| 0.401 |
| `macro avg` | 0.317| 0.297| 0.139| 0.305| 0.182| 9837| 24532| 0.401 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⇥⁺| ⏎⇥⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|7604 |4940 |92 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3689 |157 |3061 |0 |8 |0 |0 |0 |0 |0 |0 |0 |0 |
|478 |0 |0 |1289 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|927 |57 |11 |0 |115 |0 |0 |0 |0 |0 |0 |0 |0 |
|554 |5 |2 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|351 |12 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|321 |32 |8 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|184 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|174 |6 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|196 |3 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|191 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|26 |0 |0 |24 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| ecommerce/static_in_env/js/addons/datatables-select.js | 170 |
| coursera/coursera_react2/homework/course-detail/src/App.js | 30 |
| coursera/coursera_react2/homework/course-detail/src/serviceWorker.js | 22 |
| coursera/coursera_react1/frontend/src/serviceWorker.js | 22 |
| coursera/coursera_react2/homework/courses-list/src/serviceWorker.js | 22 |
| ecommerce/static_in_env/js/addons/jquery.zmd.hierarchical-display.js | 22 |
| ecommerce/static_in_env/js/modules/wow.js | 20 |
| coursera/coursera_react1/frontend/src/App.js | 17 |
| coursera/coursera_react2/homework/courses-list/src/App.js | 17 |
| ecommerce/static_in_env/js/modules/waves.js | 13 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u0027": {"f1-score": 0.9907763259031515, "precision": 0.9817212490479817, "recall": 1.0, "support": 1289}, "macro avg": {"f1-score": 0.30455682475869955, "precision": 0.31697676336908254, "recall": 0.2965821150409246, "support": 9837}, "micro avg": {"f1-score": 0.9560841720036597, "precision": 0.9560841720036597, "recall": 0.9560841720036597, "support": 9837}, "weighted avg": {"f1-score": 0.9501536729117259, "precision": 0.9455779871694019, "recall": 0.9560841720036597, "support": 9837}, "\u2205": {"f1-score": 0.9639024390243903, "precision": 0.9467228823303948, "recall": 0.9817170111287759, "support": 5032}, "\u23ce": {"f1-score": 0.7443365695792881, "precision": 0.9126984126984127, "recall": 0.6284153005464481, "support": 183}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9556665625975648, "precision": 0.9625786163522012, "recall": 0.9488530688158711, "support": 3226}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u0027": {"f1-score": 0.837012987012987, "precision": 0.9817212490479817, "recall": 0.7294850028296548, "support": 1767}, "macro avg": {"f1-score": 0.18190944624447783, "precision": 0.31697676336908254, "recall": 0.13889133255259553, "support": 24532}, "micro avg": {"f1-score": 0.5472955279466962, "precision": 0.9560841720036597, "recall": 0.3833768139572803, "support": 24532}, "weighted avg": {"f1-score": 0.5246843559068526, "precision": 0.8709774237063983, "recall": 0.3833768139572803, "support": 24532}, "\u2205": {"f1-score": 0.5533773944214182, "precision": 0.9467228823303948, "recall": 0.39094650205761317, "support": 12636}, "\u23ce": {"f1-score": 0.18608414239482202, "precision": 0.9126984126984127, "recall": 0.1036036036036036, "support": 1110}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 201}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 563}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 365}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 362}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u2423": {"f1-score": 0.6064388311045071, "precision": 0.9625786163522012, "recall": 0.44266088214027477, "support": 6915}},
  "ppcr": 0.4009864666557965
}
```
</details>
