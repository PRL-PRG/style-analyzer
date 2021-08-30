# Train report for javascript / file:///tmp/top-repos-quality-repos-vvda93vz/parinfer.git HEAD 41c74d03534a5adbdcb7430fb666899e8dbf746d

### Classification report

PPCR: 0.801

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.986| 0.957| 0.986| 0.971| 13199| 13595| 0.971 |
| `␣` | 0.955| 0.988| 0.800| 0.971| 0.871| 7131| 8803| 0.810 |
| `"` | 1.000| 1.000| 0.666| 1.000| 0.800| 1018| 1528| 0.666 |
| `⏎` | 0.944| 0.908| 0.440| 0.926| 0.600| 994| 2051| 0.485 |
| `⏎␣⁻␣⁻` | 0.927| 0.588| 0.182| 0.719| 0.305| 194| 625| 0.310 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 108| 0.407 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 597| 0.060 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 31| 148| 0.209 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 587| 0.046 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 257| 0.000 |
| `weighted avg` | 0.968| 0.974| 0.780| 0.971| 0.831| 22674| 28299| 0.801 |
| `macro avg` | 0.481| 0.447| 0.305| 0.460| 0.355| 22674| 28299| 0.801 |
| `micro avg` | 0.974| 0.974| 0.780| 0.974| 0.867| 22674| 28299| 0.801 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|396 |13008 |191 |0 |0 |0 |0 |0 |0 |0 |0 |
|1672 |80 |7044 |7 |0 |0 |0 |0 |0 |0 |0 |
|1057 |7 |84 |903 |0 |0 |0 |0 |0 |0 |0 |
|510 |0 |0 |0 |1018 |0 |0 |0 |0 |0 |0 |
|431 |47 |27 |6 |0 |114 |0 |0 |0 |0 |0 |
|560 |20 |0 |7 |0 |0 |0 |0 |0 |0 |0 |
|561 |0 |4 |32 |0 |0 |0 |0 |0 |0 |0 |
|257 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|117 |2 |27 |2 |0 |0 |0 |0 |0 |0 |0 |
|64 |35 |0 |0 |0 |9 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| site/resources/public/js/lib/gears.d3.js | 138 |
| lib/parinfer.js | 113 |
| site/resources/public/js/lib/scrollMonitor.js | 97 |
| site/resources/public/parinfer.js | 82 |
| site/resources/public/parinfer-v1.8.2.js | 65 |
| site/resources/public/js/lib/gears.d3.externs.js | 34 |
| lib/test/cases.js | 19 |
| site/resources/public/js/lib/d3.ext.js | 14 |
| site/resources/public/js/lib/scrollMonitor.externs.js | 13 |
| lib/test/cases/build.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1018}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4601501702790839, "precision": 0.48107918418270296, "recall": 0.44694085245438053, "support": 22674}, "micro avg": {"f1-score": 0.9741113169268766, "precision": 0.9741113169268766, "recall": 0.9741113169268766, "support": 22674}, "weighted avg": {"f1-score": 0.9707250146045145, "precision": 0.9681935969738115, "recall": 0.9741113169268766, "support": 22674}, "\u2205": {"f1-score": 0.9855292067580878, "precision": 0.9855292067580878, "recall": 0.9855292067580878, "support": 13199}, "\u23ce": {"f1-score": 0.9256791389031266, "precision": 0.9435736677115988, "recall": 0.9084507042253521, "support": 994}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.719242902208202, "precision": 0.926829268292683, "recall": 0.5876288659793815, "support": 194}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u2423": {"f1-score": 0.9710504549214227, "precision": 0.9548596990646604, "recall": 0.9877997475809844, "support": 7131}},
  "cl_report_full": {"\"": {"f1-score": 0.7996857816182248, "precision": 1.0, "recall": 0.6662303664921466, "support": 1528}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 257}, "macro avg": {"f1-score": 0.3546565774138666, "precision": 0.48107918418270296, "recall": 0.3045907521416471, "support": 28299}, "micro avg": {"f1-score": 0.8666156592705943, "precision": 0.9741113169268766, "recall": 0.7804869430015194, "support": 28299}, "weighted avg": {"f1-score": 0.8307328625444719, "precision": 0.9133339475565151, "recall": 0.7804869430015194, "support": 28299}, "\u2205": {"f1-score": 0.97096364857804, "precision": 0.9855292067580878, "recall": 0.956822361162192, "support": 13595}, "\u23ce": {"f1-score": 0.6003989361702128, "precision": 0.9435736677115988, "recall": 0.4402730375426621, "support": 2051}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 597}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 587}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.3048128342245989, "precision": 0.926829268292683, "recall": 0.1824, "support": 625}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u2423": {"f1-score": 0.8707045735475895, "precision": 0.9548596990646604, "recall": 0.8001817562194706, "support": 8803}},
  "ppcr": 0.8012297254319941
}
```
</details>
