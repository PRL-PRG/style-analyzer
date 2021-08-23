# Test report for javascript / file:///tmp/top-repos-quality-repos-mn5g8vcu/wordcloudapi.git HEAD bc229db8089f57c3ac2f310674232575f6a5b7f8

### Classification report

PPCR: 0.727

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.927| 0.976| 0.843| 0.951| 0.883| 627| 726| 0.864 |
| `␣` | 0.829| 0.829| 0.429| 0.829| 0.565| 152| 294| 0.517 |
| `"` | 0.940| 1.000| 0.877| 0.969| 0.907| 142| 162| 0.877 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 40| 0.400 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 39| 0.359 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 63| 0.190 |
| `macro avg` | 0.449| 0.468| 0.358| 0.458| 0.393| 963| 1324| 0.727 |
| `weighted avg` | 0.873| 0.914| 0.665| 0.893| 0.721| 963| 1324| 0.727 |
| `micro avg` | 0.914| 0.914| 0.665| 0.914| 0.770| 963| 1324| 0.727 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|99 |612 |13 |2 |0 |0 |0 |
|142 |23 |126 |3 |0 |0 |0 |
|20 |0 |0 |142 |0 |0 |0 |
|51 |3 |7 |2 |0 |0 |0 |
|24 |8 |6 |2 |0 |0 |0 |
|25 |14 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.969283276450512, "precision": 0.9403973509933775, "recall": 1.0, "support": 142}, "macro avg": {"f1-score": 0.4582132659867526, "precision": 0.44943624111452624, "recall": 0.4675039872408293, "support": 963}, "micro avg": {"f1-score": 0.913811007268951, "precision": 0.9138110072689511, "recall": 0.9138110072689511, "support": 963}, "weighted avg": {"f1-score": 0.89298641491554, "precision": 0.8732465460447141, "recall": 0.9138110072689511, "support": 963}, "\u2205": {"f1-score": 0.951048951048951, "precision": 0.9272727272727272, "recall": 0.9760765550239234, "support": 627}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.8289473684210527, "precision": 0.8289473684210527, "recall": 0.8289473684210527, "support": 152}},
  "cl_report_full": {"\"": {"f1-score": 0.9073482428115016, "precision": 0.9403973509933775, "recall": 0.8765432098765432, "support": 162}, "macro avg": {"f1-score": 0.3925812579088414, "precision": 0.44943624111452624, "recall": 0.35801497417659034, "support": 1324}, "micro avg": {"f1-score": 0.769567118495846, "precision": 0.9138110072689511, "recall": 0.6646525679758308, "support": 1324}, "weighted avg": {"f1-score": 0.7207325259868365, "precision": 0.8075943332150428, "recall": 0.6646525679758308, "support": 1324}, "\u2205": {"f1-score": 0.8831168831168832, "precision": 0.9272727272727272, "recall": 0.8429752066115702, "support": 726}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.5650224215246636, "precision": 0.8289473684210527, "recall": 0.42857142857142855, "support": 294}},
  "ppcr": 0.7273413897280967
}
```
</details>
