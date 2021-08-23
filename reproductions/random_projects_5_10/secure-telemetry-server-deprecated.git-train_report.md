# Train report for javascript / file:///tmp/top-repos-quality-repos-n5znpwc5/secure-telemetry-server-deprecated.git HEAD ebde174bc64739e41d19a6d1627359af8c2e53a2

### Classification report

PPCR: 0.686

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.990| 0.961| 0.990| 0.975| 7470| 7698| 0.970 |
| `␣` | 0.966| 0.993| 0.549| 0.979| 0.700| 2314| 4189| 0.552 |
| `⏎` | 0.936| 0.928| 0.345| 0.932| 0.504| 332| 893| 0.372 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 153| 0.163 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 194| 0.113 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 182| 0.066 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 310| 0.029 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 155| 0.026 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 669| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 416| 0.000 |
| `weighted avg` | 0.975| 0.982| 0.673| 0.979| 0.733| 10188| 14859| 0.686 |
| `macro avg` | 0.289| 0.291| 0.185| 0.290| 0.218| 10188| 14859| 0.686 |
| `micro avg` | 0.982| 0.982| 0.673| 0.982| 0.799| 10188| 14859| 0.686 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|228 |7399 |71 |0 |0 |0 |0 |0 |0 |0 |0 |
|1875 |16 |2298 |0 |0 |0 |0 |0 |0 |0 |0 |
|561 |19 |5 |308 |0 |0 |0 |0 |0 |0 |0 |
|669 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|416 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|301 |1 |0 |8 |0 |0 |0 |0 |0 |0 |0 |
|172 |14 |2 |6 |0 |0 |0 |0 |0 |0 |0 |
|170 |10 |0 |2 |0 |0 |0 |0 |0 |0 |0 |
|151 |1 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|128 |20 |0 |5 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/js/jquery.popupoverlay.js | 64 |
| static/js/justgage.js | 59 |
| static/js/blackbox.js | 36 |
| static/js/rotating-globe.js | 15 |
| static/js/form.js | 5 |
| static/js/utilities.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.29010850256250104, "precision": 0.2891293416466789, "recall": 0.2911291724084468, "support": 10188}, "micro avg": {"f1-score": 0.982037691401649, "precision": 0.982037691401649, "recall": 0.982037691401649, "support": 10188}, "weighted avg": {"f1-score": 0.9785650238780653, "precision": 0.9751796146606252, "recall": 0.982037691401649, "support": 10188}, "\u2205": {"f1-score": 0.9898327759197325, "precision": 0.9891711229946524, "recall": 0.9904953145917001, "support": 7470}, "\u23ce": {"f1-score": 0.9319213313161876, "precision": 0.9361702127659575, "recall": 0.927710843373494, "support": 332}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.9793309183890901, "precision": 0.9659520807061791, "recall": 0.993085566119274, "support": 2314}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 416}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 669}, "macro avg": {"f1-score": 0.2178811811010184, "precision": 0.2891293416466789, "recall": 0.18546431710329483, "support": 14859}, "micro avg": {"f1-score": 0.7988980716253444, "precision": 0.982037691401649, "recall": 0.6733292953765395, "support": 14859}, "weighted avg": {"f1-score": 0.7326673688342356, "precision": 0.8410399468935338, "recall": 0.6733292953765395, "support": 14859}, "\u2205": {"f1-score": 0.9749637633416787, "precision": 0.9891711229946524, "recall": 0.9611587425305274, "support": 7698}, "\u23ce": {"f1-score": 0.5040916530278232, "precision": 0.9361702127659575, "recall": 0.3449048152295633, "support": 893}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 310}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 153}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "\u2423": {"f1-score": 0.6997563946406822, "precision": 0.9659520807061791, "recall": 0.5485796132728575, "support": 4189}},
  "ppcr": 0.6856450635978195
}
```
</details>
