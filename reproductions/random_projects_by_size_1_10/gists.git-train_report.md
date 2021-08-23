# Train report for javascript / file:///tmp/top-repos-quality-repos-0jf2kc9n/gists.git HEAD 8c22220605a8fa51965e56eebf815a92a637ec8a

### Classification report

PPCR: 0.685

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.983| 0.826| 0.986| 0.900| 2772| 3301| 0.840 |
| `␣` | 0.980| 0.970| 0.511| 0.975| 0.672| 1118| 2123| 0.527 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.960| 0.976| 0.972| 0.968| 0.966| 247| 248| 0.996 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.953| 0.992| 0.953| 0.972| 0.953| 244| 254| 0.961 |
| `⏎` | 0.904| 0.979| 0.440| 0.940| 0.592| 193| 430| 0.449 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 320| 0.000 |
| `weighted avg` | 0.980| 0.980| 0.671| 0.980| 0.769| 4574| 6676| 0.685 |
| `macro avg` | 0.798| 0.817| 0.617| 0.807| 0.680| 4574| 6676| 0.685 |
| `micro avg` | 0.980| 0.980| 0.671| 0.980| 0.797| 4574| 6676| 0.685 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|529 |2725 |19 |10 |0 |8 |10 |
|1005 |25 |1085 |5 |0 |3 |0 |
|237 |0 |3 |189 |0 |1 |0 |
|320 |0 |0 |0 |0 |0 |0 |
|10 |2 |0 |0 |0 |242 |0 |
|1 |1 |0 |5 |0 |0 |241 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| JavaScript Typed Ajax Constructor/old_version/Api.js | 46 |
| JavaScript Typed Ajax Constructor/api/Api.js | 21 |
| JavaScript Input Pooling/demo.js | 8 |
| JavaScript Input Pooling/InputPooler.js | 5 |
| JavaScript Typed Ajax Constructor/models/DataObject/DataObject.js | 5 |
| JavaScript Typed Ajax Constructor/api/apiDefinitions.js | 2 |
| JavaScript Typed Ajax Constructor/namespace.js | 2 |
| JavaScript Typed Ajax Constructor/models/Item.js | 1 |
| JavaScript Typed Ajax Constructor/models/User.js | 1 |
| JavaScript Typed Ajax Constructor/old_version/Item.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.8069606303083643, "precision": 0.7978628722068081, "recall": 0.8167190217538732, "support": 4574}, "micro avg": {"f1-score": 0.979886313948404, "precision": 0.979886313948404, "recall": 0.979886313948404, "support": 4574}, "weighted avg": {"f1-score": 0.9799775944217772, "precision": 0.9802691410988824, "recall": 0.979886313948404, "support": 4574}, "\u2205": {"f1-score": 0.9864253393665159, "precision": 0.9898292771521976, "recall": 0.983044733044733, "support": 2772}, "\u23ce": {"f1-score": 0.9402985074626866, "precision": 0.9043062200956937, "recall": 0.9792746113989638, "support": 193}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9718875502008032, "precision": 0.952755905511811, "recall": 0.9918032786885246, "support": 244}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.967871485943775, "precision": 0.9601593625498008, "recall": 0.9757085020242915, "support": 247}, "\u2423": {"f1-score": 0.9752808988764045, "precision": 0.980126467931346, "recall": 0.9704830053667263, "support": 1118}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 320}, "macro avg": {"f1-score": 0.6803824904109436, "precision": 0.7978628722068081, "recall": 0.616773607735609, "support": 6676}, "micro avg": {"f1-score": 0.7968000000000001, "precision": 0.979886313948404, "recall": 0.6713600958657879, "support": 6676}, "weighted avg": {"f1-score": 0.7690036830661839, "precision": 0.9312771317931623, "recall": 0.6713600958657879, "support": 6676}, "\u2205": {"f1-score": 0.9002312520647506, "precision": 0.9898292771521976, "recall": 0.8255074219933354, "support": 3301}, "\u23ce": {"f1-score": 0.5915492957746479, "precision": 0.9043062200956937, "recall": 0.43953488372093025, "support": 430}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.952755905511811, "precision": 0.952755905511811, "recall": 0.952755905511811, "support": 254}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.965931863727455, "precision": 0.9601593625498008, "recall": 0.9717741935483871, "support": 248}, "\u2423": {"f1-score": 0.6718266253869969, "precision": 0.980126467931346, "recall": 0.5110692416391899, "support": 2123}},
  "ppcr": 0.6851408028759737
}
```
</details>
