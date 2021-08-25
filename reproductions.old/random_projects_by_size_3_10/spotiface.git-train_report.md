# Train report for javascript / file:///tmp/top-repos-quality-repos-79gw624q/spotiface.git HEAD d358f8517c32e29edef111ca9121f9cecd7615c5

### Classification report

PPCR: 0.641

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 1.000| 0.897| 0.970| 0.919| 6894| 7687| 0.897 |
| `⏎` | 0.948| 0.879| 0.555| 0.912| 0.700| 727| 1152| 0.631 |
| `␣` | 0.951| 0.630| 0.113| 0.758| 0.202| 459| 2560| 0.179 |
| `"` | 1.000| 1.000| 0.822| 1.000| 0.902| 388| 472| 0.822 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 96| 302| 0.318 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 63| 333| 0.189 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 161| 0.348 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 888| 0.000 |
| `weighted avg` | 0.922| 0.946| 0.606| 0.931| 0.650| 8683| 13555| 0.641 |
| `micro avg` | 0.946| 0.946| 0.606| 0.946| 0.738| 8683| 13555| 0.641 |
| `macro avg` | 0.480| 0.439| 0.298| 0.455| 0.340| 8683| 13555| 0.641 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|793 |6894 |0 |0 |0 |0 |0 |0 |0 |
|2101 |159 |289 |11 |0 |0 |0 |0 |0 |
|425 |88 |0 |639 |0 |0 |0 |0 |0 |
|888 |0 |0 |0 |0 |0 |0 |0 |0 |
|84 |0 |0 |0 |0 |388 |0 |0 |0 |
|270 |45 |15 |3 |0 |0 |0 |0 |0 |
|206 |86 |0 |10 |0 |0 |0 |0 |0 |
|105 |45 |0 |11 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| spotify_webapp/client/src/components/Preference.js | 85 |
| spotify_webapp/client/src/components/Container3.js | 70 |
| spotify_webapp/client/src/components/Container1.js | 46 |
| spotify_webapp/client/src/registerServiceWorker.js | 40 |
| spotify_webapp/client/src/components/Player.js | 38 |
| spotify_webapp/client/config/webpack.config.prod.js | 29 |
| spotify_webapp/client/scripts/build.js | 28 |
| spotify_webapp/auth-server/authorization_code/server.js | 28 |
| spotify_webapp/client/src/App.js | 22 |
| spotify_webapp/client/config/webpack.config.dev.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 388}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4549969918642518, "precision": 0.48011481665603395, "recall": 0.4385730297009527, "support": 8683}, "micro avg": {"f1-score": 0.9455257399516296, "precision": 0.9455257399516296, "recall": 0.9455257399516296, "support": 8683}, "weighted avg": {"f1-score": 0.9314382046458362, "precision": 0.9223832342167072, "recall": 0.9455257399516296, "support": 8683}, "\u2205": {"f1-score": 0.9702343255224826, "precision": 0.942189421894219, "recall": 1.0, "support": 6894}, "\u23ce": {"f1-score": 0.9122055674518201, "precision": 0.9480712166172107, "recall": 0.8789546079779917, "support": 727}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u2423": {"f1-score": 0.7575360419397117, "precision": 0.9506578947368421, "recall": 0.6296296296296297, "support": 459}},
  "cl_report_full": {"\"": {"f1-score": 0.9023255813953489, "precision": 1.0, "recall": 0.8220338983050848, "support": 472}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 888}, "macro avg": {"f1-score": 0.34037333002202064, "precision": 0.48011481665603395, "recall": 0.2983063552612558, "support": 13555}, "micro avg": {"f1-score": 0.7383757532152171, "precision": 0.9455257399516296, "recall": 0.6056805606787163, "support": 13555}, "weighted avg": {"f1-score": 0.6501531690567086, "precision": 0.8292491581091997, "recall": 0.6056805606787163, "support": 13555}, "\u2205": {"f1-score": 0.9189549453479073, "precision": 0.942189421894219, "recall": 0.8968388187849616, "support": 7687}, "\u23ce": {"f1-score": 0.6998904709748083, "precision": 0.9480712166172107, "recall": 0.5546875, "support": 1152}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 161}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 333}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 302}, "\u2423": {"f1-score": 0.20181564245810055, "precision": 0.9506578947368421, "recall": 0.112890625, "support": 2560}},
  "ppcr": 0.6405754334194025
}
```
</details>
