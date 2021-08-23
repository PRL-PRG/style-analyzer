# Train report for javascript / file:///tmp/top-repos-quality-repos-7_kmrrgz/mac-torrents.git HEAD b533ade849aefb261185fab7c941c843a6e9c046

### Classification report

PPCR: 0.260

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.941| 1.000| 0.970| 271| 288| 0.941 |
| `␣` | 0.984| 1.000| 0.408| 0.992| 0.576| 249| 611| 0.408 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 846| 0.002 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 175| 0.011 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 97| 0.000 |
| `weighted avg` | 0.985| 0.992| 0.258| 0.989| 0.313| 524| 2017| 0.260 |
| `micro avg` | 0.992| 0.992| 0.258| 0.992| 0.409| 524| 2017| 0.260 |
| `macro avg` | 0.397| 0.400| 0.270| 0.398| 0.309| 524| 2017| 0.260 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|844 |0 |2 |0 |0 |0 |
|362 |0 |249 |0 |0 |0 |
|17 |0 |0 |271 |0 |0 |
|173 |0 |2 |0 |0 |0 |
|97 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/src/menu/categoryMenu.js | 1 |
| src/bin/index.js | 1 |
| src/mac-torrents.js | 1 |
| src/src/menu/selectTorrents.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 271}, "macro avg": {"f1-score": 0.398406374501992, "precision": 0.39683794466403166, "recall": 0.4, "support": 524}, "micro avg": {"f1-score": 0.9923664122137404, "precision": 0.9923664122137404, "recall": 0.9923664122137404, "support": 524}, "weighted avg": {"f1-score": 0.9885800310209543, "precision": 0.9848535135624416, "recall": 0.9923664122137404, "support": 524}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.99203187250996, "precision": 0.9841897233201581, "recall": 1.0, "support": 249}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9695885509838997, "precision": 1.0, "recall": 0.9409722222222222, "support": 288}, "macro avg": {"f1-score": 0.3091954879745577, "precision": 0.39683794466403166, "recall": 0.2697001727586834, "support": 2017}, "micro avg": {"f1-score": 0.40928768201495475, "precision": 0.9923664122137404, "recall": 0.25780862667327714, "support": 2017}, "weighted avg": {"f1-score": 0.31304666028481615, "precision": 0.4409221224336225, "recall": 0.25780862667327714, "support": 2017}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 846}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 175}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u2423": {"f1-score": 0.5763888888888888, "precision": 0.9841897233201581, "recall": 0.4075286415711948, "support": 611}},
  "ppcr": 0.2597917699553793
}
```
</details>
