# Train report for javascript / file:///tmp/top-repos-quality-repos-sfowcawa/ts-node.git HEAD aaf60523ac0f77dc52b3c729f1f179a85dcac2c0

### Classification report

PPCR: 0.481

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 0.998| 0.739| 0.984| 0.840| 5663| 7641| 0.741 |
| `'` | 1.000| 1.000| 0.934| 1.000| 0.966| 1052| 1126| 0.934 |
| `␣` | 1.000| 0.887| 0.158| 0.940| 0.273| 1027| 5762| 0.178 |
| `⏎␣⁻␣⁻` | 0.956| 0.890| 0.744| 0.922| 0.837| 464| 555| 0.836 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 1129| 0.004 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 241| 0.004 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 634| 0.000 |
| `weighted avg` | 0.977| 0.977| 0.470| 0.976| 0.558| 8212| 17088| 0.481 |
| `micro avg` | 0.977| 0.977| 0.470| 0.977| 0.634| 8212| 17088| 0.481 |
| `macro avg` | 0.561| 0.539| 0.368| 0.549| 0.416| 8212| 17088| 0.481 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1978 |5649 |0 |0 |0 |0 |14 |0 |
|4735 |113 |911 |0 |0 |0 |3 |0 |
|1124 |3 |0 |0 |0 |0 |2 |0 |
|74 |0 |0 |0 |1052 |0 |0 |0 |
|634 |0 |0 |0 |0 |0 |0 |0 |
|91 |51 |0 |0 |0 |0 |413 |0 |
|240 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| dist-raw/node-esm-resolve-implementation.js | 55 |
| raw/node-esm-resolve-implementation-v15.3.0.js | 49 |
| raw/node-esm-resolve-implementation-v13.12.0.js | 30 |
| dist-raw/node-cjs-loader-utils.js | 9 |
| website/src/pages/index.js | 8 |
| dist-raw/node-package-json-reader.js | 8 |
| scripts/build-pack.js | 5 |
| src/bin-script-deprecated.ts | 3 |
| src/bin-script.ts | 2 |
| dist-raw/node-internal-fs.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1052}, "macro avg": {"f1-score": 0.5494522600439405, "precision": 0.5610196645846466, "recall": 0.5392376683159757, "support": 8212}, "micro avg": {"f1-score": 0.9772284461763273, "precision": 0.9772284461763273, "recall": 0.9772284461763273, "support": 8212}, "weighted avg": {"f1-score": 0.976436813380453, "precision": 0.9768680280104962, "recall": 0.9772284461763273, "support": 8212}, "\u2205": {"f1-score": 0.9841463414634147, "precision": 0.9711191335740073, "recall": 0.9975278121137207, "support": 5663}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.921875, "precision": 0.9560185185185185, "recall": 0.8900862068965517, "support": 464}, "\u2423": {"f1-score": 0.9401444788441693, "precision": 1.0, "recall": 0.887049659201558, "support": 1027}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9660238751147842, "precision": 1.0, "recall": 0.9342806394316163, "support": 1126}, "macro avg": {"f1-score": 0.41649208398646614, "precision": 0.5610196645846466, "recall": 0.36797582098340376, "support": 17088}, "micro avg": {"f1-score": 0.6343873517786561, "precision": 0.9772284461763273, "recall": 0.46962780898876405, "support": 17088}, "weighted avg": {"f1-score": 0.5582921103335815, "precision": 0.868381997742086, "recall": 0.46962780898876405, "support": 17088}, "\u2205": {"f1-score": 0.8395006687472136, "precision": 0.9711191335740073, "recall": 0.7393011385944248, "support": 7641}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1129}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 241}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 634}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8368794326241135, "precision": 0.9560185185185185, "recall": 0.7441441441441441, "support": 555}, "\u2423": {"f1-score": 0.27304061141915176, "precision": 1.0, "recall": 0.1581048247136411, "support": 5762}},
  "ppcr": 0.48057116104868913
}
```
</details>
