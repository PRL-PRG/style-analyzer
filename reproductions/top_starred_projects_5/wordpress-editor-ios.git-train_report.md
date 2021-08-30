# Train report for javascript / file:///tmp/top-repos-quality-repos-5le2datz/wordpress-editor-ios.git HEAD b56b59e8f76b6eaf0bb378b487d7ec38db5a407e

### Classification report

PPCR: 0.778

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.980| 0.931| 0.979| 0.953| 30394| 31999| 0.950 |
| `␣` | 0.949| 0.968| 0.912| 0.958| 0.930| 18588| 19724| 0.942 |
| `⏎⏎` | 0.937| 0.885| 0.313| 0.910| 0.469| 624| 1766| 0.353 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 178| 2132| 0.083 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 146| 3156| 0.046 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 2111| 0.036 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 97| 0.247 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 95| 0.021 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1517| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1675| 0.000 |
| `weighted avg` | 0.958| 0.966| 0.752| 0.962| 0.773| 50031| 64272| 0.778 |
| `macro avg` | 0.286| 0.283| 0.216| 0.285| 0.235| 50031| 64272| 0.778 |
| `micro avg` | 0.966| 0.966| 0.752| 0.966| 0.846| 50031| 64272| 0.778 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| "| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1605 |29790 |598 |0 |0 |0 |0 |6 |0 |0 |0 |
|1136 |601 |17985 |0 |0 |0 |0 |2 |0 |0 |0 |
|3010 |13 |104 |0 |0 |0 |0 |29 |0 |0 |0 |
|1517 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2036 |59 |16 |0 |0 |0 |0 |0 |0 |0 |0 |
|1954 |21 |157 |0 |0 |0 |0 |0 |0 |0 |0 |
|1142 |9 |63 |0 |0 |0 |0 |552 |0 |0 |0 |
|1675 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|73 |0 |24 |0 |0 |0 |0 |0 |0 |0 |0 |
|93 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Assets/ZSSRichTextEditor.js | 488 |
| Assets/rangy-core.js | 360 |
| Assets/rangy-textrange.js | 275 |
| Assets/rangy-classapplier.js | 150 |
| Assets/js-beautifier.js | 118 |
| Assets/shortcode.js | 113 |
| Assets/rangy-highlighter.js | 69 |
| Assets/rangy-serializer.js | 46 |
| Assets/wpload.js | 36 |
| Assets/wpsave.js | 33 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2846947593897415, "precision": 0.28632899103343284, "recall": 0.283230275733552, "support": 50031}, "micro avg": {"f1-score": 0.9659411165077652, "precision": 0.9659411165077652, "recall": 0.9659411165077652, "support": 50031}, "weighted avg": {"f1-score": 0.9618336082946747, "precision": 0.9578134339386314, "recall": 0.9659411165077652, "support": 50031}, "\u2205": {"f1-score": 0.9785018640476932, "precision": 0.9768814559763895, "recall": 0.9801276567743633, "support": 30394}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.9101401483924154, "precision": 0.9371816638370118, "recall": 0.8846153846153846, "support": 624}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 178}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u2423": {"f1-score": 0.9583055814573065, "precision": 0.9492267905209268, "recall": 0.9675597159457715, "support": 18588}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1675}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1517}, "macro avg": {"f1-score": 0.23523157118340707, "precision": 0.28632899103343284, "recall": 0.21553706736665385, "support": 64272}, "micro avg": {"f1-score": 0.8455946038161727, "precision": 0.9659411165077652, "recall": 0.7519137415982076, "support": 64272}, "weighted avg": {"f1-score": 0.7729833078896513, "precision": 0.8034111548475139, "recall": 0.7519137415982076, "support": 64272}, "\u2205": {"f1-score": 0.953371523666272, "precision": 0.9768814559763895, "recall": 0.9309665927060221, "support": 31999}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3156}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u23ce": {"f1-score": 0.4687898089171974, "precision": 0.9371816638370118, "recall": 0.31257078142695355, "support": 1766}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2132}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2111}, "\u2423": {"f1-score": 0.9301543792506013, "precision": 0.9492267905209268, "recall": 0.9118332995335632, "support": 19724}},
  "ppcr": 0.7784260642270351
}
```
</details>
