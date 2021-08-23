# Train report for javascript / file:///tmp/top-repos-quality-repos-khao_4x6/hostel_management.git HEAD d7707a5cf93972867bcb77ff643ae10d27859230

### Classification report

PPCR: 0.737

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.985| 0.896| 0.980| 0.934| 51107| 56184| 0.910 |
| `␣` | 0.950| 0.960| 0.819| 0.955| 0.880| 23297| 27289| 0.854 |
| `⏎` | 0.942| 0.927| 0.567| 0.934| 0.708| 4260| 6956| 0.612 |
| `"` | 0.864| 1.000| 0.320| 0.927| 0.467| 1079| 3368| 0.320 |
| `'` | 1.000| 0.835| 0.120| 0.910| 0.214| 1029| 7152| 0.144 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 221| 1848| 0.120 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 135| 783| 0.172 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 132| 2547| 0.052 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 117| 2362| 0.050 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 717| 0.054 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 356| 0.042 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 328| 0.012 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 325| 0.009 |
| `⏎␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 84| 0.024 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 245| 0.004 |
| `macro avg` | 0.315| 0.314| 0.182| 0.314| 0.214| 81441| 110544| 0.737 |
| `weighted avg` | 0.957| 0.965| 0.711| 0.961| 0.765| 81441| 110544| 0.737 |
| `micro avg` | 0.965| 0.965| 0.711| 0.965| 0.819| 81441| 110544| 0.737 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5077 |50343 |756 |0 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3992 |906 |22359 |0 |32 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6123 |0 |0 |859 |0 |170 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2696 |143 |170 |0 |3947 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2289 |0 |0 |0 |0 |1079 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2415 |64 |65 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2245 |82 |29 |0 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1627 |39 |11 |0 |171 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|648 |3 |116 |0 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|678 |25 |13 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|341 |0 |15 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|324 |2 |1 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|322 |1 |0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|244 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|82 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| hostel/templates/hostel/assets/js/plugins/markdown/markdown.js | 855 |
| hostel/templates/hostel/assets/js/plugins/magnific/jquery.magnific-popup.js | 397 |
| hostel/templates/hostel/assets/js/plugins/pnotify/pnotify.js | 341 |
| hostel/templates/hostel/assets/js/plugins/circles/circles.js | 118 |
| hostel/templates/hostel/assets/js/demo/charts/highcharts.js | 109 |
| hostel/templates/hostel/assets/js/demo/widgets.js | 102 |
| hostel/templates/hostel/assets/js/plugins/slick/slick.js | 98 |
| hostel/templates/hostel/doc/js/jquery.easing.js | 92 |
| hostel/templates/hostel/assets/js/pages/maps-basic.js | 85 |
| hostel/templates/hostel/assets/js/pages/tables-data.js | 75 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9269759450171821, "precision": 0.8638911128903123, "recall": 1.0, "support": 1079}, "\u0027": {"f1-score": 0.909957627118644, "precision": 1.0, "recall": 0.8347910592808552, "support": 1029}, "macro avg": {"f1-score": 0.31376142544767444, "precision": 0.3154523454878858, "recall": 0.31374034384004784, "support": 81441}, "micro avg": {"f1-score": 0.9649562259795434, "precision": 0.9649562259795434, "recall": 0.9649562259795434, "support": 81441}, "weighted avg": {"f1-score": 0.9609295397698556, "precision": 0.9572732080163655, "recall": 0.9649562259795434, "support": 81441}, "\u2205": {"f1-score": 0.9802463126125688, "precision": 0.9754882963881569, "recall": 0.9850509714911851, "support": 51107}, "\u23ce": {"f1-score": 0.9344223484848484, "precision": 0.9424546322827125, "recall": 0.9265258215962441, "support": 4260}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 135}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 221}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}, "\u2423": {"f1-score": 0.9548191484818722, "precision": 0.9499511407571058, "recall": 0.9597373052324334, "support": 23297}},
  "cl_report_full": {"\"": {"f1-score": 0.4674030755902101, "precision": 0.8638911128903123, "recall": 0.32036817102137766, "support": 3368}, "\u0027": {"f1-score": 0.2144551242042192, "precision": 1.0, "recall": 0.1201062639821029, "support": 7152}, "macro avg": {"f1-score": 0.2136082314166359, "precision": 0.3154523454878858, "recall": 0.18155182574608345, "support": 110544}, "micro avg": {"f1-score": 0.8186785425944735, "precision": 0.9649562259795434, "recall": 0.7109114922564771, "support": 110544}, "weighted avg": {"f1-score": 0.7646284841940502, "precision": 0.8806208461315493, "recall": 0.7109114922564771, "support": 110544}, "\u2205": {"f1-score": 0.9340767403888972, "precision": 0.9754882963881569, "recall": 0.8960380179410509, "support": 56184}, "\u23ce": {"f1-score": 0.7083632447954058, "precision": 0.9424546322827125, "recall": 0.567423806785509, "support": 6956}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 783}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 717}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1848}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 325}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 356}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2547}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 328}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2362}, "\u2423": {"f1-score": 0.8798252862708063, "precision": 0.9499511407571058, "recall": 0.8193411264612115, "support": 27289}},
  "ppcr": 0.7367292661745549
}
```
</details>
