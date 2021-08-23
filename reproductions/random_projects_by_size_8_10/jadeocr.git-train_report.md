# Train report for javascript / file:///tmp/top-repos-quality-repos-ii0_4zya/jadeocr.git HEAD 98f16bcc5b44375aff7c7ca6d1888f5590628076

### Classification report

PPCR: 0.313

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.952| 1.000| 0.546| 0.975| 0.694| 613| 1123| 0.546 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 76| 0.197 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 153| 0.092 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 509| 0.004 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 198| 0.000 |
| `macro avg` | 0.190| 0.200| 0.109| 0.195| 0.139| 644| 2059| 0.313 |
| `micro avg` | 0.952| 0.952| 0.298| 0.952| 0.454| 644| 2059| 0.313 |
| `weighted avg` | 0.906| 0.952| 0.298| 0.928| 0.378| 644| 2059| 0.313 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|510 |613 |0 |0 |0 |0 |
|507 |2 |0 |0 |0 |0 |
|198 |0 |0 |0 |0 |0 |
|139 |14 |0 |0 |0 |0 |
|61 |15 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| chinese-ocr-webapp/src/store/index.js | 30 |
| chinese-ocr-webapp/functions/index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19506762132060462, "precision": 0.1903726708074534, "recall": 0.2, "support": 644}, "micro avg": {"f1-score": 0.9518633540372671, "precision": 0.9518633540372671, "recall": 0.9518633540372671, "support": 644}, "weighted avg": {"f1-score": 0.9283886014715111, "precision": 0.9060438447590756, "recall": 0.9518633540372671, "support": 644}, "\u2205": {"f1-score": 0.9753381066030231, "precision": 0.9518633540372671, "recall": 1.0, "support": 613}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 198}, "macro avg": {"f1-score": 0.13876627051499718, "precision": 0.1903726708074534, "recall": 0.10917186108637578, "support": 2059}, "micro avg": {"f1-score": 0.4535701072881983, "precision": 0.9518633540372671, "recall": 0.2977173385138417, "support": 2059}, "weighted avg": {"f1-score": 0.3784228309576052, "precision": 0.5191561663836091, "recall": 0.2977173385138417, "support": 2059}, "\u2205": {"f1-score": 0.6938313525749858, "precision": 0.9518633540372671, "recall": 0.5458593054318789, "support": 1123}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 153}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 509}},
  "ppcr": 0.31277319086935407
}
```
</details>
