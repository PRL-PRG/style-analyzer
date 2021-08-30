# Train report for javascript / file:///tmp/top-repos-quality-repos-w7b9_hmc/fraidycat.git HEAD a5480cfaf37953463598892f977ce53069a40a69

### Classification report

PPCR: 0.705

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 0.994| 0.919| 0.968| 0.931| 11750| 12715| 0.924 |
| `␣` | 0.962| 0.866| 0.527| 0.911| 0.681| 4065| 6677| 0.609 |
| `⏎` | 0.992| 0.848| 0.393| 0.914| 0.563| 553| 1194| 0.463 |
| `"` | 1.000| 1.000| 0.297| 1.000| 0.458| 191| 643| 0.297 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 79| 603| 0.131 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 721| 0.082 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 334| 0.009 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 803| 0.000 |
| `weighted avg` | 0.942| 0.950| 0.670| 0.945| 0.732| 16700| 23690| 0.705 |
| `macro avg` | 0.487| 0.463| 0.267| 0.474| 0.329| 16700| 23690| 0.705 |
| `micro avg` | 0.950| 0.950| 0.670| 0.950| 0.785| 16700| 23690| 0.705 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|965 |11682 |68 |0 |0 |0 |0 |0 |0 |
|2612 |546 |3519 |0 |0 |0 |0 |0 |0 |
|641 |39 |45 |469 |0 |0 |0 |0 |0 |
|803 |0 |0 |0 |0 |0 |0 |0 |0 |
|452 |0 |0 |0 |0 |191 |0 |0 |0 |
|662 |36 |23 |0 |0 |0 |0 |0 |0 |
|524 |74 |3 |2 |0 |0 |0 |0 |0 |
|331 |0 |1 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/js/view.js | 224 |
| src/js/storage.js | 125 |
| src/js/electron/main.js | 99 |
| src/js/webext/storage.js | 68 |
| src/js/util.js | 54 |
| test.js | 43 |
| src/js/frago.js | 34 |
| src/js/follows.js | 34 |
| src/js/sparkline.js | 30 |
| src/js/electron/storage.js | 27 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 191}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.474223951528446, "precision": 0.48714112240095375, "recall": 0.46349958607585, "support": 16700}, "micro avg": {"f1-score": 0.9497604790419162, "precision": 0.9497604790419162, "recall": 0.9497604790419162, "support": 16700}, "weighted avg": {"f1-score": 0.9448475548536617, "precision": 0.9424548988892711, "recall": 0.9497604790419162, "support": 16700}, "\u2205": {"f1-score": 0.9683756787002115, "precision": 0.9438474589965258, "recall": 0.9942127659574468, "support": 11750}, "\u23ce": {"f1-score": 0.9142300194931774, "precision": 0.9915433403805497, "recall": 0.8481012658227848, "support": 553}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}, "\u2423": {"f1-score": 0.9111859140341791, "precision": 0.9617381798305548, "recall": 0.8656826568265683, "support": 4065}},
  "cl_report_full": {"\"": {"f1-score": 0.4580335731414868, "precision": 1.0, "recall": 0.29704510108864696, "support": 643}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 803}, "macro avg": {"f1-score": 0.32909693965773906, "precision": 0.48714112240095375, "recall": 0.26695411161249333, "support": 23690}, "micro avg": {"f1-score": 0.7853924238672939, "precision": 0.9497604790419162, "recall": 0.6695230054875475, "support": 23690}, "weighted avg": {"f1-score": 0.7324709090621709, "precision": 0.8547677930048043, "recall": 0.6695230054875475, "support": 23690}, "\u2205": {"f1-score": 0.9311334289813487, "precision": 0.9438474589965258, "recall": 0.9187573731812819, "support": 12715}, "\u23ce": {"f1-score": 0.5626874625074986, "precision": 0.9915433403805497, "recall": 0.3927973199329983, "support": 1194}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 334}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 721}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 603}, "\u2423": {"f1-score": 0.6809210526315789, "precision": 0.9617381798305548, "recall": 0.5270330986970196, "support": 6677}},
  "ppcr": 0.7049387927395525
}
```
</details>
