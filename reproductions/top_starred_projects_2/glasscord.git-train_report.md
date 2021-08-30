# Train report for javascript / file:///tmp/top-repos-quality-repos-pwf3_6co/glasscord.git HEAD a658939d0ab43eb9228d63fdbae450e6be513c10

### Classification report

PPCR: 0.328

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.999| 0.440| 0.996| 0.610| 1722| 3909| 0.441 |
| `"` | 1.000| 1.000| 0.479| 1.000| 0.648| 280| 584| 0.479 |
| `␣` | 0.996| 1.000| 0.162| 0.998| 0.279| 247| 1522| 0.162 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 209| 0.048 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 320| 0.006 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 180| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 163| 0.000 |
| `micro avg` | 0.994| 0.994| 0.326| 0.994| 0.491| 2261| 6887| 0.328 |
| `macro avg` | 0.427| 0.428| 0.155| 0.428| 0.220| 2261| 6887| 0.328 |
| `weighted avg` | 0.989| 0.994| 0.326| 0.992| 0.463| 2261| 6887| 0.328 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⇥⁺| ⏎⏎| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2187 |1721 |1 |0 |0 |0 |0 |0 |
|1275 |0 |247 |0 |0 |0 |0 |0 |
|304 |0 |0 |280 |0 |0 |0 |0 |
|318 |2 |0 |0 |0 |0 |0 |0 |
|199 |10 |0 |0 |0 |0 |0 |0 |
|180 |0 |0 |0 |0 |0 |0 |0 |
|163 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/version_check.js | 4 |
| src/utils.js | 3 |
| src/main.js | 2 |
| package.js | 2 |
| src/module.js | 1 |
| src/modules/css_loader.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 280}, "macro avg": {"f1-score": 0.42774530502461033, "precision": 0.4270061904850543, "recall": 0.428488468558155, "support": 2261}, "micro avg": {"f1-score": 0.994250331711632, "precision": 0.994250331711632, "recall": 0.994250331711632, "support": 2261}, "weighted avg": {"f1-score": 0.9916062382779528, "precision": 0.9889784169624085, "recall": 0.994250331711632, "support": 2261}, "\u2205": {"f1-score": 0.9962373371924745, "precision": 0.9930755914598961, "recall": 0.9994192799070848, "support": 1722}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.997979797979798, "precision": 0.9959677419354839, "recall": 1.0, "support": 247}},
  "cl_report_full": {"\"": {"f1-score": 0.6481481481481481, "precision": 1.0, "recall": 0.4794520547945205, "support": 584}, "macro avg": {"f1-score": 0.21961593504981772, "precision": 0.4270061904850543, "recall": 0.15457208181011697, "support": 6887}, "micro avg": {"f1-score": 0.4914735461303017, "precision": 0.994250331711632, "recall": 0.32641208073181355, "support": 6887}, "weighted avg": {"f1-score": 0.46290924615777035, "precision": 0.8685632917442341, "recall": 0.32641208073181355, "support": 6887}, "\u2205": {"f1-score": 0.6100673520028358, "precision": 0.9930755914598961, "recall": 0.44026605269889996, "support": 3909}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 320}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 209}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u2423": {"f1-score": 0.2790960451977401, "precision": 0.9959677419354839, "recall": 0.16228646517739817, "support": 1522}},
  "ppcr": 0.3282996950776826
}
```
</details>
