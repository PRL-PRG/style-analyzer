# Train report for javascript / file:///tmp/top-repos-quality-repos-ys7mc6mz/oauthd.git HEAD 2d8819d3c223eafa38f11c028db84b4162133b26

### Classification report

PPCR: 0.628

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.984| 0.673| 0.979| 0.796| 3031| 4432| 0.684 |
| `␣` | 0.960| 0.949| 0.705| 0.954| 0.813| 1830| 2464| 0.743 |
| `⏎` | 0.946| 0.982| 0.786| 0.963| 0.859| 495| 618| 0.801 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.979| 0.992| 0.958| 0.985| 0.968| 371| 384| 0.966 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 389| 0.062 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 96| 0.062 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 697| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 91| 0.000 |
| `micro avg` | 0.968| 0.968| 0.607| 0.968| 0.746| 5757| 9171| 0.628 |
| `weighted avg` | 0.963| 0.968| 0.607| 0.965| 0.701| 5757| 9171| 0.628 |
| `macro avg` | 0.482| 0.488| 0.390| 0.485| 0.429| 5757| 9171| 0.628 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1401 |2981 |50 |0 |0 |0 |0 |0 |0 |
|634 |67 |1736 |22 |0 |0 |5 |0 |0 |
|123 |3 |3 |486 |0 |0 |3 |0 |0 |
|697 |0 |0 |0 |0 |0 |0 |0 |0 |
|365 |8 |13 |3 |0 |0 |0 |0 |0 |
|13 |0 |3 |0 |0 |0 |368 |0 |0 |
|91 |0 |0 |0 |0 |0 |0 |0 |0 |
|90 |0 |3 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/scaffolding/templates/basis_structure/Gruntfile.js | 19 |
| Gruntfile.js | 19 |
| providers/vk/me.js | 17 |
| providers/foursquare/me.js | 17 |
| providers/dropbox/me.js | 16 |
| providers/500px/me.js | 6 |
| providers/github/me.js | 6 |
| config.js | 6 |
| providers/google/me.js | 5 |
| tests/instance_test/plugins/plugin_test/gruntConfig.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.485244616456579, "precision": 0.4823658947879415, "recall": 0.48823370029471014, "support": 5757}, "micro avg": {"f1-score": 0.9676915059927045, "precision": 0.9676915059927045, "recall": 0.9676915059927045, "support": 5757}, "weighted avg": {"f1-score": 0.9651168155948101, "precision": 0.9626505567221391, "recall": 0.9676915059927045, "support": 5757}, "\u2205": {"f1-score": 0.9789819376026272, "precision": 0.9745014710689768, "recall": 0.9835037941273507, "support": 3031}, "\u23ce": {"f1-score": 0.9633300297324083, "precision": 0.9455252918287937, "recall": 0.9818181818181818, "support": 495}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9852744310575636, "precision": 0.9787234042553191, "recall": 0.9919137466307277, "support": 371}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u2423": {"f1-score": 0.954370533260033, "precision": 0.9601769911504425, "recall": 0.9486338797814208, "support": 1830}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 697}, "macro avg": {"f1-score": 0.4294625972821832, "precision": 0.4823658947879415, "recall": 0.3902368572647721, "support": 9171}, "micro avg": {"f1-score": 0.7463826366559485, "precision": 0.9676915059927045, "recall": 0.6074582924435721, "support": 9171}, "weighted avg": {"f1-score": 0.7013933078303303, "precision": 0.8336093167110057, "recall": 0.6074582924435721, "support": 9171}, "\u2205": {"f1-score": 0.7958883994126286, "precision": 0.9745014710689768, "recall": 0.6726083032490975, "support": 4432}, "\u23ce": {"f1-score": 0.8586572438162544, "precision": 0.9455252918287937, "recall": 0.7864077669902912, "support": 618}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.968421052631579, "precision": 0.9787234042553191, "recall": 0.9583333333333334, "support": 384}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 389}, "\u2423": {"f1-score": 0.8127340823970038, "precision": 0.9601769911504425, "recall": 0.7045454545454546, "support": 2464}},
  "ppcr": 0.6277396140006543
}
```
</details>
