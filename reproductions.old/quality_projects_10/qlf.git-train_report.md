# Train report for javascript / file:///tmp/top-repos-quality-repos-o1r3q964/qlf.git HEAD a9c455f7aee41d7901c89ae90dd821c617340a86

### Classification report

PPCR: 0.757

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 0.999| 0.861| 0.985| 0.913| 31738| 36813| 0.862 |
| `␣` | 0.981| 0.970| 0.560| 0.975| 0.713| 8076| 13974| 0.578 |
| `'` | 1.000| 1.000| 0.983| 1.000| 0.991| 6010| 6116| 0.983 |
| `⏎` | 0.959| 0.947| 0.569| 0.953| 0.715| 2959| 4922| 0.601 |
| `⏎␣⁻␣⁻` | 0.965| 0.811| 0.611| 0.881| 0.748| 1782| 2366| 0.753 |
| `⏎␣⁺␣⁺` | 1.000| 0.466| 0.112| 0.636| 0.201| 618| 2578| 0.240 |
| `"` | 1.000| 1.000| 0.810| 1.000| 0.895| 444| 548| 0.810 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 134| 180| 0.744 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 55| 883| 0.062 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 77| 0.026 |
| `macro avg` | 0.688| 0.619| 0.451| 0.643| 0.518| 51818| 68457| 0.757 |
| `micro avg` | 0.975| 0.975| 0.738| 0.975| 0.840| 51818| 68457| 0.757 |
| `weighted avg` | 0.972| 0.975| 0.738| 0.972| 0.817| 51818| 68457| 0.757 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5075 |31710 |28 |0 |0 |0 |0 |0 |0 |0 |0 |
|5898 |167 |7830 |0 |79 |0 |0 |0 |0 |0 |0 |
|106 |0 |0 |6010 |0 |0 |0 |0 |0 |0 |0 |
|1963 |116 |40 |0 |2803 |0 |0 |0 |0 |0 |0 |
|1960 |261 |69 |0 |0 |288 |0 |0 |0 |0 |0 |
|584 |308 |11 |0 |18 |0 |1445 |0 |0 |0 |0 |
|828 |35 |0 |0 |20 |0 |0 |0 |0 |0 |0 |
|104 |0 |0 |0 |0 |0 |0 |0 |444 |0 |0 |
|46 |79 |1 |0 |2 |0 |52 |0 |0 |0 |0 |
|75 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/tests/screens/history/widgets/table-history/table-history.test.js | 110 |
| frontend/src/screens/trend-analysis/trend-analysis.js | 61 |
| frontend/src/containers/offline/offline-container.js | 58 |
| frontend/src/screens/history/widgets/table-history/history-header/history-header.js | 56 |
| frontend/src/containers/offline/connection/qlf-api.js | 53 |
| frontend/src/screens/history/widgets/table-history/history-data/history-data.js | 43 |
| frontend/src/screens/history/widgets/table-history/table-history.js | 42 |
| frontend/src/screens/history/history.js | 42 |
| frontend/src/screens/observing-conditions/observing-conditions.js | 41 |
| frontend/src/containers/online/online-container.js | 37 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 444}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6010}, "macro avg": {"f1-score": 0.6430302180178351, "precision": 0.6876241935385792, "recall": 0.6192842700734856, "support": 51818}, "micro avg": {"f1-score": 0.9751437724342893, "precision": 0.9751437724342893, "recall": 0.9751437724342893, "support": 51818}, "weighted avg": {"f1-score": 0.9719152452508236, "precision": 0.9717400711964462, "recall": 0.9751437724342893, "support": 51818}, "\u2205": {"f1-score": 0.984538002980626, "precision": 0.9703776240896016, "recall": 0.9991177767975298, "support": 31738}, "\u23ce": {"f1-score": 0.953239245026356, "precision": 0.95927446954141, "recall": 0.9472794863129436, "support": 2959}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6357615894039735, "precision": 1.0, "recall": 0.46601941747572817, "support": 618}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8813662702043307, "precision": 0.9652638610554443, "recall": 0.8108866442199776, "support": 1782}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 134}, "\u2423": {"f1-score": 0.9753970725630645, "precision": 0.9813259806993357, "recall": 0.9695393759286776, "support": 8076}},
  "cl_report_full": {"\"": {"f1-score": 0.8951612903225806, "precision": 1.0, "recall": 0.8102189781021898, "support": 548}, "\u0027": {"f1-score": 0.9912584529111002, "precision": 1.0, "recall": 0.9826684107259647, "support": 6116}, "macro avg": {"f1-score": 0.5176184628154104, "precision": 0.6876241935385792, "recall": 0.45065280751366227, "support": 68457}, "micro avg": {"f1-score": 0.84024111411349, "precision": 0.9751437724342893, "recall": 0.7381275837386972, "support": 68457}, "weighted avg": {"f1-score": 0.8169225643045901, "precision": 0.959477101892327, "recall": 0.7381275837386972, "support": 68457}, "\u2205": {"f1-score": 0.9126361687124951, "precision": 0.9703776240896016, "recall": 0.8613804905875642, "support": 36813}, "\u23ce": {"f1-score": 0.7146863844977054, "precision": 0.95927446954141, "recall": 0.5694839496139781, "support": 4922}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 883}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.20097697138869505, "precision": 1.0, "recall": 0.11171450737005431, "support": 2578}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7481232202951076, "precision": 0.9652638610554443, "recall": 0.6107354184277262, "support": 2366}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u2423": {"f1-score": 0.71334214002642, "precision": 0.9813259806993357, "recall": 0.5603263203091455, "support": 13974}},
  "ppcr": 0.7569423141534102
}
```
</details>
