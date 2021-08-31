# Train report for javascript / file:///tmp/top-repos-quality-repos-rv0csyb4/cuckoo.git HEAD 50452a39ff7c3e0c4c94d114bc6317101633b958

### Classification report

PPCR: 0.791

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.974| 0.945| 0.973| 0.958| 17229| 17753| 0.970 |
| `␣` | 0.901| 0.941| 0.873| 0.921| 0.887| 5862| 6321| 0.927 |
| `⏎` | 0.866| 0.851| 0.436| 0.859| 0.580| 806| 1575| 0.512 |
| `'` | 1.000| 1.000| 0.197| 1.000| 0.329| 359| 1822| 0.197 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 166| 896| 0.185 |
| `"` | 1.000| 1.000| 0.089| 1.000| 0.163| 67| 756| 0.089 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 417| 0.060 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 417| 0.055 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 436| 0.046 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 407| 0.044 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 10| 0.900 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 37| 0.162 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 88| 0.034 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 111| 0.009 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 27| 0.000 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.296| 0.298| 0.159| 0.297| 0.182| 24594| 31073| 0.791 |
| `weighted avg` | 0.942| 0.952| 0.753| 0.946| 0.781| 24594| 31073| 0.791 |
| `micro avg` | 0.952| 0.952| 0.753| 0.952| 0.841| 24594| 31073| 0.791 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁻| ⏎⇥⁺| ␣␣| ⏎⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|524 |16776 |440 |13 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|459 |295 |5516 |51 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|769 |55 |65 |686 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|730 |70 |59 |37 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1463 |0 |0 |0 |0 |359 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|689 |0 |0 |0 |0 |0 |67 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|394 |7 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|416 |16 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|85 |2 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|110 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|31 |0 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|27 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|392 |24 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|389 |8 |10 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |3 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| cuckoo/web/src/scripts/submission/submission.js | 174 |
| cuckoo/web/src/scripts/app.js | 141 |
| cuckoo/web/static/js/hexdump.js | 136 |
| cuckoo/web/src/scripts/analysis_network.js | 103 |
| cuckoo/web/src/scripts/submission/components/DnDUpload.js | 88 |
| cuckoo/web/src/scripts/submission/components/FileTree.js | 77 |
| cuckoo/web/src/scripts/submission/components/InterfaceControllers.js | 60 |
| cuckoo/web/src/scripts/analysis_behavior.js | 59 |
| cuckoo/web/static/js/js.cookie.js | 59 |
| cuckoo/data-private/html/static/js/ui.js | 53 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 67}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 359}, "macro avg": {"f1-score": 0.29701452375325543, "precision": 0.29622827462632545, "recall": 0.2978624702005729, "support": 24594}, "micro avg": {"f1-score": 0.9516142148491502, "precision": 0.9516142148491502, "recall": 0.9516142148491502, "support": 24594}, "weighted avg": {"f1-score": 0.9464952061493582, "precision": 0.941584941483278, "recall": 0.9516142148491502, "support": 24594}, "\u2205": {"f1-score": 0.9729447585906916, "precision": 0.972183588317107, "recall": 0.9737071217133902, "support": 17229}, "\u23ce": {"f1-score": 0.8585732165206508, "precision": 0.8661616161616161, "recall": 0.8511166253101737, "support": 806}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u2423": {"f1-score": 0.9207144049407444, "precision": 0.9013071895424837, "recall": 0.9409757761856021, "support": 5862}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}},
  "cl_report_full": {"\"": {"f1-score": 0.16281895504252733, "precision": 1.0, "recall": 0.08862433862433862, "support": 756}, "\u0027": {"f1-score": 0.3292067858780376, "precision": 1.0, "recall": 0.19703622392974754, "support": 1822}, "macro avg": {"f1-score": 0.18229937378693212, "precision": 0.29622827462632545, "recall": 0.15867686869027464, "support": 31073}, "micro avg": {"f1-score": 0.8408572403758061, "precision": 0.9516142148491502, "recall": 0.7531940913333118, "support": 31073}, "weighted avg": {"f1-score": 0.7805847367967044, "precision": 0.865656439157667, "recall": 0.7531940913333118, "support": 31073}, "\u2205": {"f1-score": 0.9583821303093489, "precision": 0.972183588317107, "recall": 0.9449670478229032, "support": 17753}, "\u23ce": {"f1-score": 0.5796366708914237, "precision": 0.8661616161616161, "recall": 0.43555555555555553, "support": 1575}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 407}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 417}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 896}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 111}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 417}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 436}, "\u2423": {"f1-score": 0.8867454384695764, "precision": 0.9013071895424837, "recall": 0.8726467331118494, "support": 6321}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}},
  "ppcr": 0.791491005052618
}
```
</details>
