# Train report for javascript / file:///tmp/top-repos-quality-repos-qdbi5g4w/hydrogen.git HEAD 79f5e21ac97302edee8cce269256dbb11b57f3c3

### Classification report

PPCR: 0.495

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.891| 0.991| 0.655| 0.938| 0.755| 13087| 19798| 0.661 |
| `␣` | 0.923| 0.636| 0.177| 0.753| 0.297| 2564| 9229| 0.278 |
| `"` | 1.000| 1.000| 0.496| 1.000| 0.663| 1607| 3241| 0.496 |
| `⏎␣⁻␣⁻` | 0.999| 0.559| 0.512| 0.716| 0.677| 1228| 1340| 0.916 |
| `⏎␣⁺␣⁺` | 0.983| 0.816| 0.243| 0.892| 0.390| 418| 1401| 0.298 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 2426| 0.023 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 888| 0.000 |
| `micro avg` | 0.909| 0.909| 0.450| 0.909| 0.602| 18960| 38323| 0.495 |
| `weighted avg` | 0.911| 0.909| 0.450| 0.900| 0.555| 18960| 38323| 0.495 |
| `macro avg` | 0.685| 0.572| 0.298| 0.614| 0.397| 18960| 38323| 0.495 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|6711 |12964 |117 |0 |0 |6 |0 |0 |
|6665 |933 |1631 |0 |0 |0 |0 |0 |
|1634 |0 |0 |1607 |0 |0 |0 |0 |
|2370 |55 |0 |0 |0 |0 |1 |0 |
|983 |57 |20 |0 |0 |341 |0 |0 |
|112 |542 |0 |0 |0 |0 |686 |0 |
|888 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/main.js | 158 |
| lib/code-manager.js | 128 |
| lib/components/kernel-monitor.js | 98 |
| lib/import-notebook.js | 92 |
| spec/store/index-spec.js | 87 |
| lib/ws-kernel-picker.js | 87 |
| lib/utils.js | 83 |
| lib/zmq-kernel.js | 66 |
| spec/code-manager-spec.js | 59 |
| lib/config.js | 58 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1607}, "macro avg": {"f1-score": 0.6141545629901675, "precision": 0.6849571389663106, "recall": 0.5715911714649246, "support": 18960}, "micro avg": {"f1-score": 0.9087025316455696, "precision": 0.9087025316455696, "recall": 0.9087025316455696, "support": 18960}, "weighted avg": {"f1-score": 0.9001810142238171, "precision": 0.9108108930679764, "recall": 0.9087025316455696, "support": 18960}, "\u2205": {"f1-score": 0.9381286634344019, "precision": 0.8909353309050925, "recall": 0.9906013601283716, "support": 13087}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8915032679738562, "precision": 0.9827089337175793, "recall": 0.8157894736842105, "support": 418}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7164490861618799, "precision": 0.9985443959243085, "recall": 0.5586319218241043, "support": 1228}, "\u2423": {"f1-score": 0.7530009233610341, "precision": 0.9225113122171946, "recall": 0.6361154446177847, "support": 2564}},
  "cl_report_full": {"\"": {"f1-score": 0.6629537953795379, "precision": 1.0, "recall": 0.49583461894477016, "support": 3241}, "macro avg": {"f1-score": 0.3973489591010236, "precision": 0.6849571389663106, "recall": 0.29753023531614836, "support": 38323}, "micro avg": {"f1-score": 0.6015397238273135, "precision": 0.9087025316455696, "recall": 0.4495733632544425, "support": 38323}, "weighted avg": {"f1-score": 0.5553879038678804, "precision": 0.8378367896142893, "recall": 0.4495733632544425, "support": 38323}, "\u2205": {"f1-score": 0.7548400244548605, "precision": 0.8909353309050925, "recall": 0.654813617537125, "support": 19798}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2426}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 888}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.3901601830663615, "precision": 0.9827089337175793, "recall": 0.24339757316202712, "support": 1401}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6768623581647756, "precision": 0.9985443959243085, "recall": 0.5119402985074627, "support": 1340}, "\u2423": {"f1-score": 0.2966263526416295, "precision": 0.9225113122171946, "recall": 0.17672553906165347, "support": 9229}},
  "ppcr": 0.49474206090337397
}
```
</details>
