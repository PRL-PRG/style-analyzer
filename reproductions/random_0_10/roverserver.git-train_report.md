# Train report for javascript / file:///tmp/top-repos-quality-repos-iel6qosv/roverserver.git HEAD 114ee385425f451e45050159440dc8454f072567

### Classification report

PPCR: 0.267

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.999| 0.992| 0.407| 0.996| 0.579| 1299| 3164| 0.411 |
| `␣` | 0.946| 0.995| 0.114| 0.970| 0.204| 194| 1692| 0.115 |
| `⏎` | 1.000| 1.000| 0.426| 1.000| 0.597| 169| 397| 0.426 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 191| 0.005 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 433| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 157| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 189| 0.000 |
| `macro avg` | 0.421| 0.427| 0.135| 0.424| 0.197| 1663| 6223| 0.267 |
| `weighted avg` | 0.993| 0.993| 0.265| 0.993| 0.388| 1663| 6223| 0.267 |
| `micro avg` | 0.993| 0.993| 0.265| 0.993| 0.419| 1663| 6223| 0.267 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1865 |1289 |10 |0 |0 |0 |0 |0 |
|1498 |1 |193 |0 |0 |0 |0 |0 |
|228 |0 |0 |169 |0 |0 |0 |0 |
|433 |0 |0 |0 |0 |0 |0 |0 |
|157 |0 |0 |0 |0 |0 |0 |0 |
|190 |0 |1 |0 |0 |0 |0 |0 |
|189 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app.js | 4 |
| lib/arduino.js | 3 |
| web/app/app.component.js | 2 |
| web/js/gamepad.js | 2 |
| web/app/controller/controller.component.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4236572145060124, "precision": 0.4207576053677285, "recall": 0.4267353044882151, "support": 1663}, "micro avg": {"f1-score": 0.9927841250751653, "precision": 0.9927841250751653, "recall": 0.9927841250751653, "support": 1663}, "weighted avg": {"f1-score": 0.9925626184111507, "precision": 0.9925028496344489, "recall": 0.9927841250751653, "support": 1663}, "\u2205": {"f1-score": 0.9957512553109309, "precision": 0.9992248062015504, "recall": 0.9923017705927637, "support": 1299}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 169}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9698492462311558, "precision": 0.946078431372549, "recall": 0.9948453608247423, "support": 194}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 433}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "macro avg": {"f1-score": 0.19708074439933698, "precision": 0.4207576053677285, "recall": 0.13530779867300327, "support": 6223}, "micro avg": {"f1-score": 0.41871671316256664, "precision": 0.9927841250751653, "recall": 0.2653061224489796, "support": 6223}, "weighted avg": {"f1-score": 0.387736929164263, "precision": 0.8290715077461126, "recall": 0.2653061224489796, "support": 6223}, "\u2205": {"f1-score": 0.5788055680287383, "precision": 0.9992248062015504, "recall": 0.40739570164348926, "support": 3164}, "\u23ce": {"f1-score": 0.5971731448763251, "precision": 1.0, "recall": 0.4256926952141058, "support": 397}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 191}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 189}, "\u2423": {"f1-score": 0.20358649789029534, "precision": 0.946078431372549, "recall": 0.1140661938534279, "support": 1692}},
  "ppcr": 0.26723445283625263
}
```
</details>
