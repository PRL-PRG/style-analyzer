# Test report for javascript / file:///tmp/top-repos-quality-repos-f4lo97qx/algorithm.git HEAD 4cd2f5ca51357494fafd058c105264bd8f02a7b3

### Classification report

PPCR: 0.828

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.908| 0.920| 0.854| 0.914| 0.880| 602| 649| 0.928 |
| `␣` | 0.856| 0.949| 0.820| 0.900| 0.837| 369| 427| 0.864 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 76| 0.250 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 24| 0.708 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 28| 0.357 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 27| 0.074 |
| `macro avg` | 0.294| 0.311| 0.279| 0.302| 0.286| 1019| 1231| 0.828 |
| `weighted avg` | 0.846| 0.887| 0.734| 0.866| 0.754| 1019| 1231| 0.828 |
| `micro avg` | 0.887| 0.887| 0.734| 0.887| 0.804| 1019| 1231| 0.828 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|47 |554 |48 |0 |0 |0 |0 |
|58 |19 |350 |0 |0 |0 |0 |
|57 |10 |9 |0 |0 |0 |0 |
|7 |15 |2 |0 |0 |0 |0 |
|25 |2 |0 |0 |0 |0 |0 |
|18 |10 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.30232239162219565, "precision": 0.29399040709714486, "recall": 0.3114625443042913, "support": 1019}, "micro avg": {"f1-score": 0.887144259077527, "precision": 0.887144259077527, "recall": 0.887144259077527, "support": 1019}, "weighted avg": {"f1-score": 0.8658963451536871, "precision": 0.846422568575714, "recall": 0.887144259077527, "support": 1019}, "\u2205": {"f1-score": 0.9141914191419142, "precision": 0.9081967213114754, "recall": 0.920265780730897, "support": 602}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.8997429305912598, "precision": 0.8557457212713936, "recall": 0.948509485094851, "support": 369}},
  "cl_report_full": {"macro avg": {"f1-score": 0.28623068610945374, "precision": 0.29399040709714486, "recall": 0.2788821810772353, "support": 1231}, "micro avg": {"f1-score": 0.8035555555555557, "precision": 0.887144259077527, "recall": 0.7343623070674249, "support": 1231}, "weighted avg": {"f1-score": 0.7544249587701852, "precision": 0.7756483307181419, "recall": 0.7343623070674249, "support": 1231}, "\u2205": {"f1-score": 0.880063542494043, "precision": 0.9081967213114754, "recall": 0.8536209553158706, "support": 649}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u2423": {"f1-score": 0.8373205741626795, "precision": 0.8557457212713936, "recall": 0.819672131147541, "support": 427}},
  "ppcr": 0.8277822908204712
}
```
</details>
