# Train report for javascript / file:///tmp/top-repos-quality-repos-cu7e4e1x/diy-biller.git HEAD 7257ab7c67bd8e36e2da9a99f66c98dfaad6ab37

### Classification report

PPCR: 0.702

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.917| 0.997| 0.858| 0.955| 0.886| 5034| 5854| 0.860 |
| `␣` | 0.965| 0.527| 0.181| 0.682| 0.305| 728| 2119| 0.344 |
| `"` | 1.000| 1.000| 0.980| 1.000| 0.990| 674| 688| 0.980 |
| `'` | 1.000| 1.000| 0.472| 1.000| 0.642| 222| 470| 0.472 |
| `⏎` | 0.982| 0.727| 0.294| 0.836| 0.453| 154| 381| 0.404 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 157| 0.248 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 142| 0.225 |
| `weighted avg` | 0.925| 0.932| 0.654| 0.920| 0.712| 6883| 9811| 0.702 |
| `micro avg` | 0.932| 0.932| 0.654| 0.932| 0.768| 6883| 9811| 0.702 |
| `macro avg` | 0.695| 0.607| 0.398| 0.639| 0.468| 6883| 9811| 0.702 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|820 |5020 |14 |0 |0 |0 |0 |0 |
|1391 |344 |384 |0 |0 |0 |0 |0 |
|14 |0 |0 |674 |0 |0 |0 |0 |
|248 |0 |0 |0 |222 |0 |0 |0 |
|227 |42 |0 |0 |0 |112 |0 |0 |
|118 |39 |0 |0 |0 |0 |0 |0 |
|110 |30 |0 |0 |0 |2 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/components/Home/Home.js | 160 |
| backend/routes/users.js | 62 |
| backend/routes/employee.js | 55 |
| frontend/src/serviceWorker.js | 45 |
| backend/routes/inventory.js | 27 |
| backend/middleware/hash.js | 25 |
| backend/routes/sale.js | 21 |
| frontend/src/components/Biller/Biller.js | 13 |
| frontend/src/components/Inventory/inventory.js | 12 |
| backend/middleware/sql_connector.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 674}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 222}, "macro avg": {"f1-score": 0.6390361246493643, "precision": 0.6948821768746917, "recall": 0.6074234523068168, "support": 6883}, "micro avg": {"f1-score": 0.9315705361034433, "precision": 0.9315705361034433, "recall": 0.9315705361034433, "support": 6883}, "weighted avg": {"f1-score": 0.9197438540776218, "precision": 0.9247914456605438, "recall": 0.9315705361034433, "support": 6883}, "\u2205": {"f1-score": 0.9553715862593967, "precision": 0.9168949771689497, "recall": 0.9972189114024632, "support": 5034}, "\u23ce": {"f1-score": 0.835820895522388, "precision": 0.9824561403508771, "recall": 0.7272727272727273, "support": 154}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.6820603907637655, "precision": 0.964824120603015, "recall": 0.5274725274725275, "support": 728}},
  "cl_report_full": {"\"": {"f1-score": 0.9897209985315712, "precision": 1.0, "recall": 0.9796511627906976, "support": 688}, "\u0027": {"f1-score": 0.6416184971098265, "precision": 1.0, "recall": 0.4723404255319149, "support": 470}, "macro avg": {"f1-score": 0.4678872999111313, "precision": 0.6948821768746917, "recall": 0.3978151012747653, "support": 9811}, "micro avg": {"f1-score": 0.7681801844974241, "precision": 0.9315705361034433, "recall": 0.6535521353582713, "support": 9811}, "weighted avg": {"f1-score": 0.7124044409711752, "precision": 0.9116584749137197, "recall": 0.6535521353582713, "support": 9811}, "\u2205": {"f1-score": 0.88622120222438, "precision": 0.9168949771689497, "recall": 0.8575333105568842, "support": 5854}, "\u23ce": {"f1-score": 0.4525252525252525, "precision": 0.9824561403508771, "recall": 0.29396325459317585, "support": 381}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u2423": {"f1-score": 0.30512514898688914, "precision": 0.964824120603015, "recall": 0.1812175554506843, "support": 2119}},
  "ppcr": 0.7015594740597288
}
```
</details>
