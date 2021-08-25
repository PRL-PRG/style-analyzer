# Train report for javascript / file:///tmp/top-repos-quality-repos-91ekfjqr/sigmapi-web.git HEAD 6cd7c2f0da8b3a6cea9cce308fa377857436947d

### Classification report

PPCR: 0.808

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.992| 0.966| 0.991| 0.978| 96215| 98817| 0.974 |
| `␣` | 0.961| 0.995| 0.893| 0.978| 0.926| 53435| 59507| 0.898 |
| `'` | 0.950| 1.000| 0.782| 0.975| 0.858| 6396| 8176| 0.782 |
| `⏎` | 0.957| 0.211| 0.019| 0.346| 0.038| 1057| 11507| 0.092 |
| `⏎⏎` | 0.981| 0.984| 0.232| 0.982| 0.376| 989| 4188| 0.236 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 445| 5275| 0.084 |
| `"` | 1.000| 0.067| 0.012| 0.126| 0.023| 358| 2057| 0.174 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 329| 4926| 0.067 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 312| 1005| 0.310 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 51| 836| 0.061 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 589| 0.025 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 495| 0.014 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 240| 0.004 |
| `weighted avg` | 0.972| 0.979| 0.790| 0.973| 0.814| 159610| 197618| 0.808 |
| `macro avg` | 0.449| 0.327| 0.223| 0.338| 0.246| 159610| 197618| 0.808 |
| `micro avg` | 0.979| 0.979| 0.790| 0.979| 0.875| 159610| 197618| 0.808 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| ⏎⏎␣⁺␣⁺| ⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2602 |95430 |785 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6072 |256 |53168 |10 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |
|10450 |45 |771 |223 |0 |0 |0 |18 |0 |0 |0 |0 |0 |0 |
|1780 |0 |0 |0 |6396 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4830 |214 |231 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4597 |298 |31 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3199 |0 |16 |0 |0 |0 |0 |973 |0 |0 |0 |0 |0 |0 |
|1699 |0 |0 |0 |334 |0 |0 |0 |24 |0 |0 |0 |0 |0 |
|693 |26 |286 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|785 |36 |15 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|574 |0 |15 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|488 |0 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|239 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| sigmapiweb/static/js/lib/vue.js | 925 |
| sigmapiweb/static/js/lib/lodash.js | 850 |
| sigmapiweb/static/js/lib/two.js | 482 |
| sigmapiweb/static/js/secure/party.guest.v5.js | 257 |
| sigmapiweb/static/js/secure/party.guest.v4.js | 247 |
| sigmapiweb/static/admin/js/urlify.js | 123 |
| sigmapiweb/static/js/public/family-tree.js | 80 |
| sigmapiweb/static/js/lib/backbone.js | 69 |
| sigmapiweb/static/admin/js/inlines.js | 62 |
| sigmapiweb/static/js/secure/standards.points.js | 49 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.1256544502617801, "precision": 1.0, "recall": 0.0670391061452514, "support": 358}, "\u0027": {"f1-score": 0.9745543196708822, "precision": 0.95037147102526, "recall": 1.0, "support": 6396}, "macro avg": {"f1-score": 0.3382586248883286, "precision": 0.4492473834181896, "recall": 0.3268215437408285, "support": 159610}, "micro avg": {"f1-score": 0.978723137648017, "precision": 0.978723137648017, "recall": 0.978723137648017, "support": 159610}, "weighted avg": {"f1-score": 0.9726468139779815, "precision": 0.9718049198601715, "recall": 0.978723137648017, "support": 159610}, "\u2205": {"f1-score": 0.9913723697674539, "precision": 0.990903993520653, "recall": 0.9918411890037936, "support": 96215}, "\u23ce": {"f1-score": 0.3457364341085271, "precision": 0.9570815450643777, "recall": 0.2109744560075686, "support": 1057}, "\u23ce\u23ce": {"f1-score": 0.9823321554770318, "precision": 0.9808467741935484, "recall": 0.9838220424671386, "support": 989}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 445}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 312}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 329}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u2423": {"f1-score": 0.9777123942625966, "precision": 0.9610122006326254, "recall": 0.9950032750070179, "support": 53435}},
  "cl_report_full": {"\"": {"f1-score": 0.023065833733781835, "precision": 1.0, "recall": 0.01166747690811862, "support": 2057}, "\u0027": {"f1-score": 0.8581779149335838, "precision": 0.95037147102526, "recall": 0.7822896281800391, "support": 8176}, "macro avg": {"f1-score": 0.24608270000308077, "precision": 0.4492473834181896, "recall": 0.22345125535092691, "support": 197618}, "micro avg": {"f1-score": 0.8745898977683719, "precision": 0.978723137648017, "recall": 0.790484672448866, "support": 197618}, "weighted avg": {"f1-score": 0.8138770347652172, "precision": 0.9111177814129122, "recall": 0.790484672448866, "support": 197618}, "\u2205": {"f1-score": 0.9781522424316969, "precision": 0.990903993520653, "recall": 0.9657245210844287, "support": 98817}, "\u23ce": {"f1-score": 0.03798977853492334, "precision": 0.9570815450643777, "recall": 0.01937950812548883, "support": 11507}, "\u23ce\u23ce": {"f1-score": 0.37567567567567567, "precision": 0.9808467741935484, "recall": 0.23233046800382043, "support": 4188}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 495}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 589}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 240}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5275}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1005}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4926}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 836}, "\u2423": {"f1-score": 0.9260136547303887, "precision": 0.9610122006326254, "recall": 0.8934747172601543, "support": 59507}},
  "ppcr": 0.8076693418615714
}
```
</details>
