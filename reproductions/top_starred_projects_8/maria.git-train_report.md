# Train report for javascript / file:///tmp/top-repos-quality-repos-ih5dwxey/maria.git HEAD a13f0367247bb850da38f11bbc7906302e939954

### Classification report

PPCR: 0.842

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.974| 0.961| 0.984| 0.977| 30824| 31240| 0.987 |
| `␣` | 0.946| 0.984| 0.906| 0.965| 0.926| 15911| 17281| 0.921 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.963| 0.965| 0.947| 0.964| 0.955| 1985| 2024| 0.981 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.937| 0.959| 0.889| 0.948| 0.912| 1979| 2136| 0.926 |
| `'` | 0.949| 1.000| 0.641| 0.974| 0.765| 811| 1266| 0.641 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 79| 2887| 0.027 |
| `"` | 1.000| 0.389| 0.010| 0.560| 0.019| 72| 2945| 0.024 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 1592| 0.004 |
| `weighted avg` | 0.973| 0.974| 0.820| 0.974| 0.838| 51668| 61371| 0.842 |
| `macro avg` | 0.724| 0.659| 0.544| 0.674| 0.569| 51668| 61371| 0.842 |
| `micro avg` | 0.974| 0.974| 0.820| 0.974| 0.891| 51668| 61371| 0.842 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|416 |30035 |768 |0 |0 |19 |2 |0 |0 |
|1370 |111 |15656 |0 |0 |75 |69 |0 |0 |
|2873 |0 |0 |28 |0 |0 |0 |0 |44 |
|2808 |6 |40 |0 |0 |31 |2 |0 |0 |
|157 |9 |72 |0 |0 |1898 |0 |0 |0 |
|39 |57 |9 |0 |0 |3 |1916 |0 |0 |
|1585 |1 |6 |0 |0 |0 |0 |0 |0 |
|455 |0 |0 |0 |0 |0 |0 |0 |811 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/buster/buster-test.js | 625 |
| eg/timeit/src/js/views/AnalogueClockView.js | 85 |
| tst/SetModel-suite.js | 58 |
| eg/scrollit/lib/JSON/json2.js | 50 |
| lib/evento/evento.js | 47 |
| tst/View-suite.js | 40 |
| tst/ElementView.subclass-suite.js | 29 |
| tst/ElementView-suite.js | 27 |
| lib/hormigas/hormigas.js | 25 |
| lib/grail/grail.js | 22 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.56, "precision": 1.0, "recall": 0.3888888888888889, "support": 72}, "\u0027": {"f1-score": 0.9735894357743097, "precision": 0.9485380116959065, "recall": 1.0, "support": 811}, "macro avg": {"f1-score": 0.6742883483598456, "precision": 0.7235616634164075, "recall": 0.6589468544263475, "support": 51668}, "micro avg": {"f1-score": 0.9743748548424557, "precision": 0.9743748548424557, "recall": 0.9743748548424557, "support": 51668}, "weighted avg": {"f1-score": 0.9735177178657016, "precision": 0.9734130397619559, "recall": 0.9743748548424557, "support": 51668}, "\u2205": {"f1-score": 0.9840604164277641, "precision": 0.9939111155233462, "recall": 0.9744030625486634, "support": 30824}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9478152309612984, "precision": 0.9368213228035538, "recall": 0.9590702374936837, "support": 1979}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9642677403120283, "precision": 0.963298139768728, "recall": 0.9652392947103274, "support": 1985}, "\u2423": {"f1-score": 0.964573963403364, "precision": 0.9459247175397257, "recall": 0.9839733517692163, "support": 15911}},
  "cl_report_full": {"\"": {"f1-score": 0.018836192398250923, "precision": 1.0, "recall": 0.009507640067911714, "support": 2945}, "\u0027": {"f1-score": 0.7647336162187647, "precision": 0.9485380116959065, "recall": 0.6406003159557662, "support": 1266}, "macro avg": {"f1-score": 0.569180222096975, "precision": 0.7235616634164075, "recall": 0.5440898497538715, "support": 61371}, "micro avg": {"f1-score": 0.8907368253434655, "precision": 0.9743748548424557, "recall": 0.820322302064493, "support": 61371}, "weighted avg": {"f1-score": 0.838055019776867, "precision": 0.9042206122755153, "recall": 0.820322302064493, "support": 61371}, "\u2205": {"f1-score": 0.9773995671911355, "precision": 0.9939111155233462, "recall": 0.961427656850192, "support": 31240}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2887}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1592}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9120615088899567, "precision": 0.9368213228035538, "recall": 0.8885767790262172, "support": 2136}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9548965860951907, "precision": 0.963298139768728, "recall": 0.9466403162055336, "support": 2024}, "\u2423": {"f1-score": 0.9255143059825017, "precision": 0.9459247175397257, "recall": 0.9059660899253515, "support": 17281}},
  "ppcr": 0.8418960095158952
}
```
</details>
