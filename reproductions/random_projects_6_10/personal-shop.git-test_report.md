# Test report for javascript / file:///tmp/top-repos-quality-repos-tsc38vid/personal-shop.git HEAD 1c6ee0d8f873ca15edf37c40658ec31a045cf2ee

### Classification report

PPCR: 0.684

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.728| 0.979| 0.693| 0.835| 0.710| 284| 401| 0.708 |
| `␣` | 0.647| 0.125| 0.075| 0.210| 0.134| 88| 147| 0.599 |
| `⏎` | 1.000| 0.121| 0.089| 0.216| 0.163| 33| 45| 0.733 |
| `"` | 0.500| 1.000| 1.000| 0.667| 0.667| 2| 2| 1.000 |
| `micro avg` | 0.725| 0.725| 0.496| 0.725| 0.589| 407| 595| 0.684 |
| `weighted avg` | 0.731| 0.725| 0.496| 0.649| 0.526| 407| 595| 0.684 |
| `macro avg` | 0.719| 0.556| 0.464| 0.482| 0.419| 407| 595| 0.684 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|117 |278 |6 |0 |0 |
|59 |75 |11 |2 |0 |
|0 |0 |0 |2 |0 |
|12 |29 |0 |0 |4 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.6666666666666666, "precision": 0.5, "recall": 1.0, "support": 2}, "macro avg": {"f1-score": 0.4818103818103818, "precision": 0.7187018786572221, "recall": 0.5562713401621853, "support": 407}, "micro avg": {"f1-score": 0.724815724815725, "precision": 0.7248157248157249, "recall": 0.7248157248157249, "support": 407}, "weighted avg": {"f1-score": 0.6486478054045621, "precision": 0.7312575055106624, "recall": 0.7248157248157249, "support": 407}, "\u2205": {"f1-score": 0.8348348348348349, "precision": 0.7277486910994765, "recall": 0.9788732394366197, "support": 284}, "\u23ce": {"f1-score": 0.21621621621621626, "precision": 1.0, "recall": 0.12121212121212122, "support": 33}, "\u2423": {"f1-score": 0.20952380952380953, "precision": 0.6470588235294118, "recall": 0.125, "support": 88}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 0.5, "recall": 1.0, "support": 2}, "macro avg": {"f1-score": 0.41854192849927563, "precision": 0.7187018786572221, "recall": 0.4642464134448459, "support": 595}, "micro avg": {"f1-score": 0.5888223552894213, "precision": 0.7248157248157249, "recall": 0.4957983193277311, "support": 595}, "weighted avg": {"f1-score": 0.5262951825235949, "precision": 0.7276384406549808, "recall": 0.4957983193277311, "support": 595}, "\u2205": {"f1-score": 0.7100893997445723, "precision": 0.7277486910994765, "recall": 0.6932668329177057, "support": 401}, "\u23ce": {"f1-score": 0.163265306122449, "precision": 1.0, "recall": 0.08888888888888889, "support": 45}, "\u2423": {"f1-score": 0.13414634146341464, "precision": 0.6470588235294118, "recall": 0.07482993197278912, "support": 147}},
  "ppcr": 0.6840336134453782
}
```
</details>
