# Test report for javascript / file:///tmp/top-repos-quality-repos-obs4oh_z/mattermost-sourcecode-edited.git HEAD 84908b440cb8c4a9fcb40968fdcb1bab8d2a47ad

### Classification report

PPCR: 0.947

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.991| 0.970| 0.988| 0.978| 25702| 26250| 0.979 |
| `␣` | 0.970| 0.980| 0.955| 0.975| 0.962| 9833| 10095| 0.974 |
| `'` | 0.996| 1.000| 0.999| 0.998| 0.997| 5335| 5338| 0.999 |
| `⏎` | 0.922| 0.914| 0.622| 0.918| 0.743| 1963| 2888| 0.680 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.990| 0.920| 0.843| 0.953| 0.911| 1371| 1495| 0.917 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.929| 0.839| 0.748| 0.882| 0.829| 1349| 1513| 0.892 |
| `⏎⏎` | 0.928| 0.908| 0.581| 0.918| 0.715| 987| 1542| 0.640 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 25| 0.400 |
| `micro avg` | 0.978| 0.978| 0.926| 0.978| 0.952| 46550| 49146| 0.947 |
| `weighted avg` | 0.978| 0.978| 0.926| 0.978| 0.948| 46550| 49146| 0.947 |
| `macro avg` | 0.840| 0.819| 0.715| 0.829| 0.767| 46550| 49146| 0.947 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|548 |25470 |145 |0 |2 |1 |81 |3 |0 |
|262 |103 |9640 |21 |59 |10 |0 |0 |0 |
|3 |0 |0 |5335 |0 |0 |0 |0 |0 |
|925 |67 |37 |0 |1795 |59 |5 |0 |0 |
|555 |5 |13 |0 |73 |896 |0 |0 |0 |
|164 |133 |80 |3 |1 |0 |1132 |0 |0 |
|124 |67 |27 |0 |16 |0 |0 |1261 |0 |
|15 |0 |0 |0 |0 |0 |0 |10 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9977557508883486, "precision": 0.9955215525284568, "recall": 1.0, "support": 5335}, "macro avg": {"f1-score": 0.8290456506429607, "precision": 0.8399706655543574, "recall": 0.819058813271689, "support": 46550}, "micro avg": {"f1-score": 0.9780665950590762, "precision": 0.9780665950590762, "recall": 0.9780665950590762, "support": 46550}, "weighted avg": {"f1-score": 0.9777591209484384, "precision": 0.9776887887282462, "recall": 0.9780665950590762, "support": 46550}, "\u2205": {"f1-score": 0.9882243389528004, "precision": 0.9854904236796286, "recall": 0.9909734650999922, "support": 25702}, "\u23ce": {"f1-score": 0.9183934510104887, "precision": 0.9224049331963001, "recall": 0.9144167091186959, "support": 1963}, "\u23ce\u23ce": {"f1-score": 0.9175627240143368, "precision": 0.927536231884058, "recall": 0.9078014184397163, "support": 987}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8819633813790416, "precision": 0.9293924466338259, "recall": 0.8391401037805782, "support": 1349}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9534971644612477, "precision": 0.9897959183673469, "recall": 0.9197665937272064, "support": 1371}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.9749683944374209, "precision": 0.9696238181452425, "recall": 0.9803722160073223, "support": 9833}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9974759278302329, "precision": 0.9955215525284568, "recall": 0.9994379917572125, "support": 5338}, "macro avg": {"f1-score": 0.7668116271929868, "precision": 0.8399706655543574, "recall": 0.7148641897705068, "support": 49146}, "micro avg": {"f1-score": 0.9515340244106337, "precision": 0.9780665950590762, "recall": 0.926402962601229, "support": 49146}, "weighted avg": {"f1-score": 0.9475570423711207, "precision": 0.9756979632018002, "recall": 0.926402962601229, "support": 49146}, "\u2205": {"f1-score": 0.9778289663115461, "precision": 0.9854904236796286, "recall": 0.9702857142857143, "support": 26250}, "\u23ce": {"f1-score": 0.7426561853537443, "precision": 0.9224049331963001, "recall": 0.6215373961218836, "support": 2888}, "\u23ce\u23ce": {"f1-score": 0.7145135566188198, "precision": 0.927536231884058, "recall": 0.5810635538261998, "support": 1542}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8290003661662394, "precision": 0.9293924466338259, "recall": 0.7481824190350297, "support": 1513}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9107981220657276, "precision": 0.9897959183673469, "recall": 0.8434782608695652, "support": 1495}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u2423": {"f1-score": 0.9622198931975845, "precision": 0.9696238181452425, "recall": 0.9549281822684498, "support": 10095}},
  "ppcr": 0.9471777967688113
}
```
</details>