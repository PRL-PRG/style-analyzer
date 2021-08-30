# Test report for javascript / file:///tmp/top-repos-quality-repos-khao_4x6/hostel_management.git HEAD d7707a5cf93972867bcb77ff643ae10d27859230

### Classification report

PPCR: 0.935

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.990| 0.976| 0.988| 0.981| 7201| 7302| 0.986 |
| `␣` | 0.929| 0.996| 0.968| 0.961| 0.948| 3745| 3856| 0.971 |
| `'` | 0.853| 0.886| 0.799| 0.869| 0.825| 1044| 1158| 0.902 |
| `⏎` | 0.973| 0.825| 0.682| 0.893| 0.802| 957| 1158| 0.826 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.958| 0.900| 0.794| 0.928| 0.868| 479| 543| 0.882 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.997| 0.846| 0.530| 0.915| 0.692| 357| 570| 0.626 |
| `"` | 0.308| 0.183| 0.157| 0.229| 0.207| 197| 230| 0.857 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 121| 0.174 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 30| 0.467 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 29| 0.034 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.400| 0.375| 0.327| 0.386| 0.355| 14016| 14997| 0.935 |
| `weighted avg` | 0.947| 0.952| 0.890| 0.948| 0.908| 14016| 14997| 0.935 |
| `micro avg` | 0.952| 0.952| 0.890| 0.952| 0.920| 14016| 14997| 0.935 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|101 |7130 |71 |0 |0 |0 |0 |0 |0 |0 |0 |
|111 |8 |3731 |0 |2 |0 |0 |4 |0 |0 |0 |
|114 |8 |30 |925 |0 |81 |0 |0 |0 |0 |0 |
|201 |19 |148 |0 |790 |0 |0 |0 |0 |0 |0 |
|33 |8 |1 |151 |0 |36 |0 |1 |0 |0 |0 |
|213 |24 |22 |9 |0 |0 |302 |0 |0 |0 |0 |
|64 |31 |14 |0 |3 |0 |0 |431 |0 |0 |0 |
|100 |4 |0 |0 |17 |0 |0 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |0 |0 |14 |0 |0 |0 |
|28 |0 |0 |0 |0 |0 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.22929936305732485, "precision": 0.3076923076923077, "recall": 0.18274111675126903, "support": 197}, "\u0027": {"f1-score": 0.868952559887271, "precision": 0.8525345622119815, "recall": 0.8860153256704981, "support": 1044}, "macro avg": {"f1-score": 0.38558710688104775, "precision": 0.4001539552190553, "recall": 0.37509228885219037, "support": 14016}, "micro avg": {"f1-score": 0.9521261415525114, "precision": 0.9521261415525114, "recall": 0.9521261415525114, "support": 14016}, "weighted avg": {"f1-score": 0.9484316534625151, "precision": 0.9470700023625931, "recall": 0.9521261415525114, "support": 14016}, "\u2205": {"f1-score": 0.9880135799902999, "precision": 0.9858960176991151, "recall": 0.9901402582974587, "support": 7201}, "\u23ce": {"f1-score": 0.893159977388355, "precision": 0.9729064039408867, "recall": 0.8254963427377221, "support": 957}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9151515151515152, "precision": 0.9966996699669967, "recall": 0.84593837535014, "support": 357}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9278794402583423, "precision": 0.9577777777777777, "recall": 0.8997912317327766, "support": 479}, "\u2423": {"f1-score": 0.9613501674826075, "precision": 0.9288025889967637, "recall": 0.9962616822429906, "support": 3745}},
  "cl_report_full": {"\"": {"f1-score": 0.20749279538904902, "precision": 0.3076923076923077, "recall": 0.1565217391304348, "support": 230}, "\u0027": {"f1-score": 0.8247882300490413, "precision": 0.8525345622119815, "recall": 0.7987910189982729, "support": 1158}, "macro avg": {"f1-score": 0.35487993749290514, "precision": 0.4001539552190553, "recall": 0.32700762098092057, "support": 14997}, "micro avg": {"f1-score": 0.9199324440767931, "precision": 0.9521261415525114, "recall": 0.8898446355937855, "support": 14997}, "weighted avg": {"f1-score": 0.9079383724041117, "precision": 0.9370739160631614, "recall": 0.8898446355937855, "support": 14997}, "\u2205": {"f1-score": 0.9811476537773497, "precision": 0.9858960176991151, "recall": 0.9764448096411942, "support": 7302}, "\u23ce": {"f1-score": 0.8020304568527918, "precision": 0.9729064039408867, "recall": 0.6822107081174439, "support": 1158}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.6918671248568155, "precision": 0.9966996699669967, "recall": 0.5298245614035088, "support": 570}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8680765357502517, "precision": 0.9577777777777777, "recall": 0.7937384898710865, "support": 543}, "\u2423": {"f1-score": 0.9477962657182776, "precision": 0.9288025889967637, "recall": 0.9675829875518672, "support": 3856}},
  "ppcr": 0.9345869173834767
}
```
</details>