# Train report for javascript / file:///tmp/top-repos-quality-repos-kvmignaw/pikbooth.git HEAD 1136cc8a9cc9b3dac6f509f3bd00b514bc6db3a2

### Classification report

PPCR: 0.879

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 0.977| 0.958| 0.987| 0.977| 19786| 20172| 0.981 |
| `␣` | 0.952| 0.991| 0.933| 0.971| 0.942| 9770| 10375| 0.942 |
| `⏎␣⁺␣⁺` | 0.972| 0.974| 0.966| 0.973| 0.969| 1134| 1143| 0.992 |
| `⏎␣⁻␣⁻` | 0.983| 0.981| 0.980| 0.982| 0.982| 1111| 1112| 0.999 |
| `'` | 1.000| 1.000| 0.419| 1.000| 0.590| 729| 1741| 0.419 |
| `⏎` | 0.948| 0.955| 0.291| 0.951| 0.445| 533| 1752| 0.304 |
| `⏎⏎` | 0.963| 0.929| 0.118| 0.945| 0.210| 140| 1102| 0.127 |
| `"` | 1.000| 1.000| 0.080| 1.000| 0.148| 34| 425| 0.080 |
| `macro avg` | 0.977| 0.976| 0.593| 0.976| 0.658| 33237| 37822| 0.879 |
| `micro avg` | 0.981| 0.981| 0.862| 0.981| 0.918| 33237| 37822| 0.879 |
| `weighted avg` | 0.982| 0.981| 0.862| 0.981| 0.893| 33237| 37822| 0.879 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|386 |19333 |434 |7 |0 |6 |6 |0 |0 |
|605 |42 |9678 |12 |0 |26 |12 |0 |0 |
|1219 |4 |14 |509 |0 |0 |1 |5 |0 |
|1012 |0 |0 |0 |729 |0 |0 |0 |0 |
|9 |0 |30 |0 |0 |1104 |0 |0 |0 |
|1 |15 |6 |0 |0 |0 |1090 |0 |0 |
|962 |0 |1 |9 |0 |0 |0 |130 |0 |
|391 |0 |0 |0 |0 |0 |0 |0 |34 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/assets/js/bootstrap.bundle.js | 352 |
| frontend/server.js | 161 |
| frontend/assets/js/booth.js | 46 |
| frontend/assets/js/cmd.js | 35 |
| frontend/assets/js/client.js | 33 |
| frontend/assets/js/cmdset.js | 2 |
| frontend/assets/js/main.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 34}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 729}, "macro avg": {"f1-score": 0.9761824638342275, "precision": 0.9768315547580776, "recall": 0.9757343514651111, "support": 33237}, "micro avg": {"f1-score": 0.9810452206877877, "precision": 0.9810452206877877, "recall": 0.9810452206877877, "support": 33237}, "weighted avg": {"f1-score": 0.9811380959640473, "precision": 0.9815737669468441, "recall": 0.9810452206877877, "support": 33237}, "\u2205": {"f1-score": 0.9868810617662073, "precision": 0.9968546973290708, "recall": 0.9771050237541696, "support": 19786}, "\u23ce": {"f1-score": 0.9514018691588786, "precision": 0.9478584729981379, "recall": 0.9549718574108818, "support": 533}, "\u23ce\u23ce": {"f1-score": 0.9454545454545454, "precision": 0.9629629629629629, "recall": 0.9285714285714286, "support": 140}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9726872246696034, "precision": 0.971830985915493, "recall": 0.9735449735449735, "support": 1134}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9819819819819819, "precision": 0.9828674481514879, "recall": 0.9810981098109811, "support": 1111}, "\u2423": {"f1-score": 0.9710530276426028, "precision": 0.9522778707074683, "recall": 0.9905834186284544, "support": 9770}},
  "cl_report_full": {"\"": {"f1-score": 0.14814814814814814, "precision": 1.0, "recall": 0.08, "support": 425}, "\u0027": {"f1-score": 0.5902834008097165, "precision": 1.0, "recall": 0.4187248707639288, "support": 1741}, "macro avg": {"f1-score": 0.6579300436498985, "precision": 0.9768315547580776, "recall": 0.5930674225517568, "support": 37822}, "micro avg": {"f1-score": 0.9177444095751417, "precision": 0.9810452206877877, "recall": 0.8621172862355243, "support": 37822}, "weighted avg": {"f1-score": 0.893431401397727, "precision": 0.9803823307132301, "recall": 0.8621172862355243, "support": 37822}, "\u2205": {"f1-score": 0.9772531971895061, "precision": 0.9968546973290708, "recall": 0.9584076938330359, "support": 20172}, "\u23ce": {"f1-score": 0.44473569244211447, "precision": 0.9478584729981379, "recall": 0.2905251141552511, "support": 1752}, "\u23ce\u23ce": {"f1-score": 0.21018593371059016, "precision": 0.9629629629629629, "recall": 0.11796733212341198, "support": 1102}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9688459850811759, "precision": 0.971830985915493, "recall": 0.9658792650918635, "support": 1143}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9815398469158036, "precision": 0.9828674481514879, "recall": 0.9802158273381295, "support": 1112}, "\u2423": {"f1-score": 0.9424481449021326, "precision": 0.9522778707074683, "recall": 0.9328192771084337, "support": 10375}},
  "ppcr": 0.8787742583681455
}
```
</details>
