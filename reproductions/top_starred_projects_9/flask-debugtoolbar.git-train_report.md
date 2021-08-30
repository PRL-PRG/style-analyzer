# Train report for javascript / file:///tmp/top-repos-quality-repos-gqx56fkx/flask-debugtoolbar.git HEAD d474a6a689be916d65c2adf173e6517290902abe

### Classification report

PPCR: 0.790

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.998| 0.984| 0.932| 0.991| 0.964| 21392| 22599| 0.947 |
| `␣` | 0.955| 0.999| 0.884| 0.976| 0.918| 12870| 14547| 0.885 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 132| 813| 0.162 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 74| 2330| 0.032 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 221| 0.149 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 715| 0.042 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 257| 0.093 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 306| 0.016 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1723| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 258| 0.000 |
| `weighted avg` | 0.973| 0.981| 0.775| 0.977| 0.803| 34560| 43769| 0.790 |
| `micro avg` | 0.981| 0.981| 0.775| 0.981| 0.866| 34560| 43769| 0.790 |
| `macro avg` | 0.195| 0.198| 0.182| 0.197| 0.188| 34560| 43769| 0.790 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1207 |21052 |340 |0 |0 |0 |0 |0 |0 |0 |0 |
|1677 |17 |12853 |0 |0 |0 |0 |0 |0 |0 |0 |
|2256 |7 |67 |0 |0 |0 |0 |0 |0 |0 |0 |
|1723 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|681 |3 |129 |0 |0 |0 |0 |0 |0 |0 |0 |
|685 |7 |23 |0 |0 |0 |0 |0 |0 |0 |0 |
|301 |1 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|233 |9 |15 |0 |0 |0 |0 |0 |0 |0 |0 |
|258 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|188 |0 |33 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| flask_debugtoolbar/static/codemirror/codemirror.js | 386 |
| flask_debugtoolbar/static/js/jquery.tablesorter.js | 110 |
| flask_debugtoolbar/static/codemirror/util/match-highlighter.js | 25 |
| flask_debugtoolbar/static/codemirror/util/formatting.js | 25 |
| flask_debugtoolbar/static/codemirror/util/javascript-hint.js | 24 |
| flask_debugtoolbar/static/codemirror/util/searchcursor.js | 23 |
| flask_debugtoolbar/static/codemirror/util/foldcode.js | 23 |
| flask_debugtoolbar/static/codemirror/util/search.js | 11 |
| flask_debugtoolbar/static/codemirror/util/closetag.js | 7 |
| flask_debugtoolbar/static/codemirror/util/simple-hint.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19671146564882505, "precision": 0.1952534023227659, "recall": 0.19827853066072962, "support": 34560}, "micro avg": {"f1-score": 0.9810474537037037, "precision": 0.9810474537037037, "recall": 0.9810474537037037, "support": 34560}, "weighted avg": {"f1-score": 0.9769023468044523, "precision": 0.9731868783024155, "recall": 0.9810474537037037, "support": 34560}, "\u2205": {"f1-score": 0.9909621540199586, "precision": 0.9979142965491088, "recall": 0.9841062079281975, "support": 21392}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 74}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u2423": {"f1-score": 0.976152502468292, "precision": 0.9546197266785502, "recall": 0.9986790986790987, "support": 12870}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1723}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 258}, "macro avg": {"f1-score": 0.18812994105608774, "precision": 0.1952534023227659, "recall": 0.18150955164385615, "support": 43769}, "micro avg": {"f1-score": 0.8657074646682582, "precision": 0.9810474537037037, "recall": 0.7746350156503461, "support": 43769}, "weighted avg": {"f1-score": 0.8025332362673511, "precision": 0.8325234378602705, "recall": 0.7746350156503461, "support": 43769}, "\u2205": {"f1-score": 0.9635885112713125, "precision": 0.9979142965491088, "recall": 0.9315456436125492, "support": 22599}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2330}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 306}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 813}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 221}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 715}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 257}, "\u2423": {"f1-score": 0.9177108992895648, "precision": 0.9546197266785502, "recall": 0.8835498728260123, "support": 14547}},
  "ppcr": 0.7895999451666704
}
```
</details>
