# Train report for javascript / file:///tmp/top-repos-quality-repos-_kcnq0hf/web-positiviser.git HEAD 8c2d5cd0a211375b224585f08540e4b8a27916b3

### Classification report

PPCR: 0.409

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.962| 1.000| 0.625| 0.981| 0.758| 1376| 2200| 0.625 |
| `␣` | 1.000| 0.772| 0.153| 0.871| 0.266| 215| 1084| 0.198 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 234| 0.017 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 134| 0.007 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 135| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 119| 0.000 |
| `micro avg` | 0.966| 0.966| 0.395| 0.966| 0.561| 1596| 3906| 0.409 |
| `macro avg` | 0.327| 0.295| 0.130| 0.309| 0.171| 1596| 3906| 0.409 |
| `weighted avg` | 0.964| 0.966| 0.395| 0.963| 0.501| 1596| 3906| 0.409 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|824 |1376 |0 |0 |0 |0 |0 |
|869 |49 |166 |0 |0 |0 |0 |
|230 |4 |0 |0 |0 |0 |0 |
|135 |0 |0 |0 |0 |0 |0 |
|133 |1 |0 |0 |0 |0 |0 |
|119 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| extension/src/content/restyling.js | 15 |
| extension/src/content/backends.js | 11 |
| extension/src/popup/popup.js | 10 |
| extension/src/content/scoring.js | 8 |
| extension/src/content/main.js | 6 |
| extension/src/content/common.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3086910999988152, "precision": 0.327039627039627, "recall": 0.29534883720930233, "support": 1596}, "micro avg": {"f1-score": 0.9661654135338346, "precision": 0.9661654135338346, "recall": 0.9661654135338346, "support": 1596}, "weighted avg": {"f1-score": 0.9629503021429444, "precision": 0.9643102511523564, "recall": 0.9661654135338346, "support": 1596}, "\u2205": {"f1-score": 0.9807555238774056, "precision": 0.9622377622377623, "recall": 1.0, "support": 1376}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8713910761154855, "precision": 1.0, "recall": 0.772093023255814, "support": 215}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 135}, "macro avg": {"f1-score": 0.17062112029384757, "precision": 0.327039627039627, "recall": 0.12976517946997654, "support": 3906}, "micro avg": {"f1-score": 0.5605234460196292, "precision": 0.9661654135338346, "recall": 0.39477726574500765, "support": 3906}, "weighted avg": {"f1-score": 0.5007140777979487, "precision": 0.819488754972626, "recall": 0.39477726574500765, "support": 3906}, "\u2205": {"f1-score": 0.7581267217630855, "precision": 0.9622377622377623, "recall": 0.6254545454545455, "support": 2200}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 234}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 134}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 119}, "\u2423": {"f1-score": 0.2656, "precision": 1.0, "recall": 0.15313653136531366, "support": 1084}},
  "ppcr": 0.40860215053763443
}
```
</details>
