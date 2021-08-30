# Train report for javascript / file:///tmp/top-repos-quality-repos-2rdmnx0m/cursedchrome.git HEAD f18eaee6e3aee866b801235e836d9576aff008de

### Classification report

PPCR: 0.629

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.997| 0.947| 0.980| 0.955| 13375| 14091| 0.949 |
| `␣` | 0.978| 0.889| 0.368| 0.932| 0.535| 2793| 6746| 0.414 |
| `⏎` | 0.938| 0.832| 0.222| 0.882| 0.359| 457| 1712| 0.267 |
| `'` | 1.000| 1.000| 0.227| 1.000| 0.370| 390| 1719| 0.227 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 53| 341| 0.155 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 736| 0.057 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 329| 0.097 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 777| 0.024 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 706| 0.020 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 144| 0.000 |
| `weighted avg` | 0.957| 0.966| 0.608| 0.961| 0.671| 17175| 27301| 0.629 |
| `macro avg` | 0.388| 0.372| 0.176| 0.379| 0.222| 17175| 27301| 0.629 |
| `micro avg` | 0.966| 0.966| 0.608| 0.966| 0.746| 17175| 27301| 0.629 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|716 |13338 |37 |0 |0 |0 |0 |0 |0 |0 |0 |
|3953 |286 |2483 |0 |24 |0 |0 |0 |0 |0 |0 |
|1329 |0 |0 |390 |0 |0 |0 |0 |0 |0 |0 |
|1255 |66 |11 |0 |380 |0 |0 |0 |0 |0 |0 |
|758 |17 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|694 |42 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|692 |8 |5 |0 |1 |0 |0 |0 |0 |0 |0 |
|288 |53 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|297 |32 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|144 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| anyproxy/lib/requestHandler.js | 119 |
| extension/src/bg/background.js | 69 |
| server.js | 58 |
| api-server.js | 44 |
| anyproxy/lib/util.js | 32 |
| anyproxy/lib/systemProxyMgr.js | 27 |
| anyproxy/proxy.js | 25 |
| cookie-sync-extension/src/browser_action/main.js | 24 |
| anyproxy/lib/recorder.js | 22 |
| anyproxy/lib/certMgr.js | 18 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 390}, "macro avg": {"f1-score": 0.3793325796550603, "precision": 0.3880190074884299, "recall": 0.37177517265598425, "support": 17175}, "micro avg": {"f1-score": 0.9659970887918486, "precision": 0.9659970887918486, "recall": 0.9659970887918486, "support": 17175}, "weighted avg": {"f1-score": 0.9609219900382601, "precision": 0.9571626212459394, "recall": 0.9659970887918486, "support": 17175}, "\u2205": {"f1-score": 0.98012271741926, "precision": 0.9635890767230169, "recall": 0.997233644859813, "support": 13375}, "\u23ce": {"f1-score": 0.8816705336426913, "precision": 0.9382716049382716, "recall": 0.8315098468271335, "support": 457}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u2423": {"f1-score": 0.9315325454886513, "precision": 0.9783293932230103, "recall": 0.8890082348728965, "support": 2793}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 144}, "\u0027": {"f1-score": 0.36984352773826457, "precision": 1.0, "recall": 0.2268760907504363, "support": 1719}, "macro avg": {"f1-score": 0.22187403241779086, "precision": 0.3880190074884299, "recall": 0.17634703100471733, "support": 27301}, "micro avg": {"f1-score": 0.7460652936415146, "precision": 0.9659970887918486, "recall": 0.6077066774110839, "support": 27301}, "weighted avg": {"f1-score": 0.6708798579679929, "precision": 0.8608865885660151, "recall": 0.6077066774110839, "support": 27301}, "\u2205": {"f1-score": 0.9549994630007519, "precision": 0.9635890767230169, "recall": 0.9465616350862253, "support": 14091}, "\u23ce": {"f1-score": 0.3589985829003307, "precision": 0.9382716049382716, "recall": 0.2219626168224299, "support": 1712}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 706}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 777}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 329}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 736}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 341}, "\u2423": {"f1-score": 0.5348987505385611, "precision": 0.9783293932230103, "recall": 0.36806996738808184, "support": 6746}},
  "ppcr": 0.6290978352441302
}
```
</details>
