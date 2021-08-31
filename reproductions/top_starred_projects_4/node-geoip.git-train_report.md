# Train report for javascript / file:///tmp/top-repos-quality-repos-ef8spp88/node-geoip.git HEAD 78f4c7f0f3b6fa673035b3cb48c3a2a95be9fb0a

### Classification report

PPCR: 0.631

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.928| 0.996| 0.931| 0.961| 0.929| 5020| 5373| 0.934 |
| `␣` | 0.965| 0.711| 0.292| 0.819| 0.448| 1054| 2568| 0.410 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 57| 289| 0.197 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 405| 0.047 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 274| 0.058 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 434| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 254| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 168| 0.000 |
| `macro avg` | 0.237| 0.213| 0.153| 0.222| 0.172| 6166| 9765| 0.631 |
| `weighted avg` | 0.921| 0.933| 0.589| 0.922| 0.629| 6166| 9765| 0.631 |
| `micro avg` | 0.933| 0.933| 0.589| 0.933| 0.722| 6166| 9765| 0.631 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| "| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|353 |5002 |18 |0 |0 |0 |0 |0 |0 |
|1514 |305 |749 |0 |0 |0 |0 |0 |0 |
|386 |18 |1 |0 |0 |0 |0 |0 |0 |
|434 |0 |0 |0 |0 |0 |0 |0 |0 |
|254 |0 |0 |0 |0 |0 |0 |0 |0 |
|168 |0 |0 |0 |0 |0 |0 |0 |0 |
|232 |49 |8 |0 |0 |0 |0 |0 |0 |
|258 |16 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/geoip.js | 208 |
| scripts/updatedb.js | 134 |
| lib/utils.js | 30 |
| test/geo-lookup.js | 18 |
| test/tests.js | 13 |
| .eslintrc.js | 8 |
| lib/fsWatcher.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2224472842947355, "precision": 0.23665262848344587, "recall": 0.21338006607346705, "support": 6166}, "micro avg": {"f1-score": 0.9326954265325982, "precision": 0.9326954265325981, "recall": 0.9326954265325981, "support": 6166}, "weighted avg": {"f1-score": 0.9223155516339505, "precision": 0.9205257586663028, "recall": 0.9326954265325981, "support": 6166}, "\u2205": {"f1-score": 0.9609990393852065, "precision": 0.9280148423005566, "recall": 0.996414342629482, "support": 5020}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8185792349726776, "precision": 0.9652061855670103, "recall": 0.7106261859582542, "support": 1054}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 168}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 434}, "macro avg": {"f1-score": 0.17218089190683838, "precision": 0.23665262848344587, "recall": 0.15282721477759167, "support": 9765}, "micro avg": {"f1-score": 0.7219885757328479, "precision": 0.9326954265325981, "recall": 0.5889400921658986, "support": 9765}, "weighted avg": {"f1-score": 0.6292347573058845, "precision": 0.7644519439034279, "recall": 0.5889400921658986, "support": 9765}, "\u2205": {"f1-score": 0.9294806280776736, "precision": 0.9280148423005566, "recall": 0.9309510515540667, "support": 5373}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 405}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 289}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 274}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 254}, "\u2423": {"f1-score": 0.4479665071770335, "precision": 0.9652061855670103, "recall": 0.2916666666666667, "support": 2568}},
  "ppcr": 0.6314388120839733
}
```
</details>
