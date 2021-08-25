# Train report for javascript / file:///tmp/top-repos-quality-repos-2koz_6hz/scratch.git HEAD 3d314ca9c54b0c67bd654ca65c7def1029cacd7e

### Classification report

PPCR: 0.966

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 63471| 63477| 1.000 |
| `∅` | 0.952| 0.991| 0.977| 0.971| 0.964| 26772| 27150| 0.986 |
| `␣` | 0.983| 0.954| 0.907| 0.969| 0.944| 25541| 26878| 0.950 |
| `⏎` | 0.985| 0.988| 0.935| 0.987| 0.959| 16679| 17630| 0.946 |
| `⏎␣⁺␣⁺` | 0.999| 0.979| 0.892| 0.989| 0.942| 6360| 6983| 0.911 |
| `⏎␣⁻␣⁻` | 0.996| 0.983| 0.876| 0.990| 0.932| 6207| 6964| 0.891 |
| `⏎␣⁺␣⁺␣⁺` | 0.992| 0.994| 0.950| 0.993| 0.970| 4098| 4289| 0.955 |
| `⏎␣⁻␣⁻␣⁻` | 0.988| 0.990| 0.867| 0.989| 0.924| 3769| 4302| 0.876 |
| `⏎⏎` | 0.951| 0.492| 0.268| 0.648| 0.418| 433| 795| 0.545 |
| `'` | 1.000| 1.000| 0.061| 1.000| 0.115| 12| 197| 0.061 |
| `weighted avg` | 0.986| 0.986| 0.953| 0.986| 0.967| 153342| 158665| 0.966 |
| `micro avg` | 0.986| 0.986| 0.953| 0.986| 0.969| 153342| 158665| 0.966 |
| `macro avg` | 0.985| 0.937| 0.773| 0.954| 0.817| 153342| 158665| 0.966 |

### Confusion matrix

|refusal|  "| ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻| ⏎⏎| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6 |63471 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|378 |0 |26530 |224 |12 |0 |0 |2 |4 |0 |0 |
|1337 |0 |972 |24376 |179 |3 |6 |2 |1 |2 |0 |
|951 |0 |95 |88 |16480 |0 |1 |2 |9 |4 |0 |
|623 |0 |23 |68 |0 |6228 |2 |28 |6 |5 |0 |
|757 |0 |81 |1 |1 |0 |6103 |0 |21 |0 |0 |
|191 |0 |0 |25 |0 |0 |0 |4073 |0 |0 |0 |
|533 |0 |24 |0 |0 |0 |14 |0 |3731 |0 |0 |
|362 |0 |146 |11 |56 |2 |0 |0 |5 |213 |0 |
|185 |0 |0 |0 |0 |0 |0 |0 |0 |0 |12 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| color_detect/tracking/build/tracking.js | 764 |
| color_detect/tracking/test/ColorTracker.js | 114 |
| color_detect/tracking/src/detection/ViolaJones.js | 96 |
| color_detect/tracking/src/trackers/ColorTracker.js | 93 |
| color_detect/tracking/src/utils/Image.js | 87 |
| color_detect/tracking/src/alignment/Regressor.js | 82 |
| color_detect/tracking/src/alignment/LBF.js | 76 |
| color_detect/tracking/test/utils/benchmark.js | 70 |
| color_detect/tracking/assets/opencv_haarcascade_mouth.js | 70 |
| color_detect/tracking/gulpfile.js | 69 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 63471}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 12}, "macro avg": {"f1-score": 0.9535122392868205, "precision": 0.984611942578192, "recall": 0.9371640677276718, "support": 153342}, "micro avg": {"f1-score": 0.9861420876211344, "precision": 0.9861420876211344, "recall": 0.9861420876211344, "support": 153342}, "weighted avg": {"f1-score": 0.9859250493946691, "precision": 0.9863411593604204, "recall": 0.9861420876211344, "support": 153342}, "\u2205": {"f1-score": 0.9710301410976704, "precision": 0.9518854723547774, "recall": 0.9909607052144032, "support": 26772}, "\u23ce": {"f1-score": 0.9866195707486456, "precision": 0.9851745576279293, "recall": 0.9880688290664909, "support": 16679}, "\u23ce\u23ce": {"f1-score": 0.6484018264840183, "precision": 0.9508928571428571, "recall": 0.49191685912240185, "support": 433}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9891209402048757, "precision": 0.9991978180651372, "recall": 0.9792452830188679, "support": 6360}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9928092626447288, "precision": 0.9917214511809106, "recall": 0.9938994631527575, "support": 4098}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9897024243898485, "precision": 0.9962455109369899, "recall": 0.9832447236990495, "support": 6207}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9888682745825603, "precision": 0.9878210219751126, "recall": 0.9899177500663305, "support": 3769}, "\u2423": {"f1-score": 0.968569952715858, "precision": 0.9831807364982051, "recall": 0.9543870639364159, "support": 25541}},
  "cl_report_full": {"\"": {"f1-score": 0.9999527365535495, "precision": 1.0, "recall": 0.9999054775745546, "support": 63477}, "\u0027": {"f1-score": 0.11483253588516747, "precision": 1.0, "recall": 0.06091370558375635, "support": 197}, "macro avg": {"f1-score": 0.8168834561537366, "precision": 0.984611942578192, "recall": 0.7732744696688291, "support": 158665}, "micro avg": {"f1-score": 0.969317996070601, "precision": 0.9861420876211344, "recall": 0.9530583304446475, "support": 158665}, "weighted avg": {"f1-score": 0.9674059436571815, "precision": 0.9862701990029674, "recall": 0.9530583304446475, "support": 158665}, "\u2205": {"f1-score": 0.964359062903255, "precision": 0.9518854723547774, "recall": 0.9771639042357274, "support": 27150}, "\u23ce": {"f1-score": 0.9593107864252868, "precision": 0.9851745576279293, "recall": 0.9347702779353375, "support": 17630}, "\u23ce\u23ce": {"f1-score": 0.4180569185475957, "precision": 0.9508928571428571, "recall": 0.2679245283018868, "support": 795}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9424939467312349, "precision": 0.9991978180651372, "recall": 0.8918802806816555, "support": 6983}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9702239161505479, "precision": 0.9917214511809106, "recall": 0.9496386103986943, "support": 4289}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9324675324675324, "precision": 0.9962455109369899, "recall": 0.8763641585295807, "support": 6964}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9236291620250031, "precision": 0.9878210219751126, "recall": 0.8672710367271037, "support": 4302}, "\u2423": {"f1-score": 0.9435079638481934, "precision": 0.9831807364982051, "recall": 0.9069127167199941, "support": 26878}},
  "ppcr": 0.9664513282702549
}
```
</details>
