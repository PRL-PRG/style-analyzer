# Train report for javascript / file:///tmp/top-repos-quality-repos-gmcdxd0m/aleph.git HEAD 42b2afc8ef9c63033c20ff335e70fd8e7b75b1ed

### Classification report

PPCR: 0.794

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.995| 0.927| 0.990| 0.956| 8164| 8759| 0.932 |
| `␣` | 0.972| 0.984| 0.682| 0.978| 0.802| 4125| 5950| 0.693 |
| `'` | 1.000| 1.000| 0.994| 1.000| 0.997| 842| 847| 0.994 |
| `⏎␣⁺␣⁺` | 0.948| 0.866| 0.636| 0.905| 0.762| 380| 517| 0.735 |
| `⏎␣⁻␣⁻` | 0.986| 0.854| 0.600| 0.915| 0.746| 335| 477| 0.702 |
| `⏎` | 0.935| 0.844| 0.282| 0.887| 0.433| 307| 918| 0.334 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 384| 0.044 |
| `macro avg` | 0.832| 0.792| 0.589| 0.811| 0.671| 14170| 17852| 0.794 |
| `weighted avg` | 0.979| 0.981| 0.778| 0.980| 0.848| 14170| 17852| 0.794 |
| `micro avg` | 0.981| 0.981| 0.778| 0.981| 0.868| 14170| 17852| 0.794 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|595 |8122 |42 |0 |0 |0 |0 |0 |
|1825 |44 |4059 |0 |0 |18 |4 |0 |
|611 |16 |32 |259 |0 |0 |0 |0 |
|5 |0 |0 |0 |842 |0 |0 |0 |
|137 |35 |16 |0 |0 |329 |0 |0 |
|142 |20 |28 |1 |0 |0 |286 |0 |
|367 |0 |0 |17 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| ui/src/components/MappingEditor/MappingList.js | 39 |
| ui/src/app/Query.js | 23 |
| ui/src/components/AdvancedSearch/util.js | 21 |
| ui/src/queries.js | 15 |
| ui/src/reducers/util.js | 14 |
| ui/src/reducers/results.js | 11 |
| ui/src/selectors.js | 9 |
| ui/src/actions/entityMappingActions.js | 8 |
| ui/src/app/toast.js | 8 |
| ui/src/actions/index.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 842}, "macro avg": {"f1-score": 0.8107913983891903, "precision": 0.8324486306266063, "recall": 0.7917177840635967, "support": 14170}, "micro avg": {"f1-score": 0.9807339449541285, "precision": 0.9807339449541285, "recall": 0.9807339449541285, "support": 14170}, "weighted avg": {"f1-score": 0.9798346094478245, "precision": 0.9794076369038645, "recall": 0.9807339449541285, "support": 14170}, "\u2205": {"f1-score": 0.9904274129626244, "precision": 0.9860386062886973, "recall": 0.9948554630083293, "support": 8164}, "\u23ce": {"f1-score": 0.8869863013698631, "precision": 0.9350180505415162, "recall": 0.8436482084690554, "support": 307}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9050894085281981, "precision": 0.9481268011527377, "recall": 0.8657894736842106, "support": 380}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9151999999999999, "precision": 0.9862068965517241, "recall": 0.8537313432835821, "support": 335}, "\u2423": {"f1-score": 0.9778366658636473, "precision": 0.9717500598515681, "recall": 0.984, "support": 4125}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9970396684428656, "precision": 1.0, "recall": 0.9940968122786304, "support": 847}, "macro avg": {"f1-score": 0.670746140865039, "precision": 0.8324486306266063, "recall": 0.5888051306703629, "support": 17852}, "micro avg": {"f1-score": 0.8679657735306977, "precision": 0.9807339449541285, "recall": 0.7784561953842707, "support": 17852}, "weighted avg": {"f1-score": 0.8476904963622212, "precision": 0.9570117535765059, "recall": 0.7784561953842707, "support": 17852}, "\u2205": {"f1-score": 0.9557542951282656, "precision": 0.9860386062886973, "recall": 0.92727480305971, "support": 8759}, "\u23ce": {"f1-score": 0.4334728033472804, "precision": 0.9350180505415162, "recall": 0.2821350762527233, "support": 918}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 384}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7615740740740741, "precision": 0.9481268011527377, "recall": 0.6363636363636364, "support": 517}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7457627118644068, "precision": 0.9862068965517241, "recall": 0.59958071278826, "support": 477}, "\u2423": {"f1-score": 0.8016194331983806, "precision": 0.9717500598515681, "recall": 0.6821848739495798, "support": 5950}},
  "ppcr": 0.7937485995966839
}
```
</details>
