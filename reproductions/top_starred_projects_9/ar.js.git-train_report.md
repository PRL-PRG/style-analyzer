# Train report for javascript / file:///tmp/top-repos-quality-repos-pp738myk/ar.js.git HEAD aaad9847f67a4738b00724c38d9f501d78e0a8af

### Classification report

PPCR: 0.662

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.966| 0.993| 0.898| 0.979| 0.931| 21384| 23643| 0.904 |
| `␣` | 0.960| 0.888| 0.424| 0.923| 0.589| 5348| 11186| 0.478 |
| `'` | 0.982| 1.000| 0.850| 0.991| 0.911| 1913| 2251| 0.850 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 105| 3181| 0.033 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 61| 1123| 0.054 |
| `"` | 1.000| 0.250| 0.207| 0.400| 0.343| 48| 58| 0.828 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 31| 741| 0.042 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 729| 0.005 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 231| 0.009 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 232| 0.004 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 138| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 140| 0.000 |
| `weighted avg` | 0.959| 0.966| 0.639| 0.962| 0.702| 28897| 43653| 0.662 |
| `micro avg` | 0.966| 0.966| 0.639| 0.966| 0.769| 28897| 43653| 0.662 |
| `macro avg` | 0.326| 0.261| 0.198| 0.274| 0.231| 28897| 43653| 0.662 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⇥⁻| ⏎⇥⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2259 |21239 |145 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5838 |600 |4748 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3076 |59 |46 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|338 |0 |0 |0 |1913 |0 |0 |0 |0 |0 |0 |0 |0 |
|1062 |59 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|710 |31 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|725 |2 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|229 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|231 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|138 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|140 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|10 |0 |0 |0 |36 |0 |0 |0 |0 |0 |0 |0 |12 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| aframe/build/aframe-ar-location-only.js | 164 |
| three.js/src/threex/threex-armarkercloak.js | 111 |
| three.js/src/threex/threex-armarkercontrols.js | 56 |
| three.js/src/markers-area/threex-armultimarkerutils.js | 56 |
| aframe/src/location-based/arjs-look-controls.js | 47 |
| three.js/src/threex/threex-artoolkitsource.js | 45 |
| three.js/src/markers-area/threex-armultimarkercontrols.js | 44 |
| aframe/src/location-based/ArjsDeviceOrientationControls.js | 44 |
| three.js/src/markers-area/threex-armultimarkerlearning.js | 40 |
| three.js/src/threex/threex-arsmoothedcontrols.js | 36 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.4, "precision": 1.0, "recall": 0.25, "support": 48}, "\u0027": {"f1-score": 0.9906784049715174, "precision": 0.9815289892252437, "recall": 1.0, "support": 1913}, "macro avg": {"f1-score": 0.27438617397295345, "precision": 0.325637104395369, "recall": 0.26091897965686023, "support": 28897}, "micro avg": {"f1-score": 0.965913416617642, "precision": 0.965913416617642, "recall": 0.965913416617642, "support": 28897}, "weighted avg": {"f1-score": 0.9616928538997046, "precision": 0.959043033085369, "recall": 0.965913416617642, "support": 28897}, "\u2205": {"f1-score": 0.9792973072666913, "precision": 0.9657602764641688, "recall": 0.9932192293303405, "support": 21384}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u2423": {"f1-score": 0.9226583754372328, "precision": 0.9603559870550162, "recall": 0.8878085265519821, "support": 5348}},
  "cl_report_full": {"\"": {"f1-score": 0.34285714285714286, "precision": 1.0, "recall": 0.20689655172413793, "support": 58}, "\u0027": {"f1-score": 0.910952380952381, "precision": 0.9815289892252437, "recall": 0.8498445135495335, "support": 2251}, "macro avg": {"f1-score": 0.2311122369049968, "precision": 0.325637104395369, "recall": 0.1982934222251065, "support": 43653}, "micro avg": {"f1-score": 0.7694555478980013, "precision": 0.965913416617642, "recall": 0.639406226376194, "support": 43653}, "weighted avg": {"f1-score": 0.702430914547402, "precision": 0.8210989861495378, "recall": 0.639406226376194, "support": 43653}, "\u2205": {"f1-score": 0.9308206420510574, "precision": 0.9657602764641688, "recall": 0.8983208560673349, "support": 23643}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3181}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 232}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 231}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1123}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 138}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 729}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 741}, "\u2423": {"f1-score": 0.5887166769993801, "precision": 0.9603559870550162, "recall": 0.4244591453602718, "support": 11186}},
  "ppcr": 0.6619705403981399
}
```
</details>
