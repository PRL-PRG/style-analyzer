# Train report for javascript / file:///tmp/top-repos-quality-repos-y4ywcjea/uvu.git HEAD d56753ec20928da1c22ff093cb07ce3c67613706

### Classification report

PPCR: 0.844

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.993| 0.945| 0.990| 0.965| 13772| 14482| 0.951 |
| `␣` | 0.951| 0.977| 0.644| 0.964| 0.768| 4261| 6461| 0.659 |
| `'` | 1.000| 1.000| 0.996| 1.000| 0.998| 3492| 3506| 0.996 |
| `⏎⇥⁺` | 0.936| 0.887| 0.752| 0.911| 0.834| 648| 765| 0.847 |
| `⏎⇥⁻` | 1.000| 0.893| 0.646| 0.943| 0.785| 541| 748| 0.723 |
| `⏎` | 1.000| 0.687| 0.265| 0.814| 0.419| 450| 1167| 0.386 |
| `⏎⏎` | 0.964| 0.997| 0.490| 0.981| 0.650| 381| 776| 0.491 |
| `weighted avg` | 0.981| 0.980| 0.827| 0.980| 0.883| 23545| 27905| 0.844 |
| `macro avg` | 0.977| 0.919| 0.677| 0.943| 0.774| 23545| 27905| 0.844 |
| `micro avg` | 0.980| 0.980| 0.827| 0.980| 0.897| 23545| 27905| 0.844 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|710 |13680 |92 |0 |0 |0 |0 |0 |
|2200 |60 |4162 |0 |0 |39 |0 |0 |
|14 |0 |0 |3492 |0 |0 |0 |0 |
|717 |36 |91 |0 |309 |0 |0 |14 |
|117 |67 |6 |0 |0 |575 |0 |0 |
|207 |35 |23 |0 |0 |0 |483 |0 |
|395 |0 |1 |0 |0 |0 |0 |380 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/diff.js | 119 |
| test/assert.js | 106 |
| src/diff.js | 32 |
| src/index.js | 23 |
| test/suite.js | 20 |
| examples/preact/tests/Count.js | 19 |
| examples/svelte/tests/Count.js | 19 |
| bin.js | 16 |
| src/assert.js | 15 |
| examples/supertest/tests/pets.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3492}, "macro avg": {"f1-score": 0.9432670976274153, "precision": 0.9768565985725101, "recall": 0.9191806568476684, "support": 23545}, "micro avg": {"f1-score": 0.9802930558504991, "precision": 0.9802930558504991, "recall": 0.9802930558504991, "support": 23545}, "weighted avg": {"f1-score": 0.9797193815162378, "precision": 0.9805209094458375, "recall": 0.9802930558504991, "support": 23545}, "\u2205": {"f1-score": 0.9895117540687162, "precision": 0.9857328145265889, "recall": 0.9933197792622713, "support": 13772}, "\u23ce": {"f1-score": 0.8142292490118578, "precision": 1.0, "recall": 0.6866666666666666, "support": 450}, "\u23ce\u21e5\u207a": {"f1-score": 0.9112519809825674, "precision": 0.9364820846905537, "recall": 0.8873456790123457, "support": 648}, "\u23ce\u21e5\u207b": {"f1-score": 0.943359375, "precision": 1.0, "recall": 0.8927911275415896, "support": 541}, "\u23ce\u23ce": {"f1-score": 0.9806451612903225, "precision": 0.9644670050761421, "recall": 0.9973753280839895, "support": 381}, "\u2423": {"f1-score": 0.9638721630384436, "precision": 0.9513142857142857, "recall": 0.9767660173668153, "support": 4261}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9979994284081166, "precision": 1.0, "recall": 0.9960068454078722, "support": 3506}, "macro avg": {"f1-score": 0.7739794248358438, "precision": 0.9768565985725101, "recall": 0.6766612296451084, "support": 27905}, "micro avg": {"f1-score": 0.8972206025267251, "precision": 0.9802930558504991, "recall": 0.8271277548826376, "support": 27905}, "weighted avg": {"f1-score": 0.8833964573336691, "precision": 0.9785937792761662, "recall": 0.8271277548826376, "support": 27905}, "\u2205": {"f1-score": 0.9647390691114245, "precision": 0.9857328145265889, "recall": 0.944620908714266, "support": 14482}, "\u23ce": {"f1-score": 0.4186991869918699, "precision": 1.0, "recall": 0.2647814910025707, "support": 1167}, "\u23ce\u21e5\u207a": {"f1-score": 0.8339376359680928, "precision": 0.9364820846905537, "recall": 0.7516339869281046, "support": 765}, "\u23ce\u21e5\u207b": {"f1-score": 0.784727863525589, "precision": 1.0, "recall": 0.6457219251336899, "support": 748}, "\u23ce\u23ce": {"f1-score": 0.6495726495726495, "precision": 0.9644670050761421, "recall": 0.4896907216494845, "support": 776}, "\u2423": {"f1-score": 0.7681801402731635, "precision": 0.9513142857142857, "recall": 0.6441727286797709, "support": 6461}},
  "ppcr": 0.8437555993549543
}
```
</details>
