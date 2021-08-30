# Train report for javascript / file:///tmp/top-repos-quality-repos-xhjr11z0/house.git HEAD c3641e300d9c40568cd57dce1884f1a96c4c7849

### Classification report

PPCR: 0.531

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.965| 0.997| 0.654| 0.981| 0.780| 4136| 6298| 0.657 |
| `"` | 0.997| 1.000| 0.623| 0.998| 0.767| 1239| 1989| 0.623 |
| `␣` | 0.952| 0.748| 0.211| 0.838| 0.345| 500| 1776| 0.282 |
| `⏎` | 0.943| 0.926| 0.522| 0.935| 0.672| 216| 383| 0.564 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 134| 0.082 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 222| 0.032 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 242| 0.025 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 478| 0.008 |
| `micro avg` | 0.970| 0.970| 0.515| 0.970| 0.673| 6119| 11522| 0.531 |
| `macro avg` | 0.482| 0.459| 0.251| 0.469| 0.320| 6119| 11522| 0.531 |
| `weighted avg` | 0.965| 0.970| 0.515| 0.966| 0.634| 6119| 11522| 0.531 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2162 |4122 |14 |0 |0 |0 |0 |0 |0 |
|1276 |123 |374 |0 |0 |3 |0 |0 |0 |
|750 |0 |0 |1239 |0 |0 |0 |0 |0 |
|474 |0 |0 |4 |0 |0 |0 |0 |0 |
|167 |15 |1 |0 |0 |200 |0 |0 |0 |
|236 |4 |2 |0 |0 |0 |0 |0 |0 |
|215 |7 |0 |0 |0 |0 |0 |0 |0 |
|123 |0 |2 |0 |0 |9 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/js/app.js | 134 |
| scripts/hook/hook_tpl_mini.js | 13 |
| scripts/hook/hook_tpl.js | 10 |
| scripts/hook/native_hook_frag.js | 7 |
| scripts/misc/sideload_stetho.js | 6 |
| scripts/monitor/monitor_tpl.js | 6 |
| scripts/enum/env.js | 6 |
| static/src-min/ext-error_marker.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9983883964544722, "precision": 0.996781979082864, "recall": 1.0, "support": 1239}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "macro avg": {"f1-score": 0.4689006513519855, "precision": 0.48211821325780624, "recall": 0.4588176266208181, "support": 6119}, "micro avg": {"f1-score": 0.9699297270795881, "precision": 0.9699297270795881, "recall": 0.9699297270795881, "support": 6119}, "weighted avg": {"f1-score": 0.9664151173111233, "precision": 0.9652431931469274, "recall": 0.9699297270795881, "support": 6119}, "\u2205": {"f1-score": 0.9806113952658498, "precision": 0.9651135565441349, "recall": 0.996615087040619, "support": 4136}, "\u23ce": {"f1-score": 0.9345794392523364, "precision": 0.9433962264150944, "recall": 0.9259259259259259, "support": 216}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.8376259798432252, "precision": 0.9516539440203562, "recall": 0.748, "support": 500}},
  "cl_report_full": {"\"": {"f1-score": 0.7667079207920792, "precision": 0.996781979082864, "recall": 0.6229260935143288, "support": 1989}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 478}, "macro avg": {"f1-score": 0.3204816551873031, "precision": 0.48211821325780624, "recall": 0.2512747975731237, "support": 11522}, "micro avg": {"f1-score": 0.6728643500935321, "precision": 0.9699297270795881, "recall": 0.5151015448706822, "support": 11522}, "weighted avg": {"f1-score": 0.6342196293788598, "precision": 0.8776551549043491, "recall": 0.5151015448706822, "support": 11522}, "\u2205": {"f1-score": 0.7800170309395402, "precision": 0.9651135565441349, "recall": 0.6544934899968244, "support": 6298}, "\u23ce": {"f1-score": 0.6722689075630252, "precision": 0.9433962264150944, "recall": 0.5221932114882507, "support": 383}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 134}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 242}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 222}, "\u2423": {"f1-score": 0.3448593822037805, "precision": 0.9516539440203562, "recall": 0.21058558558558557, "support": 1776}},
  "ppcr": 0.5310709946189898
}
```
</details>
