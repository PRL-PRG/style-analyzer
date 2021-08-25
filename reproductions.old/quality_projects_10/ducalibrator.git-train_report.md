# Train report for javascript / file:///tmp/top-repos-quality-repos-4lpsneg4/ducalibrator.git HEAD 56a9eb2aef79ee473b8cc2e8157f5cd9967b92ad

### Classification report

PPCR: 0.702

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 0.982| 0.866| 0.962| 0.903| 6870| 7783| 0.883 |
| `␣` | 0.913| 0.812| 0.437| 0.859| 0.591| 1863| 3457| 0.539 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 169| 0.237 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 624| 0.038 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 203| 0.069 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 323| 0.000 |
| `macro avg` | 0.309| 0.299| 0.217| 0.303| 0.249| 8811| 12559| 0.702 |
| `micro avg` | 0.937| 0.937| 0.657| 0.937| 0.773| 8811| 12559| 0.702 |
| `weighted avg` | 0.928| 0.937| 0.657| 0.931| 0.722| 8811| 12559| 0.702 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|913 |6743 |127 |0 |0 |0 |0 |
|1594 |351 |1512 |0 |0 |0 |0 |
|600 |19 |5 |0 |0 |0 |0 |
|323 |0 |0 |0 |0 |0 |0 |
|189 |3 |11 |0 |0 |0 |0 |
|129 |39 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| octoprint_DuCalibrator/static/js/TrackballControls.js | 218 |
| octoprint_DuCalibrator/static/js/DuCalMachine.js | 113 |
| octoprint_DuCalibrator/static/js/DuCalGeometry.js | 109 |
| octoprint_DuCalibrator/static/js/DuCalViewModel.js | 72 |
| octoprint_DuCalibrator/static/js/DuCalCommon.js | 23 |
| octoprint_DuCalibrator/static/js/trilateration.js | 21 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.30348394430235864, "precision": 0.30924356130809505, "recall": 0.2988513385228783, "support": 8811}, "micro avg": {"f1-score": 0.9368970604925662, "precision": 0.9368970604925662, "recall": 0.9368970604925662, "support": 8811}, "weighted avg": {"f1-score": 0.9314399781022384, "precision": 0.9278641359059704, "recall": 0.9368970604925662, "support": 8811}, "\u2205": {"f1-score": 0.9615686274509804, "precision": 0.9424178895877009, "recall": 0.981513828238719, "support": 6870}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u2423": {"f1-score": 0.8593350383631713, "precision": 0.9130434782608695, "recall": 0.8115942028985508, "support": 1863}},
  "cl_report_full": {"macro avg": {"f1-score": 0.24903863888683264, "precision": 0.30924356130809505, "recall": 0.21729147980351668, "support": 12559}, "micro avg": {"f1-score": 0.7725783809078147, "precision": 0.9368970604925662, "recall": 0.6572975555378613, "support": 12559}, "weighted avg": {"f1-score": 0.7222760253547145, "precision": 0.8353555011552594, "recall": 0.6572975555378613, "support": 12559}, "\u2205": {"f1-score": 0.90279823269514, "precision": 0.9424178895877009, "recall": 0.8663754336374149, "support": 7783}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 624}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 323}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 203}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 169}, "\u2423": {"f1-score": 0.5914336006258557, "precision": 0.9130434782608695, "recall": 0.43737344518368526, "support": 3457}},
  "ppcr": 0.7015685962258141
}
```
</details>
