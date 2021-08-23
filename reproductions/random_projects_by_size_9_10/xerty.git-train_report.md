# Train report for javascript / file:///tmp/top-repos-quality-repos-s6iy5kje/xerty.git HEAD e6996544af10ffc291c08b54df13b43fc299d6b3

### Classification report

PPCR: 0.070

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.996| 1.000| 0.307| 0.998| 0.469| 222| 724| 0.307 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 2124| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 151| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 104| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 104| 0.000 |
| `micro avg` | 0.996| 0.996| 0.069| 0.996| 0.129| 223| 3207| 0.070 |
| `weighted avg` | 0.991| 0.996| 0.069| 0.993| 0.106| 223| 3207| 0.070 |
| `macro avg` | 0.199| 0.200| 0.061| 0.200| 0.094| 223| 3207| 0.070 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|2123 |0 |1 |0 |0 |0 |
|502 |0 |222 |0 |0 |0 |
|151 |0 |0 |0 |0 |0 |
|104 |0 |0 |0 |0 |0 |
|104 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| backend/routes/createGuild.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19955056179775282, "precision": 0.19910313901345292, "recall": 0.2, "support": 223}, "micro avg": {"f1-score": 0.9955156950672646, "precision": 0.9955156950672646, "recall": 0.9955156950672646, "support": 223}, "weighted avg": {"f1-score": 0.9932785811457651, "precision": 0.9910514991252589, "recall": 0.9955156950672646, "support": 223}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9977528089887641, "precision": 0.9955156950672646, "recall": 1.0, "support": 222}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "macro avg": {"f1-score": 0.09376979936642028, "precision": 0.19910313901345292, "recall": 0.06132596685082873, "support": 3207}, "micro avg": {"f1-score": 0.12944606413994167, "precision": 0.9955156950672646, "recall": 0.06922357343311505, "support": 3207}, "weighted avg": {"f1-score": 0.10584554839614636, "precision": 0.22474379894876817, "recall": 0.06922357343311505, "support": 3207}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2124}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "\u2423": {"f1-score": 0.4688489968321014, "precision": 0.9955156950672646, "recall": 0.30662983425414364, "support": 724}},
  "ppcr": 0.06953539133146243
}
```
</details>
