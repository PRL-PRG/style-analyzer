# Train report for javascript / file:///tmp/top-repos-quality-repos-oeiz69fs/tweetdeckplayer.git HEAD ff7737dabeed3d7d98e52adcd839744f0618a7e6

### Classification report

PPCR: 0.776

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.994| 0.968| 0.975| 0.962| 11481| 11791| 0.974 |
| `␣` | 0.987| 0.830| 0.404| 0.901| 0.574| 2682| 5501| 0.488 |
| `'` | 1.000| 1.000| 0.974| 1.000| 0.987| 2327| 2389| 0.974 |
| `⏎␣⁺␣⁺` | 0.905| 0.957| 0.843| 0.930| 0.873| 714| 810| 0.881 |
| `⏎␣⁻␣⁻` | 0.936| 0.928| 0.849| 0.932| 0.891| 710| 776| 0.915 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 61| 1609| 0.038 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 275| 0.004 |
| `macro avg` | 0.683| 0.673| 0.577| 0.677| 0.612| 17976| 23151| 0.776 |
| `micro avg` | 0.963| 0.963| 0.748| 0.963| 0.842| 17976| 23151| 0.776 |
| `weighted avg` | 0.960| 0.963| 0.748| 0.960| 0.789| 17976| 23151| 0.776 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|310 |11415 |27 |0 |0 |24 |15 |0 |
|2819 |383 |2225 |0 |0 |44 |30 |0 |
|62 |0 |0 |2327 |0 |0 |0 |0 |
|1548 |58 |1 |0 |0 |2 |0 |0 |
|96 |29 |2 |0 |0 |683 |0 |0 |
|66 |49 |0 |0 |0 |2 |659 |0 |
|274 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/index.js | 186 |
| src/preload.js | 83 |
| src/config-schema.js | 54 |
| src/util.js | 37 |
| .eslintrc.js | 36 |
| src/setting.js | 32 |
| src/preload_scripts/quote-without-notification.js | 27 |
| src/preload_scripts/image-viewer.js | 26 |
| src/preload_scripts/wordfilter.js | 23 |
| src/preload_popup.js | 21 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2327}, "macro avg": {"f1-score": 0.676903321061252, "precision": 0.6834060291072435, "recall": 0.6726582559324633, "support": 17976}, "micro avg": {"f1-score": 0.9628949710725412, "precision": 0.9628949710725412, "recall": 0.9628949710725412, "support": 17976}, "weighted avg": {"f1-score": 0.9603835253738586, "precision": 0.9604264674621786, "recall": 0.9628949710725412, "support": 17976}, "\u2205": {"f1-score": 0.9749743764947045, "precision": 0.9564306661080855, "recall": 0.994251371831722, "support": 11481}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9298842750170183, "precision": 0.904635761589404, "recall": 0.9565826330532213, "support": 714}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9321074964639321, "precision": 0.9360795454545454, "recall": 0.928169014084507, "support": 710}, "\u2423": {"f1-score": 0.9013570994531092, "precision": 0.9866962305986696, "recall": 0.8296047725577927, "support": 2682}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9868532654792197, "precision": 1.0, "recall": 0.9740477187107577, "support": 2389}, "macro avg": {"f1-score": 0.6123174477212451, "precision": 0.6834060291072435, "recall": 0.5770096549833889, "support": 23151}, "micro avg": {"f1-score": 0.8417341405889075, "precision": 0.9628949710725412, "recall": 0.7476566886959527, "support": 23151}, "weighted avg": {"f1-score": 0.788629836797179, "precision": 0.8877907063523761, "recall": 0.7476566886959527, "support": 23151}, "\u2205": {"f1-score": 0.9622355222119194, "precision": 0.9564306661080855, "recall": 0.9681112713086252, "support": 11791}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1609}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 275}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8728434504792333, "precision": 0.904635761589404, "recall": 0.8432098765432099, "support": 810}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8905405405405404, "precision": 0.9360795454545454, "recall": 0.8492268041237113, "support": 776}, "\u2423": {"f1-score": 0.573749355337803, "precision": 0.9866962305986696, "recall": 0.4044719141974186, "support": 5501}},
  "ppcr": 0.7764675391991707
}
```
</details>
