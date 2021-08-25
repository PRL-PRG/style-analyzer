# Test report for javascript / file:///tmp/top-repos-quality-repos-h854ovbm/lime.git HEAD 152bd3df2cf661bd6251e2a3533dbc410ee7d2fd

### Classification report

PPCR: 0.657

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.934| 1.000| 0.938| 0.966| 0.936| 408| 435| 0.938 |
| `␣` | 0.766| 0.874| 0.442| 0.816| 0.561| 127| 251| 0.506 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 50| 0.560 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 88| 0.159 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 27| 0.185 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 23| 0.000 |
| `macro avg` | 0.243| 0.268| 0.197| 0.255| 0.214| 582| 886| 0.657 |
| `micro avg` | 0.892| 0.892| 0.586| 0.892| 0.707| 582| 886| 0.657 |
| `weighted avg` | 0.822| 0.892| 0.586| 0.855| 0.618| 582| 886| 0.657 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|27 |408 |0 |0 |0 |0 |0 |0 |
|124 |16 |111 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |0 |0 |
|22 |0 |28 |0 |0 |0 |0 |0 |
|74 |13 |1 |0 |0 |0 |0 |0 |
|22 |0 |5 |0 |0 |0 |0 |0 |
|23 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "macro avg": {"f1-score": 0.25455099199443093, "precision": 0.2427365264736053, "recall": 0.2677165354330709, "support": 582}, "micro avg": {"f1-score": 0.8917525773195877, "precision": 0.8917525773195877, "recall": 0.8917525773195877, "support": 582}, "weighted avg": {"f1-score": 0.8550722420975955, "precision": 0.8215552831289183, "recall": 0.8917525773195877, "support": 582}, "\u2205": {"f1-score": 0.965680473372781, "precision": 0.9336384439359268, "recall": 1.0, "support": 408}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8161764705882353, "precision": 0.7655172413793103, "recall": 0.8740157480314961, "support": 127}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "macro avg": {"f1-score": 0.2137694110171174, "precision": 0.2427365264736053, "recall": 0.19716601573999568, "support": 886}, "micro avg": {"f1-score": 0.7070844686648501, "precision": 0.8917525773195877, "recall": 0.5857787810383747, "support": 886}, "weighted avg": {"f1-score": 0.6182577216654712, "precision": 0.6752568292306265, "recall": 0.5857787810383747, "support": 886}, "\u2205": {"f1-score": 0.9357798165137614, "precision": 0.9336384439359268, "recall": 0.9379310344827586, "support": 435}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.5606060606060606, "precision": 0.7655172413793103, "recall": 0.44223107569721115, "support": 251}},
  "ppcr": 0.6568848758465011
}
```
</details>
