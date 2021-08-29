# Train report for javascript / file:///tmp/top-repos-quality-repos-82qvy_7y/sockjs-node.git HEAD 9efed2c754226b702eb468ef0643b316b91bf37f

### Classification report

PPCR: 0.589

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.977| 0.728| 0.985| 0.840| 4207| 5647| 0.745 |
| `␣` | 0.916| 0.964| 0.372| 0.939| 0.529| 899| 2328| 0.386 |
| `'` | 1.000| 1.000| 0.739| 1.000| 0.850| 709| 959| 0.739 |
| `⏎␣⁺␣⁺` | 0.919| 0.997| 0.941| 0.956| 0.930| 306| 324| 0.944 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 615| 0.008 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 322| 0.003 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 215| 0.000 |
| `micro avg` | 0.978| 0.978| 0.576| 0.978| 0.725| 6127| 10410| 0.589 |
| `weighted avg` | 0.978| 0.978| 0.576| 0.978| 0.681| 6127| 10410| 0.589 |
| `macro avg` | 0.547| 0.563| 0.397| 0.554| 0.450| 6127| 10410| 0.589 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1440 |4110 |80 |0 |0 |17 |0 |0 |
|1429 |26 |867 |0 |0 |6 |0 |0 |
|250 |0 |0 |709 |0 |0 |0 |0 |
|610 |1 |0 |0 |0 |4 |0 |0 |
|18 |1 |0 |0 |0 |305 |0 |0 |
|321 |1 |0 |0 |0 |0 |0 |0 |
|215 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| tests/test_server/sockjs_app.js | 30 |
| lib/trans-jsonp.js | 10 |
| examples/multiplex/server.js | 9 |
| lib/utils.js | 8 |
| lib/trans-websocket.js | 7 |
| lib/generic-app.js | 7 |
| examples/echo/server.js | 7 |
| lib/trans-xhr.js | 6 |
| examples/koa/server.js | 5 |
| examples/express-3.x/server.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 709}, "macro avg": {"f1-score": 0.5543348682201132, "precision": 0.5467415541076951, "recall": 0.5625828729131971, "support": 6127}, "micro avg": {"f1-score": 0.9778031663130407, "precision": 0.9778031663130407, "recall": 0.9778031663130407, "support": 6127}, "weighted avg": {"f1-score": 0.9775605279050887, "precision": 0.9777530479008961, "recall": 0.9778031663130407, "support": 6127}, "\u2205": {"f1-score": 0.9849029475197699, "precision": 0.9929934766851897, "recall": 0.9769431899215593, "support": 4207}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9561128526645768, "precision": 0.9186746987951807, "recall": 0.9967320261437909, "support": 306}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9393282773564463, "precision": 0.9155227032734953, "recall": 0.96440489432703, "support": 899}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8501199040767385, "precision": 1.0, "recall": 0.7393117831074035, "support": 959}, "macro avg": {"f1-score": 0.449919868125828, "precision": 0.5467415541076951, "recall": 0.3972732242386164, "support": 10410}, "micro avg": {"f1-score": 0.72455705387918, "precision": 0.9778031663130407, "recall": 0.5755043227665706, "support": 10410}, "weighted avg": {"f1-score": 0.6813144106268926, "precision": 0.8641135080184055, "recall": 0.5755043227665706, "support": 10410}, "\u2205": {"f1-score": 0.8399754751686082, "precision": 0.9929934766851897, "recall": 0.7278200814591819, "support": 5647}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 615}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 215}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9298780487804879, "precision": 0.9186746987951807, "recall": 0.941358024691358, "support": 324}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 322}, "\u2423": {"f1-score": 0.5294656488549618, "precision": 0.9155227032734953, "recall": 0.37242268041237114, "support": 2328}},
  "ppcr": 0.588568683957733
}
```
</details>
