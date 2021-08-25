# Train report for javascript / file:///tmp/top-repos-quality-repos-udpselns/nc.js.git HEAD 8344a77c68bc97791d20b9db902e275ed7473531

### Classification report

PPCR: 0.622

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 0.974| 0.846| 0.966| 0.899| 36014| 41466| 0.869 |
| `␣` | 0.894| 0.870| 0.454| 0.882| 0.603| 9638| 18450| 0.522 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 136| 4656| 0.029 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 545| 0.090 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 969| 0.045 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 613| 0.060 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 1502| 0.018 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 1427| 0.017 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1845| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2406| 0.000 |
| `micro avg` | 0.945| 0.945| 0.588| 0.945| 0.725| 45969| 73879| 0.622 |
| `macro avg` | 0.185| 0.184| 0.130| 0.185| 0.150| 45969| 73879| 0.622 |
| `weighted avg` | 0.938| 0.945| 0.588| 0.942| 0.655| 45969| 73879| 0.622 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5452 |35077 |937 |0 |0 |0 |0 |0 |0 |0 |0 |
|8812 |1253 |8385 |0 |0 |0 |0 |0 |0 |0 |0 |
|4520 |105 |31 |0 |0 |0 |0 |0 |0 |0 |0 |
|1845 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|576 |34 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|2406 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|496 |49 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1475 |21 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|1403 |24 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|925 |26 |18 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/MTclient/views/compass/compass.js | 349 |
| src/client/views/compass/compass.js | 349 |
| src/MTclient/views/geometry/trackball_controls.js | 319 |
| src/client/views/geometry/trackball_controls.js | 319 |
| src/MTclient/models/webworker.js | 99 |
| src/client/models/webworker.js | 99 |
| src/MTclient/models/nc.js | 92 |
| src/client/models/nc.js | 92 |
| src/server/api/v3/statecache.js | 64 |
| src/server/api/v3/MTstate.js | 62 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.18480648045701228, "precision": 0.1852599345778419, "recall": 0.1843976114843075, "support": 45969}, "micro avg": {"f1-score": 0.9454632469707848, "precision": 0.9454632469707847, "recall": 0.9454632469707847, "support": 45969}, "weighted avg": {"f1-score": 0.9418945263787336, "precision": 0.9384888389105464, "recall": 0.9454632469707847, "support": 45969}, "\u2205": {"f1-score": 0.9662686114898832, "precision": 0.9586761048402526, "recall": 0.9739823402010329, "support": 36014}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u2423": {"f1-score": 0.8817961930802398, "precision": 0.8939232409381663, "recall": 0.869993774642042, "support": 9638}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2406}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1845}, "macro avg": {"f1-score": 0.15013636399313318, "precision": 0.1852599345778419, "recall": 0.13003935048755783, "support": 73879}, "micro avg": {"f1-score": 0.7252853614578466, "precision": 0.9454632469707847, "recall": 0.5882862518442318, "support": 73879}, "weighted avg": {"f1-score": 0.6549411763421575, "precision": 0.7613171152643523, "recall": 0.5882862518442318, "support": 73879}, "\u2205": {"f1-score": 0.8987765037473577, "precision": 0.9586761048402526, "recall": 0.8459219601601312, "support": 41466}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4656}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 969}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 613}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1502}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 545}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1427}, "\u2423": {"f1-score": 0.6025871361839741, "precision": 0.8939232409381663, "recall": 0.45447154471544715, "support": 18450}},
  "ppcr": 0.6222201166772696
}
```
</details>
