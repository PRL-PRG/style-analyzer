# Train report for javascript / file:///tmp/top-repos-quality-repos-396j0jvo/pose-animator.git HEAD cb2be70a3501e57e66fe185daca44cc2afe18ce8

### Classification report

PPCR: 0.539

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 1.000| 0.848| 0.992| 0.911| 6879| 8108| 0.848 |
| `␣` | 1.000| 0.870| 0.160| 0.931| 0.276| 772| 4196| 0.184 |
| `'` | 1.000| 1.000| 0.494| 1.000| 0.661| 469| 950| 0.494 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 213| 0.038 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 1040| 0.006 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 229| 0.009 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 192| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 84| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 81| 0.000 |
| `weighted avg` | 0.984| 0.986| 0.531| 0.984| 0.608| 8136| 15093| 0.539 |
| `micro avg` | 0.986| 0.986| 0.531| 0.986| 0.691| 8136| 15093| 0.539 |
| `macro avg` | 0.331| 0.319| 0.167| 0.325| 0.205| 8136| 15093| 0.539 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1229 |6879 |0 |0 |0 |0 |0 |0 |0 |0 |
|3424 |100 |672 |0 |0 |0 |0 |0 |0 |0 |
|1034 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|481 |0 |0 |0 |469 |0 |0 |0 |0 |0 |
|192 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|227 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|205 |8 |0 |0 |0 |0 |0 |0 |0 |0 |
|84 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|81 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| illustrationGen/skeleton.js | 43 |
| illustrationGen/illustration.js | 26 |
| utils/mathUtils.js | 16 |
| static_image.js | 6 |
| camera.js | 6 |
| utils/colorUtils.js | 6 |
| utils/demoUtils.js | 5 |
| utils/svgUtils.js | 5 |
| utils/fileUtils.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 469}, "macro avg": {"f1-score": 0.32470966216504316, "precision": 0.33149074735922485, "recall": 0.31894070236039146, "support": 8136}, "micro avg": {"f1-score": 0.9857423795476893, "precision": 0.9857423795476893, "recall": 0.9857423795476893, "support": 8136}, "weighted avg": {"f1-score": 0.9843931086495964, "precision": 0.9840122492326658, "recall": 0.9857423795476893, "support": 8136}, "\u2205": {"f1-score": 0.9916390370477152, "precision": 0.9834167262330236, "recall": 1.0, "support": 6879}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.9307479224376731, "precision": 1.0, "recall": 0.8704663212435233, "support": 772}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6610288935870331, "precision": 1.0, "recall": 0.4936842105263158, "support": 950}, "macro avg": {"f1-score": 0.20534027575468633, "precision": 0.33149074735922485, "recall": 0.16691756100288033, "support": 15093}, "micro avg": {"f1-score": 0.6905161651384047, "precision": 0.9857423795476893, "recall": 0.5313721592791361, "support": 15093}, "weighted avg": {"f1-score": 0.6077245491420874, "precision": 0.8692468572382797, "recall": 0.5313721592791361, "support": 15093}, "\u2205": {"f1-score": 0.9109448453949546, "precision": 0.9834167262330236, "recall": 0.8484213122841638, "support": 8108}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1040}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 192}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 229}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 213}, "\u2423": {"f1-score": 0.276088742810189, "precision": 1.0, "recall": 0.1601525262154433, "support": 4196}},
  "ppcr": 0.5390578413834228
}
```
</details>
