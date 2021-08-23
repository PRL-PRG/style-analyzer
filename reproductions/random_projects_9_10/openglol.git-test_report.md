# Test report for javascript / file:///tmp/top-repos-quality-repos-kl6i6z45/openglol.git HEAD 9f62f0ded5041f76b8518a002509722b2c152a4b

### Classification report

PPCR: 0.831

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.902| 0.996| 0.936| 0.947| 0.919| 678| 721| 0.940 |
| `␣` | 0.979| 0.882| 0.756| 0.928| 0.853| 323| 377| 0.857 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 110| 0.318 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 42| 0.071 |
| `micro avg` | 0.924| 0.924| 0.768| 0.924| 0.839| 1039| 1250| 0.831 |
| `macro avg` | 0.470| 0.469| 0.423| 0.469| 0.443| 1039| 1250| 0.831 |
| `weighted avg` | 0.893| 0.924| 0.768| 0.906| 0.787| 1039| 1250| 0.831 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|43 |675 |3 |0 |0 |
|54 |38 |285 |0 |0 |
|75 |32 |3 |0 |0 |
|39 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.4687607073840404, "precision": 0.47044696510281714, "recall": 0.4694820406038521, "support": 1039}, "micro avg": {"f1-score": 0.9239653512993262, "precision": 0.9239653512993262, "recall": 0.9239653512993262, "support": 1039}, "weighted avg": {"f1-score": 0.9063703347826743, "precision": 0.8933318161575612, "recall": 0.9239653512993262, "support": 1039}, "\u2205": {"f1-score": 0.9467040673211782, "precision": 0.9024064171122995, "recall": 0.995575221238938, "support": 678}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9283387622149836, "precision": 0.979381443298969, "recall": 0.8823529411764706, "support": 323}},
  "cl_report_full": {"macro avg": {"f1-score": 0.44307148127162965, "precision": 0.47044696510281714, "recall": 0.4230419730921907, "support": 1250}, "micro avg": {"f1-score": 0.8387942332896461, "precision": 0.9239653512993262, "recall": 0.768, "support": 1250}, "weighted avg": {"f1-score": 0.7874281742845147, "precision": 0.8158894646893433, "recall": 0.768, "support": 1250}, "\u2205": {"f1-score": 0.9189925119128659, "precision": 0.9024064171122995, "recall": 0.9361997226074896, "support": 721}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 110}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u2423": {"f1-score": 0.8532934131736527, "precision": 0.979381443298969, "recall": 0.7559681697612732, "support": 377}},
  "ppcr": 0.8312
}
```
</details>
