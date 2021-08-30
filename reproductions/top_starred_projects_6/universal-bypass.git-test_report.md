# Test report for javascript / file:///tmp/top-repos-quality-repos-wzyxmepu/universal-bypass.git HEAD 404d1a46fd22020635a977fdf8767ce707b10a76

### Classification report

PPCR: 0.947

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.975| 0.929| 0.969| 0.946| 1366| 1434| 0.953 |
| `"` | 0.994| 1.000| 1.000| 0.997| 0.997| 336| 336| 1.000 |
| `⏎` | 0.844| 0.857| 0.747| 0.850| 0.792| 189| 217| 0.871 |
| `⏎⇥⁻` | 0.948| 1.000| 1.000| 0.974| 0.974| 92| 92| 1.000 |
| `⏎⇥⁺` | 1.000| 1.000| 0.880| 1.000| 0.936| 81| 92| 0.880 |
| `␣` | 0.286| 0.125| 0.100| 0.174| 0.148| 48| 60| 0.800 |
| `micro avg` | 0.951| 0.951| 0.900| 0.951| 0.925| 2112| 2231| 0.947 |
| `weighted avg` | 0.943| 0.951| 0.900| 0.946| 0.918| 2112| 2231| 0.947 |
| `macro avg` | 0.839| 0.826| 0.776| 0.827| 0.799| 2112| 2231| 0.947 |

### Confusion matrix

|refusal|  ∅| "| ⏎| ␣| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|68 |1332 |0 |29 |0 |0 |5 |
|0 |0 |336 |0 |0 |0 |0 |
|28 |12 |0 |162 |15 |0 |0 |
|12 |39 |2 |1 |6 |0 |0 |
|11 |0 |0 |0 |0 |81 |0 |
|0 |0 |0 |0 |0 |0 |92 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9970326409495549, "precision": 0.9940828402366864, "recall": 1.0, "support": 336}, "macro avg": {"f1-score": 0.8273273373488399, "precision": 0.8391873964083368, "recall": 0.826208777801018, "support": 2112}, "micro avg": {"f1-score": 0.9512310606060606, "precision": 0.9512310606060606, "recall": 0.9512310606060606, "support": 2112}, "weighted avg": {"f1-score": 0.9462140924742587, "precision": 0.9427459753978681, "recall": 0.9512310606060606, "support": 2112}, "\u2205": {"f1-score": 0.9690796653328483, "precision": 0.9631236442516269, "recall": 0.9751098096632503, "support": 1366}, "\u23ce": {"f1-score": 0.8503937007874015, "precision": 0.84375, "recall": 0.8571428571428571, "support": 189}, "\u23ce\u21e5\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 81}, "\u23ce\u21e5\u207b": {"f1-score": 0.9735449735449735, "precision": 0.9484536082474226, "recall": 1.0, "support": 92}, "\u2423": {"f1-score": 0.17391304347826086, "precision": 0.2857142857142857, "recall": 0.125, "support": 48}},
  "cl_report_full": {"\"": {"f1-score": 0.9970326409495549, "precision": 0.9940828402366864, "recall": 1.0, "support": 336}, "macro avg": {"f1-score": 0.7988341479486744, "precision": 0.8391873964083368, "recall": 0.7759748090495947, "support": 2231}, "micro avg": {"f1-score": 0.925166935298181, "precision": 0.9512310606060606, "recall": 0.9004930524428507, "support": 2231}, "weighted avg": {"f1-score": 0.9178061404279948, "precision": 0.9388729176503718, "recall": 0.9004930524428507, "support": 2231}, "\u2205": {"f1-score": 0.9456869009584664, "precision": 0.9631236442516269, "recall": 0.9288702928870293, "support": 1434}, "\u23ce": {"f1-score": 0.7921760391198043, "precision": 0.84375, "recall": 0.7465437788018433, "support": 217}, "\u23ce\u21e5\u207a": {"f1-score": 0.9364161849710982, "precision": 1.0, "recall": 0.8804347826086957, "support": 92}, "\u23ce\u21e5\u207b": {"f1-score": 0.9735449735449735, "precision": 0.9484536082474226, "recall": 1.0, "support": 92}, "\u2423": {"f1-score": 0.14814814814814817, "precision": 0.2857142857142857, "recall": 0.1, "support": 60}},
  "ppcr": 0.94666069027342
}
```
</details>
