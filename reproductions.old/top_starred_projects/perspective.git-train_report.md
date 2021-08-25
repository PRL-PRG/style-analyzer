# Train report for javascript / file:///tmp/top-repos-quality-repos-_b5lskm2/perspective.git HEAD ef1690e7a9474ccd93243fd80ca48010478166fb

### Classification report

PPCR: 0.939

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.986| 0.978| 0.984| 0.979| 144988| 146250| 0.991 |
| `␣` | 0.957| 0.982| 0.956| 0.969| 0.956| 67298| 69159| 0.973 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 25228| 25228| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.966| 0.911| 0.877| 0.938| 0.920| 7363| 7647| 0.963 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.885| 0.899| 0.789| 0.892| 0.834| 6850| 7807| 0.877 |
| `⏎` | 0.921| 0.674| 0.283| 0.778| 0.433| 6665| 15870| 0.420 |
| `⏎⏎` | 0.907| 0.690| 0.219| 0.784| 0.353| 1525| 4801| 0.318 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 133| 0.060 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 2| 1.000 |
| `weighted avg` | 0.972| 0.972| 0.913| 0.971| 0.927| 259927| 276897| 0.939 |
| `micro avg` | 0.972| 0.972| 0.913| 0.972| 0.941| 259927| 276897| 0.939 |
| `macro avg` | 0.735| 0.683| 0.567| 0.705| 0.608| 259927| 276897| 0.939 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| '| ⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1262 |142969 |988 |0 |0 |797 |234 |0 |0 |0 |
|1861 |930 |66101 |0 |267 |0 |0 |0 |0 |0 |
|0 |0 |0 |25228 |0 |0 |0 |0 |0 |0 |
|9205 |656 |1417 |0 |4489 |0 |0 |103 |0 |0 |
|957 |483 |207 |0 |0 |6160 |0 |0 |0 |0 |
|284 |621 |32 |0 |0 |0 |6709 |1 |0 |0 |
|3276 |36 |319 |0 |117 |0 |0 |1053 |0 |0 |
|0 |0 |0 |2 |0 |0 |0 |0 |0 |0 |
|125 |0 |2 |0 |1 |0 |1 |4 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/perspective/test/js/pivots.js | 270 |
| packages/perspective/src/js/perspective.js | 177 |
| docs/pages/en/index.js | 174 |
| packages/perspective-viewer/src/js/polyfill.js | 154 |
| packages/perspective/test/js/ports.js | 152 |
| packages/perspective/test/js/updates.js | 150 |
| packages/perspective-viewer-hypergrid/src/js/perspective-plugin.js | 147 |
| packages/perspective/test/js/computed/updates.js | 137 |
| packages/perspective-workspace/src/js/workspace/workspace.js | 131 |
| packages/perspective-test/src/js/index.js | 129 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.99996036307424, "precision": 0.9999207292905271, "recall": 1.0, "support": 25228}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "macro avg": {"f1-score": 0.7050446252831679, "precision": 0.7353181778785629, "recall": 0.6825273237111185, "support": 259927}, "micro avg": {"f1-score": 0.9722306647635682, "precision": 0.9722306647635682, "recall": 0.9722306647635682, "support": 259927}, "weighted avg": {"f1-score": 0.9713936810166931, "precision": 0.971853151430786, "recall": 0.9722306647635682, "support": 259927}, "\u2205": {"f1-score": 0.9836763759834597, "precision": 0.9812896804969286, "recall": 0.9860747096311419, "support": 144988}, "\u23ce": {"f1-score": 0.7780570240055464, "precision": 0.9210094378334017, "recall": 0.6735183795948987, "support": 6665}, "\u23ce\u23ce": {"f1-score": 0.7840655249441548, "precision": 0.9069767441860465, "recall": 0.6904918032786885, "support": 1525}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8923010067357138, "precision": 0.8854391260600833, "recall": 0.8992700729927007, "support": 6850}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9378625847487244, "precision": 0.9661578341013825, "recall": 0.9111775091674589, "support": 7363}, "\u2423": {"f1-score": 0.9694787480566718, "precision": 0.9570700489386963, "recall": 0.9822134387351779, "support": 67298}},
  "cl_report_full": {"\"": {"f1-score": 0.99996036307424, "precision": 0.9999207292905271, "recall": 1.0, "support": 25228}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "macro avg": {"f1-score": 0.6084353204317363, "precision": 0.7353181778785629, "recall": 0.5668791012791666, "support": 276897}, "micro avg": {"f1-score": 0.9414966543969719, "precision": 0.9722306647635682, "recall": 0.9126462186300321, "support": 276897}, "weighted avg": {"f1-score": 0.9271482822556848, "precision": 0.9685957983927098, "recall": 0.9126462186300321, "support": 276897}, "\u2205": {"f1-score": 0.9794242066142594, "precision": 0.9812896804969286, "recall": 0.9775658119658119, "support": 146250}, "\u23ce": {"f1-score": 0.43279984573852676, "precision": 0.9210094378334017, "recall": 0.28286074354127283, "support": 15870}, "\u23ce\u23ce": {"f1-score": 0.35323716873532374, "precision": 0.9069767441860465, "recall": 0.21932930639450116, "support": 4801}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 133}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8344622053644, "precision": 0.8854391260600833, "recall": 0.789035480978609, "support": 7807}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9196079775203894, "precision": 0.9661578341013825, "recall": 0.8773375179809075, "support": 7647}, "\u2423": {"f1-score": 0.956426116838488, "precision": 0.9570700489386963, "recall": 0.9557830506513975, "support": 69159}},
  "ppcr": 0.9387136733153483
}
```
</details>
