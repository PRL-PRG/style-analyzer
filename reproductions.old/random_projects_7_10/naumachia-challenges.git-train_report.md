# Train report for javascript / file:///tmp/top-repos-quality-repos-mlrr7gjb/naumachia-challenges.git HEAD 8dc3053b9fb4c4adaf1627fbae31eb393aa61ebd

### Classification report

PPCR: 0.604

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.993| 0.851| 0.993| 0.917| 52008| 60639| 0.858 |
| `␣` | 0.936| 0.979| 0.426| 0.957| 0.585| 10956| 25173| 0.435 |
| `"` | 1.000| 0.999| 0.233| 0.999| 0.378| 1629| 6972| 0.234 |
| `'` | 0.997| 1.000| 0.176| 0.999| 0.300| 732| 4151| 0.176 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 216| 5960| 0.036 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 72| 984| 0.073 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 1070| 0.038 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 1277| 0.023 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 119| 0.185 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 234| 0.085 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 974| 0.015 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 706| 0.016 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 142| 0.035 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 101| 0.040 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 246| 0.012 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 151| 0.020 |
| `macro avg` | 0.245| 0.248| 0.105| 0.247| 0.136| 65766| 108899| 0.604 |
| `micro avg` | 0.984| 0.984| 0.594| 0.984| 0.741| 65766| 108899| 0.604 |
| `weighted avg` | 0.978| 0.984| 0.594| 0.981| 0.682| 65766| 108899| 0.604 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎⇥⁺| ⏎␣⁻␣⁻| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⏎| ⏎⏎⇥⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8631 |51618 |390 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|14217 |235 |10721 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5343 |0 |0 |1627 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5744 |36 |180 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3419 |0 |0 |0 |0 |732 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1248 |13 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1029 |0 |41 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|912 |1 |71 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|959 |7 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|695 |10 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|243 |2 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|214 |20 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|148 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|137 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|97 |1 |21 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|97 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| wordpress/apache/revslider/rs-plugin/pluginsources/jquery.themepunch.revolution.js | 280 |
| wordpress/apache/revslider/js/codemirror/codemirror.js | 229 |
| wordpress/apache/revslider/js/dropdownchecklist/ui.dropdownchecklist.js | 151 |
| wordpress/apache/revslider/js/edit_layers.js | 104 |
| wordpress/apache/revslider/js/rev_admin.js | 55 |
| wordpress/apache/revslider/js/admin.js | 32 |
| wordpress/apache/revslider/js/farbtastic/farbtastic.js | 29 |
| wordpress/apache/revslider/js/farbtastic/my-farbtastic.js | 28 |
| wordpress/apache/revslider/js/codemirror/util/foldcode.js | 19 |
| wordpress/apache/revslider/js/codemirror/util/formatting.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9993857493857494, "precision": 1.0, "recall": 0.9987722529158993, "support": 1629}, "\u0027": {"f1-score": 0.9986357435197818, "precision": 0.997275204359673, "recall": 1.0, "support": 732}, "macro avg": {"f1-score": 0.24674406068884286, "precision": 0.24543330048907963, "recall": 0.24811399828028308, "support": 65766}, "micro avg": {"f1-score": 0.9837606057841438, "precision": 0.9837606057841438, "recall": 0.9837606057841438, "support": 65766}, "weighted avg": {"f1-score": 0.9805679089452262, "precision": 0.9775326213752907, "recall": 0.9837606057841438, "support": 65766}, "\u2205": {"f1-score": 0.9930357829934591, "precision": 0.993570988604866, "recall": 0.9925011536686663, "support": 52008}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 216}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.9568476951224953, "precision": 0.9360866148607352, "recall": 0.9785505658999635, "support": 10956}},
  "cl_report_full": {"\"": {"f1-score": 0.37841609489475525, "precision": 1.0, "recall": 0.23336201950659782, "support": 6972}, "\u0027": {"f1-score": 0.29969293756397136, "precision": 0.997275204359673, "recall": 0.1763430498675018, "support": 4151}, "macro avg": {"f1-score": 0.13627823754075752, "precision": 0.24543330048907963, "recall": 0.1054270153235404, "support": 108899}, "micro avg": {"f1-score": 0.7408238628231186, "precision": 0.9837606057841438, "recall": 0.594110138752422, "support": 108899}, "weighted avg": {"f1-score": 0.6815490097314368, "precision": 0.8716787932597797, "recall": 0.594110138752422, "support": 108899}, "\u2205": {"f1-score": 0.9169116536845751, "precision": 0.993570988604866, "recall": 0.8512343541285311, "support": 60639}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5960}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 984}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 706}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1277}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 246}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 234}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1070}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 119}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 974}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u2423": {"f1-score": 0.5854311145088188, "precision": 0.9360866148607352, "recall": 0.4258928216740158, "support": 25173}},
  "ppcr": 0.6039173913442731
}
```
</details>
