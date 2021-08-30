# Test report for javascript / file:///tmp/top-repos-quality-repos-1xyiwzkj/recurse-graph-wiki.git HEAD 849622735ce0aa7ee4ad2e8fc570bc579152edc6

### Classification report

PPCR: 0.790

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 0.938| 0.685| 0.961| 0.808| 714| 978| 0.730 |
| `'` | 0.931| 0.996| 0.996| 0.963| 0.963| 464| 464| 1.000 |
| `␣` | 0.885| 0.950| 0.784| 0.916| 0.832| 439| 532| 0.825 |
| `⏎` | 0.827| 0.669| 0.397| 0.740| 0.536| 121| 204| 0.593 |
| `⏎␣⁺␣⁺` | 0.818| 0.969| 0.887| 0.887| 0.851| 65| 71| 0.915 |
| `⏎␣⁻␣⁻` | 0.958| 1.000| 0.648| 0.979| 0.773| 46| 71| 0.648 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 47| 0.468 |
| `micro avg` | 0.929| 0.929| 0.735| 0.929| 0.821| 1871| 2367| 0.790 |
| `macro avg` | 0.772| 0.789| 0.628| 0.778| 0.680| 1871| 2367| 0.790 |
| `weighted avg` | 0.920| 0.929| 0.735| 0.923| 0.804| 1871| 2367| 0.790 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|264 |670 |32 |0 |0 |12 |0 |0 |
|93 |9 |417 |7 |2 |2 |2 |0 |
|0 |1 |1 |462 |0 |0 |0 |0 |
|83 |0 |14 |26 |81 |0 |0 |0 |
|6 |1 |0 |1 |0 |63 |0 |0 |
|25 |0 |0 |0 |0 |0 |46 |0 |
|25 |0 |7 |0 |15 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9625, "precision": 0.9314516129032258, "recall": 0.9956896551724138, "support": 464}, "macro avg": {"f1-score": 0.7779043383572253, "precision": 0.7719564255059096, "recall": 0.7889433381328778, "support": 1871}, "micro avg": {"f1-score": 0.9294494922501336, "precision": 0.9294494922501336, "recall": 0.9294494922501336, "support": 1871}, "weighted avg": {"f1-score": 0.923030416007855, "precision": 0.9196176665661256, "recall": 0.9294494922501336, "support": 1871}, "\u2205": {"f1-score": 0.960573476702509, "precision": 0.9838472834067548, "recall": 0.938375350140056, "support": 714}, "\u23ce": {"f1-score": 0.7397260273972602, "precision": 0.826530612244898, "recall": 0.6694214876033058, "support": 121}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8873239436619719, "precision": 0.8181818181818182, "recall": 0.9692307692307692, "support": 65}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9787234042553191, "precision": 0.9583333333333334, "recall": 1.0, "support": 46}, "\u2423": {"f1-score": 0.9164835164835164, "precision": 0.8853503184713376, "recall": 0.9498861047835991, "support": 439}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9625, "precision": 0.9314516129032258, "recall": 0.9956896551724138, "support": 464}, "macro avg": {"f1-score": 0.6803722015596543, "precision": 0.7719564255059096, "recall": 0.6281237010593931, "support": 2367}, "micro avg": {"f1-score": 0.8206701274185938, "precision": 0.9294494922501336, "recall": 0.7346852555978032, "support": 2367}, "weighted avg": {"f1-score": 0.8042559499471739, "precision": 0.912609793680266, "recall": 0.7346852555978032, "support": 2367}, "\u2205": {"f1-score": 0.8077154912597951, "precision": 0.9838472834067548, "recall": 0.6850715746421268, "support": 978}, "\u23ce": {"f1-score": 0.5364238410596026, "precision": 0.826530612244898, "recall": 0.39705882352941174, "support": 204}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8513513513513514, "precision": 0.8181818181818182, "recall": 0.8873239436619719, "support": 71}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7731092436974789, "precision": 0.9583333333333334, "recall": 0.647887323943662, "support": 71}, "\u2423": {"f1-score": 0.8315054835493519, "precision": 0.8853503184713376, "recall": 0.7838345864661654, "support": 532}},
  "ppcr": 0.7904520490071821
}
```
</details>