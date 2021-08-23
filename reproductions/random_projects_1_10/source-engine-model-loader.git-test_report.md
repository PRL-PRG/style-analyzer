# Test report for javascript / file:///tmp/top-repos-quality-repos-v4hlockb/source-engine-model-loader.git HEAD e90f2a62fcae645742df40dae29fd65fe7529811

### Classification report

PPCR: 0.863

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.928| 0.981| 0.929| 0.954| 0.929| 953| 1006| 0.947 |
| `∅` | 0.959| 0.938| 0.895| 0.949| 0.926| 757| 793| 0.955 |
| `⏎⏎⇥⁻` | 1.000| 0.806| 0.781| 0.893| 0.877| 31| 32| 0.969 |
| `⏎⏎⇥⁺` | 0.903| 1.000| 1.000| 0.949| 0.949| 28| 28| 1.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 52| 0.500 |
| `⏎⏎` | 0.923| 0.857| 0.214| 0.889| 0.348| 14| 56| 0.250 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 139| 0.058 |
| `micro avg` | 0.941| 0.941| 0.812| 0.941| 0.872| 1817| 2106| 0.863 |
| `weighted avg` | 0.924| 0.941| 0.812| 0.932| 0.828| 1817| 2106| 0.863 |
| `macro avg` | 0.673| 0.655| 0.546| 0.662| 0.576| 1817| 2106| 0.863 |

### Confusion matrix

|refusal|  ␣| ∅| ⏎| ⏎⏎| '| ⏎⏎⇥⁺| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|53 |935 |15 |0 |0 |0 |3 |0 |
|36 |47 |710 |0 |0 |0 |0 |0 |
|131 |1 |6 |0 |1 |0 |0 |0 |
|42 |2 |0 |0 |12 |0 |0 |0 |
|26 |17 |9 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |28 |0 |
|1 |6 |0 |0 |0 |0 |0 |25 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "macro avg": {"f1-score": 0.6618653532732272, "precision": 0.6733345077239087, "recall": 0.6546599372577802, "support": 1817}, "micro avg": {"f1-score": 0.9411117226197028, "precision": 0.9411117226197028, "recall": 0.9411117226197028, "support": 1817}, "weighted avg": {"f1-score": 0.9320518513680884, "precision": 0.9243298542846273, "recall": 0.9411117226197028, "support": 1817}, "\u2205": {"f1-score": 0.9485637942551771, "precision": 0.9594594594594594, "recall": 0.9379128137384413, "support": 757}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.888888888888889, "precision": 0.9230769230769231, "recall": 0.8571428571428571, "support": 14}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.9491525423728813, "precision": 0.9032258064516129, "recall": 1.0, "support": 28}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.8928571428571428, "precision": 1.0, "recall": 0.8064516129032258, "support": 31}, "\u2423": {"f1-score": 0.9535951045385007, "precision": 0.9275793650793651, "recall": 0.9811122770199371, "support": 953}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 52}, "macro avg": {"f1-score": 0.5755657759797815, "precision": 0.6733345077239087, "recall": 0.5457561925075637, "support": 2106}, "micro avg": {"f1-score": 0.8717817996431303, "precision": 0.9411117226197028, "recall": 0.811965811965812, "support": 2106}, "weighted avg": {"f1-score": 0.827513222968716, "precision": 0.8561153005195372, "recall": 0.811965811965812, "support": 2106}, "\u2205": {"f1-score": 0.9262883235485975, "precision": 0.9594594594594594, "recall": 0.8953341740226987, "support": 793}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 139}, "\u23ce\u23ce": {"f1-score": 0.34782608695652173, "precision": 0.9230769230769231, "recall": 0.21428571428571427, "support": 56}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.9491525423728813, "precision": 0.9032258064516129, "recall": 1.0, "support": 28}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.8771929824561403, "precision": 1.0, "recall": 0.78125, "support": 32}, "\u2423": {"f1-score": 0.9285004965243296, "precision": 0.9275793650793651, "recall": 0.9294234592445328, "support": 1006}},
  "ppcr": 0.8627730294396961
}
```
</details>
