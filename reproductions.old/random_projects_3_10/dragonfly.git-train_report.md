# Train report for javascript / file:///tmp/top-repos-quality-repos-2y1_wsr9/dragonfly.git HEAD 891ac39a1ce9f879e56a5ad90964aaba6a7d5564

### Classification report

PPCR: 0.731

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 0.997| 0.929| 0.974| 0.941| 10387| 11146| 0.932 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 1622| 1622| 1.000 |
| `␣` | 0.969| 0.864| 0.269| 0.913| 0.421| 1447| 4645| 0.312 |
| `⏎␣⁻␣⁻` | 0.974| 0.937| 0.802| 0.955| 0.879| 475| 555| 0.856 |
| `⏎` | 1.000| 0.425| 0.118| 0.596| 0.212| 252| 903| 0.279 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 242| 242| 1.000 |
| `⏎␣⁺␣⁺` | 0.926| 0.475| 0.185| 0.627| 0.309| 236| 605| 0.390 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 386| 0.104 |
| `weighted avg` | 0.959| 0.961| 0.703| 0.956| 0.755| 14701| 20104| 0.731 |
| `micro avg` | 0.961| 0.961| 0.703| 0.961| 0.812| 14701| 20104| 0.731 |
| `macro avg` | 0.853| 0.712| 0.538| 0.758| 0.595| 14701| 20104| 0.731 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|759 |10353 |25 |0 |0 |9 |0 |0 |0 |
|3198 |194 |1250 |0 |0 |0 |3 |0 |0 |
|0 |0 |0 |1622 |0 |0 |0 |0 |0 |
|651 |131 |5 |0 |107 |0 |9 |0 |0 |
|369 |120 |4 |0 |0 |112 |0 |0 |0 |
|80 |24 |6 |0 |0 |0 |445 |0 |0 |
|346 |40 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |242 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/client/charts/Graph.js | 125 |
| src/client/components/Modals/BulkAdd/BulkAdd.js | 35 |
| src/server/api/Station.js | 35 |
| src/server/api/Sensor.js | 30 |
| src/server/utilities/Validators.js | 29 |
| src/client/components/SensorDetails/SensorDetails.js | 27 |
| src/client/components/TreeView/TreeView.js | 23 |
| src/client/components/Modals/AddStation/AddStation.js | 21 |
| src/server/api/Reading.js | 20 |
| src/client/components/Modals/AddSensor/AddSensor.js | 19 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 242}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1622}, "macro avg": {"f1-score": 0.7582926974168078, "precision": 0.8526866577619312, "recall": 0.7120755603687245, "support": 14701}, "micro avg": {"f1-score": 0.9612271274062989, "precision": 0.9612271274062989, "recall": 0.9612271274062989, "support": 14701}, "weighted avg": {"f1-score": 0.9563405091751617, "precision": 0.9590751791275682, "recall": 0.9612271274062989, "support": 14701}, "\u2205": {"f1-score": 0.9744458562755894, "precision": 0.9531393850119684, "recall": 0.9967266775777414, "support": 10387}, "\u23ce": {"f1-score": 0.596100278551532, "precision": 1.0, "recall": 0.4246031746031746, "support": 252}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6274509803921569, "precision": 0.9256198347107438, "recall": 0.4745762711864407, "support": 236}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9549356223175965, "precision": 0.973741794310722, "recall": 0.9368421052631579, "support": 475}, "\u2423": {"f1-score": 0.9134088417975885, "precision": 0.9689922480620154, "recall": 0.8638562543192813, "support": 1447}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 242}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1622}, "macro avg": {"f1-score": 0.5952421824463109, "precision": 0.8526866577619312, "recall": 0.5379224555573419, "support": 20104}, "micro avg": {"f1-score": 0.8120097687113919, "precision": 0.9612271274062989, "recall": 0.7028949462793473, "support": 20104}, "weighted avg": {"f1-score": 0.7547403905900518, "precision": 0.9446919654513487, "recall": 0.7028949462793473, "support": 20104}, "\u2205": {"f1-score": 0.9408396946564885, "precision": 0.9531393850119684, "recall": 0.9288534003229858, "support": 11146}, "\u23ce": {"f1-score": 0.21188118811881188, "precision": 1.0, "recall": 0.1184939091915836, "support": 903}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 386}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.3085399449035812, "precision": 0.9256198347107438, "recall": 0.18512396694214875, "support": 605}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8794466403162056, "precision": 0.973741794310722, "recall": 0.8018018018018018, "support": 555}, "\u2423": {"f1-score": 0.4212299915754002, "precision": 0.9689922480620154, "recall": 0.2691065662002153, "support": 4645}},
  "ppcr": 0.7312475129327497
}
```
</details>
