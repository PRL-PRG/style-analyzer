# Train report for javascript / file:///tmp/top-repos-quality-repos-8_frcltu/node-open-mining-portal.git HEAD 8a673b6d7ce22d3ec1a3b5115378f0d4d72df581

### Classification report

PPCR: 0.628

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.998| 0.887| 0.990| 0.932| 11868| 13356| 0.889 |
| `'` | 1.000| 1.000| 0.888| 1.000| 0.941| 1573| 1772| 0.888 |
| `␣` | 0.997| 0.815| 0.129| 0.897| 0.228| 895| 5656| 0.158 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.946| 0.982| 0.757| 0.963| 0.841| 602| 781| 0.771 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 1062| 0.034 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 697| 0.023 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 419| 0.002 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 112| 0.009 |
| `micro avg` | 0.983| 0.983| 0.618| 0.983| 0.759| 14992| 23855| 0.628 |
| `weighted avg` | 0.980| 0.983| 0.618| 0.981| 0.673| 14992| 23855| 0.628 |
| `macro avg` | 0.491| 0.474| 0.333| 0.481| 0.368| 14992| 23855| 0.628 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1488 |11848 |2 |0 |0 |18 |0 |0 |0 |
|4761 |152 |729 |0 |0 |14 |0 |0 |0 |
|199 |0 |0 |1573 |0 |0 |0 |0 |0 |
|1026 |34 |0 |0 |0 |2 |0 |0 |0 |
|179 |11 |0 |0 |0 |591 |0 |0 |0 |
|681 |16 |0 |0 |0 |0 |0 |0 |0 |
|418 |1 |0 |0 |0 |0 |0 |0 |0 |
|111 |1 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| website/static/stats.js | 40 |
| libs/stats.js | 38 |
| libs/paymentProcessor.js | 28 |
| libs/mposCompatibility.js | 24 |
| website/static/admin.js | 21 |
| libs/poolWorker.js | 19 |
| libs/profitSwitch.js | 18 |
| init.js | 17 |
| libs/website.js | 16 |
| libs/workerapi.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1573}, "macro avg": {"f1-score": 0.481273031410977, "precision": 0.4906301158090114, "recall": 0.4743209388132453, "support": 14992}, "micro avg": {"f1-score": 0.9832577374599787, "precision": 0.9832577374599787, "recall": 0.9832577374599787, "support": 14992}, "weighted avg": {"f1-score": 0.9809836496670461, "precision": 0.9799411688364243, "recall": 0.9832577374599787, "support": 14992}, "\u2205": {"f1-score": 0.9901801011240651, "precision": 0.982176904584266, "recall": 0.9983147960903269, "support": 11868}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9633251833740832, "precision": 0.9456, "recall": 0.9817275747508306, "support": 602}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.8966789667896679, "precision": 0.9972640218878249, "recall": 0.8145251396648044, "support": 895}},
  "cl_report_full": {"\u0027": {"f1-score": 0.94050822122571, "precision": 1.0, "recall": 0.8876975169300225, "support": 1772}, "macro avg": {"f1-score": 0.3677104067666689, "precision": 0.4906301158090114, "recall": 0.33255016079948024, "support": 23855}, "micro avg": {"f1-score": 0.7589260431950988, "precision": 0.9832577374599787, "recall": 0.6179417312932299, "support": 23855}, "weighted avg": {"f1-score": 0.6734421134169253, "precision": 0.8915947870645563, "recall": 0.6179417312932299, "support": 23855}, "\u2205": {"f1-score": 0.9322160588536135, "precision": 0.982176904584266, "recall": 0.8870919436957173, "support": 13356}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1062}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 419}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8406827880512092, "precision": 0.9456, "recall": 0.7567221510883483, "support": 781}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 697}, "\u2423": {"f1-score": 0.22827618600281818, "precision": 0.9972640218878249, "recall": 0.12888967468175389, "support": 5656}},
  "ppcr": 0.6284636344581849
}
```
</details>
