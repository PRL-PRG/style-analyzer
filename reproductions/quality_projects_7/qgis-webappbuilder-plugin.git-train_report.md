# Train report for javascript / file:///tmp/top-repos-quality-repos-a8rzqgwl/qgis-webappbuilder-plugin.git HEAD ceed00caa091ca875feef624c1cf4a12bb23cda4

### Classification report

PPCR: 0.823

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.922| 0.995| 0.951| 0.957| 0.936| 14201| 14857| 0.956 |
| `␣` | 0.964| 0.816| 0.532| 0.884| 0.686| 4334| 6652| 0.652 |
| `"` | 1.000| 1.000| 0.643| 1.000| 0.783| 653| 1016| 0.643 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 1.000| 0.985| 0.819| 0.992| 0.900| 394| 474| 0.831 |
| `⏎⏎` | 0.946| 0.997| 0.919| 0.971| 0.932| 387| 420| 0.921 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 295| 812| 0.363 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 190| 609| 0.312 |
| `weighted avg` | 0.913| 0.934| 0.769| 0.921| 0.809| 20454| 24840| 0.823 |
| `micro avg` | 0.934| 0.934| 0.769| 0.934| 0.843| 20454| 24840| 0.823 |
| `macro avg` | 0.690| 0.685| 0.552| 0.686| 0.605| 20454| 24840| 0.823 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|656 |14134 |65 |0 |0 |0 |2 |0 |
|2318 |796 |3538 |0 |0 |0 |0 |0 |
|363 |0 |0 |653 |0 |0 |0 |0 |
|80 |6 |0 |0 |388 |0 |0 |0 |
|419 |190 |0 |0 |0 |0 |0 |0 |
|33 |0 |1 |0 |0 |0 |386 |0 |
|517 |209 |66 |0 |0 |0 |20 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| webappbuilder/js/qgis2web_expressions.js | 242 |
| webappbuilder/tests/expected/symbologycluster.js | 217 |
| webappbuilder/tests/expected/symbologycluster-2.14.js | 199 |
| webappbuilder/tests/expected/symbologypoints-2.14.js | 149 |
| webappbuilder/tests/expected/symbologypoints.js | 147 |
| webappbuilder/tests/expected/symbologypolygons.js | 79 |
| webappbuilder/tests/expected/symbologysimplelabels.js | 62 |
| webappbuilder/tests/expected/nochartwidget.js | 58 |
| webappbuilder/tests/expected/layers_locallines.js | 56 |
| webappbuilder/tests/expected/symbologylines.js | 49 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 653}, "macro avg": {"f1-score": 0.6863605511778461, "precision": 0.6902562221062161, "recall": 0.6848293664263326, "support": 20454}, "micro avg": {"f1-score": 0.9337537889899287, "precision": 0.9337537889899287, "recall": 0.9337537889899287, "support": 20454}, "weighted avg": {"f1-score": 0.9212197949927952, "precision": 0.9132718389180228, "recall": 0.9337537889899287, "support": 20454}, "\u2205": {"f1-score": 0.9570693391115928, "precision": 0.9216824258232801, "recall": 0.9952820223927893, "support": 14201}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 295}, "\u23ce\u23ce": {"f1-score": 0.9710691823899371, "precision": 0.946078431372549, "recall": 0.9974160206718347, "support": 387}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9923273657289001, "precision": 1.0, "recall": 0.9847715736040609, "support": 394}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 190}, "\u2423": {"f1-score": 0.8840579710144928, "precision": 0.964032697547684, "recall": 0.8163359483156437, "support": 4334}},
  "cl_report_full": {"\"": {"f1-score": 0.7825044937088076, "precision": 1.0, "recall": 0.6427165354330708, "support": 1016}, "macro avg": {"f1-score": 0.6052720332390844, "precision": 0.6902562221062161, "recall": 0.551933677159295, "support": 24840}, "micro avg": {"f1-score": 0.8433346580120987, "precision": 0.9337537889899287, "recall": 0.7688808373590982, "support": 24840}, "weighted avg": {"f1-score": 0.8085216785094816, "precision": 0.8854079809066078, "recall": 0.7688808373590982, "support": 24840}, "\u2205": {"f1-score": 0.9362745098039216, "precision": 0.9216824258232801, "recall": 0.9513360705391398, "support": 14857}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 812}, "\u23ce\u23ce": {"f1-score": 0.932367149758454, "precision": 0.946078431372549, "recall": 0.919047619047619, "support": 420}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9002320185614849, "precision": 1.0, "recall": 0.8185654008438819, "support": 474}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 609}, "\u2423": {"f1-score": 0.6855260608409224, "precision": 0.964032697547684, "recall": 0.531870114251353, "support": 6652}},
  "ppcr": 0.8234299516908212
}
```
</details>
