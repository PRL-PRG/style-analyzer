# Train report for javascript / file:///tmp/top-repos-quality-repos-ibuxsox4/raspberrycast.git HEAD 1976dae9cfb5f17fe72acc7bf3ec9a7fe9b8ecd8

### Classification report

PPCR: 0.122

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 303| 606| 0.500 |
| `␣` | 0.897| 1.000| 0.186| 0.946| 0.308| 183| 983| 0.186 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 2105| 0.010 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 157| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 149| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 157| 0.000 |
| `weighted avg` | 0.921| 0.959| 0.117| 0.939| 0.170| 507| 4157| 0.122 |
| `macro avg` | 0.316| 0.333| 0.114| 0.324| 0.163| 507| 4157| 0.122 |
| `micro avg` | 0.959| 0.959| 0.117| 0.959| 0.208| 507| 4157| 0.122 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎⇥⁺| ⏎| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|2084 |0 |21 |0 |0 |0 |0 |
|800 |0 |183 |0 |0 |0 |0 |
|303 |0 |0 |303 |0 |0 |0 |
|157 |0 |0 |0 |0 |0 |0 |
|149 |0 |0 |0 |0 |0 |0 |
|157 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| FirefoxExtension/index.js | 11 |
| ChromeExtension/js/popup.js | 4 |
| FirefoxExtension/data/js/background.js | 2 |
| ChromeExtension/js/background.js | 2 |
| static/remote.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 303}, "macro avg": {"f1-score": 0.32428940568475456, "precision": 0.3161764705882353, "recall": 0.3333333333333333, "support": 507}, "micro avg": {"f1-score": 0.9585798816568046, "precision": 0.9585798816568047, "recall": 0.9585798816568047, "support": 507}, "weighted avg": {"f1-score": 0.9389936241456813, "precision": 0.9214235990254089, "recall": 0.9585798816568047, "support": 507}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9457364341085273, "precision": 0.8970588235294118, "recall": 1.0, "support": 183}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 606}, "macro avg": {"f1-score": 0.16250117008330991, "precision": 0.3161764705882353, "recall": 0.11436080027127841, "support": 4157}, "micro avg": {"f1-score": 0.20840480274442538, "precision": 0.9585798816568047, "recall": 0.11691123406302623, "support": 4157}, "weighted avg": {"f1-score": 0.17009828429589335, "precision": 0.35790445598494386, "recall": 0.11691123406302623, "support": 4157}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2105}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 149}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "\u2423": {"f1-score": 0.30834035383319297, "precision": 0.8970588235294118, "recall": 0.1861648016276704, "support": 983}},
  "ppcr": 0.12196295405340389
}
```
</details>
