# Train report for javascript / file:///tmp/top-repos-quality-repos-vry1bsel/redux.git HEAD ca9463dd692ddcf85073bca921fd05f970bba6cf

### Classification report

PPCR: 0.982

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.934| 0.965| 0.965| 0.949| 0.949| 13954| 13958| 1.000 |
| `␣` | 0.879| 0.921| 0.920| 0.900| 0.899| 8995| 9001| 0.999 |
| `'` | 1.000| 1.000| 0.971| 1.000| 0.985| 2824| 2908| 0.971 |
| `⏎` | 0.865| 0.690| 0.604| 0.767| 0.711| 1508| 1723| 0.875 |
| `⏎␣⁺␣⁺` | 0.846| 0.645| 0.645| 0.732| 0.732| 1308| 1309| 0.999 |
| `⏎␣⁻␣⁻` | 0.870| 0.759| 0.759| 0.810| 0.810| 1161| 1161| 1.000 |
| `⏎⏎` | 0.883| 0.588| 0.364| 0.706| 0.515| 400| 646| 0.619 |
| `weighted avg` | 0.913| 0.915| 0.898| 0.912| 0.901| 30150| 30706| 0.982 |
| `macro avg` | 0.897| 0.795| 0.747| 0.838| 0.800| 30150| 30706| 0.982 |
| `micro avg` | 0.915| 0.915| 0.898| 0.915| 0.906| 30150| 30706| 0.982 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|4 |13468 |402 |0 |28 |21 |35 |0 |
|6 |372 |8283 |0 |99 |133 |97 |11 |
|84 |0 |0 |2824 |0 |0 |0 |0 |
|215 |158 |290 |0 |1040 |0 |0 |20 |
|1 |149 |313 |0 |2 |844 |0 |0 |
|0 |238 |42 |0 |0 |0 |881 |0 |
|246 |40 |91 |0 |34 |0 |0 |235 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| examples/todomvc/src/reducers/todos.spec.js | 81 |
| rollup.config.js | 77 |
| examples/real-world/src/actions/index.js | 63 |
| examples/todomvc/src/reducers/todos.js | 52 |
| examples/async/src/containers/App.js | 44 |
| examples/real-world/src/middleware/api.js | 44 |
| examples/todomvc/src/components/Footer.spec.js | 43 |
| examples/todos-flow/src/reducers/todos.spec.js | 43 |
| examples/real-world/src/containers/UserPage.js | 43 |
| examples/tree-view/src/reducers/index.js | 43 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2824}, "macro avg": {"f1-score": 0.8377339616972524, "precision": 0.8966017550944333, "recall": 0.7953228425976872, "support": 30150}, "micro avg": {"f1-score": 0.9145936981757877, "precision": 0.9145936981757877, "recall": 0.9145936981757877, "support": 30150}, "weighted avg": {"f1-score": 0.9120263823271817, "precision": 0.9132219735095619, "recall": 0.9145936981757877, "support": 30150}, "\u2205": {"f1-score": 0.9491525423728814, "precision": 0.9336568457538995, "recall": 0.9651712770531747, "support": 13954}, "\u23ce": {"f1-score": 0.7672445592032461, "precision": 0.8645054031587698, "recall": 0.6896551724137931, "support": 1508}, "\u23ce\u23ce": {"f1-score": 0.7057057057057058, "precision": 0.8834586466165414, "recall": 0.5875, "support": 400}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7320034692107545, "precision": 0.845691382765531, "recall": 0.6452599388379205, "support": 1308}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8104875804967802, "precision": 0.8696939782823297, "recall": 0.7588285960378983, "support": 1161}, "\u2423": {"f1-score": 0.8995438748913989, "precision": 0.8792060290839614, "recall": 0.9208449138410227, "support": 8995}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9853454291695743, "precision": 1.0, "recall": 0.9711141678129298, "support": 2908}, "macro avg": {"f1-score": 0.8002868304545151, "precision": 0.8966017550944333, "recall": 0.7467444279652635, "support": 30706}, "micro avg": {"f1-score": 0.9062376758248981, "precision": 0.9145936981757877, "recall": 0.8980329577281313, "support": 30706}, "weighted avg": {"f1-score": 0.9008806612108176, "precision": 0.9128736255129414, "recall": 0.8980329577281313, "support": 30706}, "\u2205": {"f1-score": 0.9490187788464927, "precision": 0.9336568457538995, "recall": 0.9648946840521565, "support": 13958}, "\u23ce": {"f1-score": 0.7108680792891321, "precision": 0.8645054031587698, "recall": 0.6035983749274522, "support": 1723}, "\u23ce\u23ce": {"f1-score": 0.5153508771929824, "precision": 0.8834586466165414, "recall": 0.3637770897832817, "support": 646}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.731686172518422, "precision": 0.845691382765531, "recall": 0.6447669977081741, "support": 1309}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8104875804967802, "precision": 0.8696939782823297, "recall": 0.7588285960378983, "support": 1161}, "\u2423": {"f1-score": 0.8992508956682227, "precision": 0.8792060290839614, "recall": 0.9202310854349517, "support": 9001}},
  "ppcr": 0.9818927896827981
}
```
</details>
