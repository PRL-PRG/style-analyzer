# Train report for javascript / file:///tmp/top-repos-quality-repos-_1nu3kix/fantasticfour.git HEAD 90ccc5247582b0a7a145ffacddf0c289847e2268

### Classification report

PPCR: 0.918

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.927| 1.000| 0.998| 0.962| 0.961| 7918| 7933| 0.998 |
| `⏎` | 0.961| 0.697| 0.416| 0.808| 0.580| 872| 1462| 0.596 |
| `␣` | 1.000| 0.432| 0.303| 0.603| 0.465| 672| 956| 0.703 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 506| 506| 1.000 |
| `macro avg` | 0.972| 0.782| 0.679| 0.843| 0.752| 9968| 10857| 0.918 |
| `micro avg` | 0.935| 0.935| 0.858| 0.935| 0.895| 9968| 10857| 0.918 |
| `weighted avg` | 0.938| 0.935| 0.858| 0.926| 0.868| 9968| 10857| 0.918 |

### Confusion matrix

|refusal|  ∅| ⏎| ␣| "| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|15 |7915 |3 |0 |0 |
|590 |264 |608 |0 |0 |
|284 |360 |22 |290 |0 |
|0 |0 |0 |0 |506 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| .metadata/.plugins/org.eclipse.wst.jsdt.core/libraries/baseBrowserLibrary.js | 432 |
| .metadata/.plugins/org.eclipse.wst.jsdt.core/libraries/browserWindow.js | 177 |
| .metadata/.plugins/org.eclipse.wst.jsdt.core/libraries/dom5.js | 33 |
| .metadata/.plugins/org.eclipse.wst.jsdt.core/libraries/xhr.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 506}, "macro avg": {"f1-score": 0.8431961839452962, "precision": 0.9718572641427576, "recall": 0.782104110478296, "support": 9968}, "micro avg": {"f1-score": 0.9348916532905297, "precision": 0.9348916532905297, "recall": 0.9348916532905297, "support": 9968}, "weighted avg": {"f1-score": 0.9261675941340469, "precision": 0.938497322532471, "recall": 0.9348916532905297, "support": 9968}, "\u2205": {"f1-score": 0.9619007109436714, "precision": 0.9269235273451224, "recall": 0.9996211164435463, "support": 7918}, "\u23ce": {"f1-score": 0.8079734219269104, "precision": 0.9605055292259084, "recall": 0.6972477064220184, "support": 872}, "\u2423": {"f1-score": 0.6029106029106029, "precision": 1.0, "recall": 0.43154761904761907, "support": 672}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 506}, "macro avg": {"f1-score": 0.7517359825476817, "precision": 0.9718572641427576, "recall": 0.6792367376215155, "support": 10857}, "micro avg": {"f1-score": 0.8949819927971188, "precision": 0.9348916532905297, "recall": 0.8583402413189647, "support": 10857}, "weighted avg": {"f1-score": 0.8679566719543518, "precision": 0.9412861219634461, "recall": 0.8583402413189647, "support": 10857}, "\u2205": {"f1-score": 0.9610247693054882, "precision": 0.9269235273451224, "recall": 0.9977309971007186, "support": 7933}, "\u23ce": {"f1-score": 0.5804295942720764, "precision": 0.9605055292259084, "recall": 0.4158686730506156, "support": 1462}, "\u2423": {"f1-score": 0.46548956661316215, "precision": 1.0, "recall": 0.303347280334728, "support": 956}},
  "ppcr": 0.9181173436492586
}
```
</details>
