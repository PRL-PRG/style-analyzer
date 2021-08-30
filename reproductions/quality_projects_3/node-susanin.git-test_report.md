# Test report for javascript / file:///tmp/top-repos-quality-repos-2vqzdidv/node-susanin.git HEAD 6f35e4da69815d6250e16655af8a32bcaded0ea7

### Classification report

PPCR: 0.900

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.886| 0.980| 0.957| 0.931| 0.920| 815| 835| 0.976 |
| `␣` | 0.905| 0.761| 0.752| 0.827| 0.822| 503| 509| 0.988 |
| `'` | 0.909| 1.000| 1.000| 0.952| 0.952| 120| 120| 1.000 |
| `⏎` | 0.940| 0.839| 0.591| 0.887| 0.726| 112| 159| 0.704 |
| `⏎␣⁺␣⁺` | 0.762| 0.853| 0.853| 0.805| 0.805| 75| 75| 1.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 69| 0.159 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 56| 0.089 |
| `weighted avg` | 0.883| 0.890| 0.801| 0.883| 0.810| 1641| 1823| 0.900 |
| `micro avg` | 0.890| 0.890| 0.801| 0.890| 0.843| 1641| 1823| 0.900 |
| `macro avg` | 0.629| 0.633| 0.593| 0.629| 0.604| 1641| 1823| 0.900 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|20 |799 |16 |0 |0 |0 |0 |0 |
|6 |82 |383 |6 |12 |20 |0 |0 |
|47 |9 |9 |94 |0 |0 |0 |0 |
|0 |0 |0 |0 |120 |0 |0 |0 |
|0 |10 |1 |0 |0 |64 |0 |0 |
|58 |0 |11 |0 |0 |0 |0 |0 |
|51 |2 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9523809523809523, "precision": 0.9090909090909091, "recall": 1.0, "support": 120}, "macro avg": {"f1-score": 0.6288731062790172, "precision": 0.6288917622685879, "recall": 0.6334883653299103, "support": 1641}, "micro avg": {"f1-score": 0.8897014015843998, "precision": 0.8897014015843998, "recall": 0.8897014015843998, "support": 1641}, "weighted avg": {"f1-score": 0.8827466373517224, "precision": 0.8829270836160138, "recall": 0.8897014015843998, "support": 1641}, "\u2205": {"f1-score": 0.9306930693069307, "precision": 0.885809312638581, "recall": 0.9803680981595092, "support": 815}, "\u23ce": {"f1-score": 0.8867924528301886, "precision": 0.94, "recall": 0.8392857142857143, "support": 112}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8050314465408804, "precision": 0.7619047619047619, "recall": 0.8533333333333334, "support": 75}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.8272138228941684, "precision": 0.9054373522458629, "recall": 0.7614314115308151, "support": 503}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9523809523809523, "precision": 0.9090909090909091, "recall": 1.0, "support": 120}, "macro avg": {"f1-score": 0.6035923583711671, "precision": 0.6288917622685879, "recall": 0.5934100464442146, "support": 1823}, "micro avg": {"f1-score": 0.8429561200923786, "precision": 0.8897014015843998, "recall": 0.8008776741634668, "support": 1823}, "weighted avg": {"f1-score": 0.8099825416078803, "precision": 0.8317126465058287, "recall": 0.8008776741634668, "support": 1823}, "\u2205": {"f1-score": 0.9199769717904432, "precision": 0.885809312638581, "recall": 0.9568862275449102, "support": 835}, "\u23ce": {"f1-score": 0.7258687258687259, "precision": 0.94, "recall": 0.5911949685534591, "support": 159}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8050314465408804, "precision": 0.7619047619047619, "recall": 0.8533333333333334, "support": 75}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}, "\u2423": {"f1-score": 0.8218884120171674, "precision": 0.9054373522458629, "recall": 0.7524557956777996, "support": 509}},
  "ppcr": 0.90016456390565
}
```
</details>