# Train report for javascript / file:///tmp/top-repos-quality-repos-h1ss_7yc/rov-gui.git HEAD a725c430eed689736c456696e2a69f1be8b8121c

### Classification report

PPCR: 0.422

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.920| 1.000| 0.642| 0.959| 0.756| 2275| 3543| 0.642 |
| `'` | 1.000| 1.000| 0.330| 1.000| 0.496| 124| 376| 0.330 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 116| 1387| 0.084 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 167| 0.353 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 171| 0.082 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 273| 0.029 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 142| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 86| 0.000 |
| `macro avg` | 0.240| 0.250| 0.121| 0.245| 0.157| 2596| 6145| 0.422 |
| `micro avg` | 0.924| 0.924| 0.390| 0.924| 0.549| 2596| 6145| 0.422 |
| `weighted avg` | 0.854| 0.924| 0.390| 0.888| 0.466| 2596| 6145| 0.422 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1268 |2275 |0 |0 |0 |0 |0 |0 |0 |
|1271 |116 |0 |0 |0 |0 |0 |0 |0 |
|252 |0 |0 |124 |0 |0 |0 |0 |0 |
|265 |8 |0 |0 |0 |0 |0 |0 |0 |
|108 |59 |0 |0 |0 |0 |0 |0 |0 |
|157 |14 |0 |0 |0 |0 |0 |0 |0 |
|142 |0 |0 |0 |0 |0 |0 |0 |0 |
|86 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/App.js | 43 |
| src/registerServiceWorker.js | 25 |
| src/electron-starter.js | 16 |
| src/components/obs-tab/ObsTab.js | 16 |
| src/components/obs-tab/Seismograph.js | 14 |
| src/components/aircraft-id-tab/AircraftIdTab.js | 12 |
| src/components/obs-tab/BubbleLevel.js | 12 |
| src/utils/rovMock.js | 11 |
| src/components/search-zone-tab/SearchZoneOutput.js | 8 |
| src/components/search-zone-tab/SearchZoneTab.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 124}, "macro avg": {"f1-score": 0.24481251316621022, "precision": 0.240038430420712, "recall": 0.25, "support": 2596}, "micro avg": {"f1-score": 0.9241140215716487, "precision": 0.9241140215716487, "recall": 0.9241140215716487, "support": 2596}, "weighted avg": {"f1-score": 0.8877456624133383, "precision": 0.8542755907769483, "recall": 0.9241140215716487, "support": 2596}, "\u2205": {"f1-score": 0.9585001053296819, "precision": 0.9203074433656958, "recall": 1.0, "support": 2275}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "\u0027": {"f1-score": 0.496, "precision": 1.0, "recall": 0.32978723404255317, "support": 376}, "macro avg": {"f1-score": 0.15655527847049044, "precision": 0.240038430420712, "recall": 0.12148730490448652, "support": 6145}, "micro avg": {"f1-score": 0.5489074476604509, "precision": 0.9241140215716487, "recall": 0.3903986981285598, "support": 6145}, "weighted avg": {"f1-score": 0.46648833408748275, "precision": 0.5918062281276908, "recall": 0.3903986981285598, "support": 6145}, "\u2205": {"f1-score": 0.7564422277639236, "precision": 0.9203074433656958, "recall": 0.642111205193339, "support": 3543}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 273}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 171}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 167}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1387}},
  "ppcr": 0.4224572823433686
}
```
</details>
