# Train report for javascript / file:///tmp/top-repos-quality-repos-bndkwo4g/sights.git HEAD e9269a3f5c392e33160b1d036c3d979089a9b31d

### Classification report

PPCR: 0.732

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.939| 0.998| 0.956| 0.967| 0.947| 8888| 9276| 0.958 |
| `"` | 1.000| 1.000| 0.798| 1.000| 0.888| 4429| 5550| 0.798 |
| `␣` | 0.982| 0.806| 0.383| 0.885| 0.551| 2469| 5196| 0.475 |
| `⏎` | 0.887| 0.971| 0.696| 0.927| 0.780| 1420| 1982| 0.716 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.974| 0.769| 0.470| 0.859| 0.634| 432| 707| 0.611 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.971| 0.766| 0.441| 0.856| 0.607| 432| 750| 0.576 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 315| 0.111 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 194| 0.134 |
| `'` | 1.000| 1.000| 0.018| 1.000| 0.036| 12| 653| 0.018 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 164| 0.024 |
| `micro avg` | 0.956| 0.956| 0.700| 0.956| 0.808| 18147| 24787| 0.732 |
| `macro avg` | 0.675| 0.631| 0.376| 0.650| 0.444| 18147| 24787| 0.732 |
| `weighted avg` | 0.954| 0.956| 0.700| 0.952| 0.769| 18147| 24787| 0.732 |

### Confusion matrix

|refusal|  "| ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1121 |4429 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|388 |0 |8868 |12 |0 |0 |0 |8 |0 |0 |0 |
|2727 |0 |321 |1989 |150 |0 |7 |2 |0 |0 |0 |
|562 |0 |36 |5 |1379 |0 |0 |0 |0 |0 |0 |
|641 |0 |0 |0 |0 |12 |0 |0 |0 |0 |0 |
|275 |0 |84 |13 |3 |0 |332 |0 |0 |0 |0 |
|318 |0 |87 |0 |14 |0 |0 |331 |0 |0 |0 |
|280 |0 |34 |0 |1 |0 |0 |0 |0 |0 |0 |
|168 |0 |10 |6 |8 |0 |2 |0 |0 |0 |0 |
|160 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| interface/js/gamepad.js | 286 |
| interface/js/sights.supervisor.js | 134 |
| interface/js/sights.sensors.js | 66 |
| interface/js/sights.interface.js | 59 |
| interface/js/sights.control.js | 55 |
| interface/js/graphs/thermalcamera.js | 54 |
| interface/js/sights.config.schema.js | 40 |
| interface/js/graphs/linegraph.js | 40 |
| interface/js/sights.util.js | 18 |
| interface/js/graphs/circlegraph.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4429}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 12}, "macro avg": {"f1-score": 0.649512217509481, "precision": 0.6752329361942356, "recall": 0.6309188065175009, "support": 18147}, "micro avg": {"f1-score": 0.9555298396429162, "precision": 0.9555298396429162, "recall": 0.9555298396429162, "support": 18147}, "weighted avg": {"f1-score": 0.9523900468473859, "precision": 0.9539442928769567, "recall": 0.9555298396429162, "support": 18147}, "\u2205": {"f1-score": 0.967488544621427, "precision": 0.9390088945362135, "recall": 0.9977497749774977, "support": 8888}, "\u23ce": {"f1-score": 0.9270588235294117, "precision": 0.8868167202572347, "recall": 0.9711267605633803, "support": 1420}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8589909443725743, "precision": 0.9736070381231672, "recall": 0.7685185185185185, "support": 432}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8564036222509702, "precision": 0.9706744868035191, "recall": 0.7662037037037037, "support": 432}, "\u2423": {"f1-score": 0.8851802403204272, "precision": 0.9822222222222222, "recall": 0.8055893074119077, "support": 2469}},
  "cl_report_full": {"\"": {"f1-score": 0.8876640945986571, "precision": 1.0, "recall": 0.798018018018018, "support": 5550}, "\u0027": {"f1-score": 0.03609022556390978, "precision": 1.0, "recall": 0.018376722817764167, "support": 653}, "macro avg": {"f1-score": 0.44422108561381507, "precision": 0.6752329361942356, "recall": 0.3761889728211535, "support": 24787}, "micro avg": {"f1-score": 0.8077514324311734, "precision": 0.9555298396429162, "recall": 0.6995602533586154, "support": 24787}, "weighted avg": {"f1-score": 0.7685266861579791, "precision": 0.9356069694997435, "recall": 0.6995602533586154, "support": 24787}, "\u2205": {"f1-score": 0.9474358974358974, "precision": 0.9390088945362135, "recall": 0.9560155239327296, "support": 9276}, "\u23ce": {"f1-score": 0.7797568560927339, "precision": 0.8868167202572347, "recall": 0.6957618567103936, "support": 1982}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 315}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.633587786259542, "precision": 0.9736070381231672, "recall": 0.4695898161244696, "support": 707}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.6067827681026581, "precision": 0.9706744868035191, "recall": 0.44133333333333336, "support": 750}, "\u2423": {"f1-score": 0.5508932280847528, "precision": 0.9822222222222222, "recall": 0.3827944572748268, "support": 5196}},
  "ppcr": 0.7321176423125025
}
```
</details>
