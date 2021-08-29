# Train report for javascript / file:///tmp/top-repos-quality-repos-j6ce3glj/covid19india-cluster.git HEAD dd60c7ee7054665404ee1a4ff7b7074b4776318e

### Classification report

PPCR: 0.538

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 0.993| 0.697| 0.980| 0.811| 3037| 4323| 0.703 |
| `␣` | 0.975| 0.933| 0.346| 0.953| 0.511| 978| 2633| 0.371 |
| `'` | 1.000| 1.000| 0.998| 1.000| 0.999| 495| 496| 0.998 |
| `⏎␣⁻␣⁻` | 0.927| 0.919| 0.688| 0.923| 0.790| 221| 295| 0.749 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 307| 0.068 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 619| 0.015 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 187| 0.021 |
| `micro avg` | 0.971| 0.971| 0.522| 0.971| 0.679| 4765| 8860| 0.538 |
| `weighted avg` | 0.964| 0.971| 0.522| 0.967| 0.630| 4765| 8860| 0.538 |
| `macro avg` | 0.553| 0.549| 0.390| 0.551| 0.444| 4765| 8860| 0.538 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1286 |3015 |16 |0 |0 |0 |6 |0 |
|1655 |56 |912 |0 |0 |0 |10 |0 |
|610 |9 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |495 |0 |0 |0 |
|286 |21 |0 |0 |0 |0 |0 |0 |
|74 |15 |3 |0 |0 |0 |203 |0 |
|183 |0 |4 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| components/NetworkMap/index.js | 21 |
| components/SidePanel/datagrid.js | 14 |
| components/FilterPanel/index.js | 12 |
| components/Redux/reducers.js | 12 |
| components/NetworkMapLegend/index.js | 12 |
| components/DatePicker.js | 11 |
| util/filters/city.js | 11 |
| components/Dashboard/index.js | 9 |
| pages/_document.js | 8 |
| util/filters/travel.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 495}, "macro avg": {"f1-score": 0.5508876056338019, "precision": 0.5528469083341184, "recall": 0.5491176261202904, "support": 4765}, "micro avg": {"f1-score": 0.9706190975865687, "precision": 0.9706190975865687, "recall": 0.9706190975865687, "support": 4765}, "weighted avg": {"f1-score": 0.9669909928069652, "precision": 0.963768474701645, "recall": 0.9706190975865687, "support": 4765}, "\u2205": {"f1-score": 0.9800097513408095, "precision": 0.9675866495507061, "recall": 0.9927560092196246, "support": 3037}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9227272727272728, "precision": 0.9269406392694064, "recall": 0.918552036199095, "support": 221}, "\u2423": {"f1-score": 0.953476215368531, "precision": 0.9754010695187165, "recall": 0.9325153374233128, "support": 978}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9989909182643794, "precision": 1.0, "recall": 0.9979838709677419, "support": 496}, "macro avg": {"f1-score": 0.4443825386706453, "precision": 0.5528469083341184, "recall": 0.3899892516348783, "support": 8860}, "micro avg": {"f1-score": 0.6788990825688073, "precision": 0.9706190975865687, "recall": 0.5220090293453724, "support": 8860}, "weighted avg": {"f1-score": 0.6296530208494738, "precision": 0.8488211727578958, "recall": 0.5220090293453724, "support": 8860}, "\u2205": {"f1-score": 0.8105928216158086, "precision": 0.9675866495507061, "recall": 0.6974323386537127, "support": 4323}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 619}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 307}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7898832684824904, "precision": 0.9269406392694064, "recall": 0.688135593220339, "support": 295}, "\u2423": {"f1-score": 0.5112107623318385, "precision": 0.9754010695187165, "recall": 0.34637295860235473, "support": 2633}},
  "ppcr": 0.5378103837471784
}
```
</details>
