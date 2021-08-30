# Test report for javascript / file:///tmp/top-repos-quality-repos-e4r9sw52/vanilla-lazyload.git HEAD 7b2d8b26bde0a6e03ed411b83f80c62b491fc81d

### Classification report

PPCR: 0.827

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.842| 0.996| 0.984| 0.913| 0.907| 566| 573| 0.988 |
| `␣` | 0.862| 0.782| 0.695| 0.820| 0.770| 376| 423| 0.889 |
| `"` | 0.872| 1.000| 0.636| 0.932| 0.735| 75| 118| 0.636 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 95| 0.368 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 49| 0.571 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 50| 0.320 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 19| 0.053 |
| `weighted avg` | 0.789| 0.851| 0.703| 0.816| 0.703| 1097| 1327| 0.827 |
| `macro avg` | 0.368| 0.397| 0.331| 0.381| 0.345| 1097| 1327| 0.827 |
| `micro avg` | 0.851| 0.851| 0.703| 0.851| 0.770| 1097| 1327| 0.827 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|7 |564 |2 |0 |0 |0 |0 |0 |
|47 |71 |294 |11 |0 |0 |0 |0 |
|43 |0 |0 |75 |0 |0 |0 |0 |
|60 |1 |34 |0 |0 |0 |0 |0 |
|34 |6 |10 |0 |0 |0 |0 |0 |
|21 |28 |0 |0 |0 |0 |0 |0 |
|18 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9316770186335404, "precision": 0.872093023255814, "recall": 1.0, "support": 75}, "macro avg": {"f1-score": 0.38062600855217277, "precision": 0.36800773657263897, "recall": 0.3969116178160611, "support": 1097}, "micro avg": {"f1-score": 0.8505013673655424, "precision": 0.8505013673655424, "recall": 0.8505013673655424, "support": 1097}, "weighted avg": {"f1-score": 0.8156526254813584, "precision": 0.7894591259495429, "recall": 0.8505013673655424, "support": 1097}, "\u2205": {"f1-score": 0.912621359223301, "precision": 0.8417910447761194, "recall": 0.9964664310954063, "support": 566}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u2423": {"f1-score": 0.8200836820083681, "precision": 0.8621700879765396, "recall": 0.7819148936170213, "support": 376}},
  "cl_report_full": {"\"": {"f1-score": 0.7352941176470589, "precision": 0.872093023255814, "recall": 0.635593220338983, "support": 118}, "macro avg": {"f1-score": 0.3446299320189719, "precision": 0.36800773657263897, "recall": 0.33070312500702403, "support": 1327}, "micro avg": {"f1-score": 0.7698019801980198, "precision": 0.8505013673655424, "recall": 0.7030896759608138, "support": 1327}, "weighted avg": {"f1-score": 0.7025673003923691, "precision": 0.7158637472607224, "recall": 0.7030896759608138, "support": 1327}, "\u2205": {"f1-score": 0.9074818986323412, "precision": 0.8417910447761194, "recall": 0.9842931937172775, "support": 573}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u2423": {"f1-score": 0.7696335078534031, "precision": 0.8621700879765396, "recall": 0.6950354609929078, "support": 423}},
  "ppcr": 0.8266767143933685
}
```
</details>
