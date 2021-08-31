# Train report for javascript / file:///tmp/top-repos-quality-repos-ryqlbjzk/content.git HEAD 0b6a7be6f6287cde880b0d37c88e6461efa167ff

### Classification report

PPCR: 0.809

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.995| 0.939| 0.983| 0.955| 10058| 10661| 0.943 |
| `␣` | 0.966| 0.945| 0.739| 0.956| 0.837| 4581| 5860| 0.782 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 2744| 2744| 1.000 |
| `⏎␣⁺␣⁺` | 0.949| 0.891| 0.787| 0.919| 0.861| 880| 996| 0.884 |
| `⏎⏎` | 0.951| 0.971| 0.459| 0.960| 0.619| 238| 503| 0.473 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 60| 1219| 0.049 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 997| 0.036 |
| `macro avg` | 0.691| 0.686| 0.561| 0.688| 0.610| 18597| 22980| 0.809 |
| `weighted avg` | 0.968| 0.973| 0.788| 0.970| 0.827| 18597| 22980| 0.809 |
| `micro avg` | 0.973| 0.973| 0.788| 0.973| 0.871| 18597| 22980| 0.809 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|603 |10008 |50 |0 |0 |0 |0 |0 |
|1279 |207 |4331 |0 |0 |42 |0 |1 |
|0 |0 |0 |2744 |0 |0 |0 |0 |
|1159 |29 |20 |0 |0 |0 |0 |11 |
|116 |21 |75 |0 |0 |784 |0 |0 |
|961 |36 |0 |0 |0 |0 |0 |0 |
|265 |0 |7 |0 |0 |0 |0 |231 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/content/lib/index.js | 44 |
| packages/content/lib/database.js | 43 |
| packages/content/test/module.test.js | 40 |
| packages/content/test/server.test.js | 38 |
| packages/content/test/plugin.test.js | 37 |
| packages/content/lib/query-builder.js | 31 |
| packages/content/templates/nuxt-content.js | 26 |
| packages/theme-docs/src/store/index.js | 24 |
| packages/content/test/options.test.js | 23 |
| packages/create-nuxt-content-docs/src/saofile.js | 18 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2744}, "macro avg": {"f1-score": 0.6883441602628816, "precision": 0.6910600170446388, "recall": 0.6859932745269582, "support": 18597}, "micro avg": {"f1-score": 0.9731677152228854, "precision": 0.9731677152228854, "recall": 0.9731677152228854, "support": 18597}, "weighted avg": {"f1-score": 0.9704677348656242, "precision": 0.96806421622622, "recall": 0.9731677152228854, "support": 18597}, "\u2205": {"f1-score": 0.9831524141657252, "precision": 0.9715561595961557, "recall": 0.9950288327699344, "support": 10058}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 60}, "\u23ce\u23ce": {"f1-score": 0.9604989604989606, "precision": 0.9506172839506173, "recall": 0.9705882352941176, "support": 238}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9191090269636577, "precision": 0.9491525423728814, "recall": 0.8909090909090909, "support": 880}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.955648720211827, "precision": 0.9660941333928174, "recall": 0.9454267627155643, "support": 4581}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2744}, "macro avg": {"f1-score": 0.6103201490335571, "precision": 0.6910600170446388, "recall": 0.5606029051037896, "support": 22980}, "micro avg": {"f1-score": 0.8705774827428625, "precision": 0.9731677152228854, "recall": 0.7875543951261967, "support": 22980}, "weighted avg": {"f1-score": 0.8268119135385726, "precision": 0.8784416129315525, "recall": 0.7875543951261967, "support": 22980}, "\u2205": {"f1-score": 0.9548707184428967, "precision": 0.9715561595961557, "recall": 0.9387487102523215, "support": 10661}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1219}, "\u23ce\u23ce": {"f1-score": 0.6193029490616622, "precision": 0.9506172839506173, "recall": 0.4592445328031809, "support": 503}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8605927552140504, "precision": 0.9491525423728814, "recall": 0.7871485943775101, "support": 996}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 997}, "\u2423": {"f1-score": 0.8374746205162913, "precision": 0.9660941333928174, "recall": 0.7390784982935154, "support": 5860}},
  "ppcr": 0.8092689295039165
}
```
</details>
