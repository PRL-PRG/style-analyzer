# Train report for javascript / file:///tmp/top-repos-quality-repos-9dd21pez/engine.io.git HEAD 7e9e544a9ac4365ef055c9482cc1e9fed624031c

### Classification report

PPCR: 0.910

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.927| 0.999| 0.990| 0.962| 0.958| 16304| 16443| 0.992 |
| `␣` | 0.954| 0.787| 0.697| 0.863| 0.805| 5975| 6752| 0.885 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 2178| 2491| 0.874 |
| `⏎␣⁺␣⁺` | 0.901| 0.966| 0.942| 0.932| 0.921| 1014| 1040| 0.975 |
| `⏎␣⁻␣⁻` | 0.989| 0.985| 0.879| 0.987| 0.931| 887| 994| 0.892 |
| `⏎⏎` | 0.943| 0.638| 0.324| 0.761| 0.483| 362| 712| 0.508 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 167| 1120| 0.149 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.589| 0.547| 0.479| 0.563| 0.512| 26887| 29552| 0.910 |
| `weighted avg` | 0.854| 0.858| 0.781| 0.853| 0.792| 26887| 29552| 0.910 |
| `micro avg` | 0.858| 0.858| 0.781| 0.858| 0.818| 26887| 29552| 0.910 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|139 |16285 |17 |0 |0 |2 |0 |0 |0 |
|777 |1155 |4704 |0 |0 |106 |10 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|953 |60 |93 |0 |0 |0 |0 |14 |0 |
|26 |30 |4 |0 |0 |980 |0 |0 |0 |
|107 |5 |8 |0 |0 |0 |874 |0 |0 |
|350 |28 |103 |0 |0 |0 |0 |231 |0 |
|313 |0 |0 |2178 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/server.js | 2466 |
| lib/socket.js | 241 |
| lib/server.js | 237 |
| test/jsonp.js | 234 |
| lib/transports/polling.js | 187 |
| test/engine.io.js | 170 |
| lib/transports/websocket.js | 53 |
| examples/latency/public/index.js | 50 |
| lib/transport.js | 48 |
| lib/engine.io.js | 33 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2178}, "macro avg": {"f1-score": 0.5631355393599983, "precision": 0.5892331754561935, "recall": 0.5470062258995362, "support": 26887}, "micro avg": {"f1-score": 0.8581842526127869, "precision": 0.8581842526127869, "recall": 0.8581842526127869, "support": 26887}, "weighted avg": {"f1-score": 0.85287905822156, "precision": 0.8536278156516119, "recall": 0.8581842526127869, "support": 26887}, "\u2205": {"f1-score": 0.9617031328431807, "precision": 0.9272333883732847, "recall": 0.9988346418056918, "support": 16304}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 167}, "\u23ce\u23ce": {"f1-score": 0.7611202635914333, "precision": 0.9428571428571428, "recall": 0.638121546961326, "support": 362}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9324452901998097, "precision": 0.9007352941176471, "recall": 0.9664694280078896, "support": 1014}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.987012987012987, "precision": 0.9886877828054299, "recall": 0.9853438556933484, "support": 887}, "\u2423": {"f1-score": 0.8628026412325753, "precision": 0.9543517954960439, "recall": 0.7872803347280335, "support": 5975}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2491}, "macro avg": {"f1-score": 0.5122214160737552, "precision": 0.5892331754561935, "recall": 0.47913688259946413, "support": 29552}, "micro avg": {"f1-score": 0.8176615460940129, "precision": 0.8581842526127869, "recall": 0.7807931781266919, "support": 29552}, "weighted avg": {"f1-score": 0.7922844779181588, "precision": 0.8216403822386453, "recall": 0.7807931781266919, "support": 29552}, "\u2205": {"f1-score": 0.9577721578544962, "precision": 0.9272333883732847, "recall": 0.9903910478623122, "support": 16443}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1120}, "\u23ce\u23ce": {"f1-score": 0.48275862068965514, "precision": 0.9428571428571428, "recall": 0.324438202247191, "support": 712}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9210526315789475, "precision": 0.9007352941176471, "recall": 0.9423076923076923, "support": 1040}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9307774227902024, "precision": 0.9886877828054299, "recall": 0.8792756539235412, "support": 994}, "\u2423": {"f1-score": 0.80541049567674, "precision": 0.9543517954960439, "recall": 0.6966824644549763, "support": 6752}},
  "ppcr": 0.9098199783432593
}
```
</details>
