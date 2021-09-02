# Test report for javascript / file:///tmp/top-repos-quality-repos-lqd7xs4d/sonos.git HEAD 787364aa6e1e9bdcc7828f71a3ffca5da680c3e7

### Classification report

PPCR: 0.757

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.924| 0.981| 0.869| 0.952| 0.896| 1819| 2053| 0.886 |
| `'` | 0.973| 0.978| 0.973| 0.976| 0.973| 553| 556| 0.995 |
| `␣` | 0.887| 0.840| 0.405| 0.863| 0.556| 469| 973| 0.482 |
| `⏎␣⁺␣⁺` | 0.893| 0.745| 0.727| 0.812| 0.801| 157| 161| 0.975 |
| `⏎␣⁻␣⁻` | 1.000| 0.757| 0.682| 0.862| 0.811| 136| 151| 0.901 |
| `⏎` | 0.972| 0.929| 0.342| 0.950| 0.506| 113| 307| 0.368 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 122| 0.189 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 2| 1.000 |
| `micro avg` | 0.930| 0.930| 0.704| 0.930| 0.801| 3272| 4325| 0.757 |
| `weighted avg` | 0.924| 0.930| 0.704| 0.925| 0.769| 3272| 4325| 0.757 |
| `macro avg` | 0.706| 0.654| 0.500| 0.677| 0.568| 3272| 4325| 0.757 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|234 |1784 |27 |0 |0 |8 |0 |0 |0 |
|504 |68 |394 |1 |0 |6 |0 |0 |0 |
|3 |12 |0 |541 |0 |0 |0 |0 |0 |
|194 |7 |1 |0 |105 |0 |0 |0 |0 |
|4 |26 |2 |12 |0 |117 |0 |0 |0 |
|15 |28 |4 |0 |1 |0 |103 |0 |0 |
|99 |5 |16 |0 |2 |0 |0 |0 |0 |
|0 |0 |0 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u0027": {"f1-score": 0.975653742110009, "precision": 0.9730215827338129, "recall": 0.9783001808318263, "support": 553}, "macro avg": {"f1-score": 0.6768892312432231, "precision": 0.7062641618677508, "recall": 0.6538654422772152, "support": 3272}, "micro avg": {"f1-score": 0.9303178484107579, "precision": 0.9303178484107579, "recall": 0.9303178484107579, "support": 3272}, "weighted avg": {"f1-score": 0.9253253257452505, "precision": 0.9235161969086508, "recall": 0.9303178484107579, "support": 3272}, "\u2205": {"f1-score": 0.9517204587890105, "precision": 0.9243523316062177, "recall": 0.9807586586036283, "support": 1819}, "\u23ce": {"f1-score": 0.9502262443438914, "precision": 0.9722222222222222, "recall": 0.9292035398230089, "support": 113}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8124999999999999, "precision": 0.8931297709923665, "recall": 0.7452229299363057, "support": 157}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8619246861924686, "precision": 1.0, "recall": 0.7573529411764706, "support": 136}, "\u2423": {"f1-score": 0.8630887185104053, "precision": 0.8873873873873874, "recall": 0.8400852878464818, "support": 469}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u0027": {"f1-score": 0.9730215827338129, "precision": 0.9730215827338129, "recall": 0.9730215827338129, "support": 556}, "macro avg": {"f1-score": 0.567918848838775, "precision": 0.7062641618677508, "recall": 0.4997217298240736, "support": 4325}, "micro avg": {"f1-score": 0.8013689614321442, "precision": 0.9303178484107579, "recall": 0.7038150289017341, "support": 4325}, "weighted avg": {"f1-score": 0.7694835329565013, "precision": 0.9006680647554881, "recall": 0.7038150289017341, "support": 4325}, "\u2205": {"f1-score": 0.8958071805171981, "precision": 0.9243523316062177, "recall": 0.8689722357525572, "support": 2053}, "\u23ce": {"f1-score": 0.5060240963855421, "precision": 0.9722222222222222, "recall": 0.34201954397394135, "support": 307}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8013698630136987, "precision": 0.8931297709923665, "recall": 0.7267080745341615, "support": 161}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8110236220472441, "precision": 1.0, "recall": 0.6821192052980133, "support": 151}, "\u2423": {"f1-score": 0.5561044460127029, "precision": 0.8873873873873874, "recall": 0.40493319630010277, "support": 973}},
  "ppcr": 0.7565317919075144
}
```
</details>