# Train report for javascript / file:///tmp/top-repos-quality-repos-t71h_dym/objc.git HEAD 14168534948813a7e1b5e656c879b1ed032f3306

### Classification report

PPCR: 0.404

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 1.000| 0.595| 0.986| 0.738| 3713| 6236| 0.595 |
| `'` | 1.000| 1.000| 0.496| 1.000| 0.663| 617| 1244| 0.496 |
| `␣` | 1.000| 0.743| 0.092| 0.853| 0.169| 393| 3173| 0.124 |
| `⏎␣⁻␣⁻` | 0.996| 0.984| 0.718| 0.990| 0.834| 248| 340| 0.729 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 342| 0.015 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 512| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 468| 0.000 |
| `macro avg` | 0.567| 0.532| 0.272| 0.547| 0.343| 4976| 12315| 0.404 |
| `micro avg` | 0.978| 0.978| 0.395| 0.978| 0.563| 4976| 12315| 0.404 |
| `weighted avg` | 0.978| 0.978| 0.395| 0.976| 0.507| 4976| 12315| 0.404 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2523 |3713 |0 |0 |0 |0 |0 |0 |
|2780 |100 |292 |0 |0 |0 |0 |1 |
|627 |0 |0 |617 |0 |0 |0 |0 |
|512 |0 |0 |0 |0 |0 |0 |0 |
|468 |0 |0 |0 |0 |0 |0 |0 |
|337 |5 |0 |0 |0 |0 |0 |0 |
|92 |4 |0 |0 |0 |0 |0 |244 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test.js | 61 |
| src/type-encodings.js | 9 |
| src/instance.js | 8 |
| src/proxies.js | 5 |
| src/block.js | 4 |
| examples/blocks.js | 4 |
| examples/swizzle.js | 4 |
| examples/nsarray.js | 4 |
| src/util.js | 4 |
| src/selector.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 617}, "macro avg": {"f1-score": 0.5468495615073249, "precision": 0.5667713239141811, "recall": 0.5324105017530283, "support": 4976}, "micro avg": {"f1-score": 0.9778938906752411, "precision": 0.9778938906752411, "recall": 0.9778938906752411, "support": 4976}, "weighted avg": {"f1-score": 0.9760504801329742, "precision": 0.977511321743637, "recall": 0.9778938906752411, "support": 4976}, "\u2205": {"f1-score": 0.9855341738553417, "precision": 0.9714809000523287, "recall": 1.0, "support": 3713}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9898580121703854, "precision": 0.9959183673469387, "recall": 0.9838709677419355, "support": 248}, "\u2423": {"f1-score": 0.8525547445255475, "precision": 1.0, "recall": 0.7430025445292621, "support": 393}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6630843632455669, "precision": 1.0, "recall": 0.4959807073954984, "support": 1244}, "macro avg": {"f1-score": 0.34344753185507365, "precision": 0.5667713239141811, "recall": 0.2715811380479992, "support": 12315}, "micro avg": {"f1-score": 0.5628361575386038, "precision": 0.9778938906752411, "recall": 0.3951278928136419, "support": 12315}, "weighted avg": {"f1-score": 0.5073029624195913, "precision": 0.8780972097137052, "recall": 0.3951278928136419, "support": 12315}, "\u2205": {"f1-score": 0.7383177570093458, "precision": 0.9714809000523287, "recall": 0.5954137267479154, "support": 6236}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 512}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 468}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 342}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8341880341880341, "precision": 0.9959183673469387, "recall": 0.7176470588235294, "support": 340}, "\u2423": {"f1-score": 0.16854256854256852, "precision": 1.0, "recall": 0.09202647336905137, "support": 3173}},
  "ppcr": 0.4040600893219651
}
```
</details>
