# Train report for javascript / file:///tmp/top-repos-quality-repos-uu2rpnux/selected-chapters-linear-algebra.git HEAD 129997ded763f0752ec453478f067e61d7ba2463

### Classification report

PPCR: 0.278

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.983| 0.339| 0.977| 0.503| 695| 2015| 0.345 |
| `␣` | 0.922| 0.877| 0.179| 0.899| 0.300| 162| 794| 0.204 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 143| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 127| 0.000 |
| `micro avg` | 0.963| 0.963| 0.268| 0.963| 0.419| 857| 3079| 0.278 |
| `weighted avg` | 0.962| 0.963| 0.268| 0.962| 0.406| 857| 3079| 0.278 |
| `macro avg` | 0.473| 0.465| 0.129| 0.469| 0.201| 857| 3079| 0.278 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|1320 |683 |12 |0 |0 |
|632 |20 |142 |0 |0 |
|143 |0 |0 |0 |0 |
|127 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/App.js | 14 |
| src/serviceWorker.js | 8 |
| src/components/Message.js | 5 |
| src/utils/Enigma.js | 3 |
| server.js | 1 |
| src/App.test.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4689610836457144, "precision": 0.4734071049860523, "recall": 0.46481925570654586, "support": 857}, "micro avg": {"f1-score": 0.9626604434072346, "precision": 0.9626604434072346, "recall": 0.9626604434072346, "support": 857}, "weighted avg": {"f1-score": 0.9622946278639324, "precision": 0.9621986223963745, "recall": 0.9626604434072346, "support": 857}, "\u2205": {"f1-score": 0.977110157367668, "precision": 0.9715504978662873, "recall": 0.9827338129496402, "support": 695}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8987341772151898, "precision": 0.922077922077922, "recall": 0.8765432098765432, "support": 162}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 143}, "macro avg": {"f1-score": 0.20053837054423856, "precision": 0.4734071049860523, "recall": 0.1294497815502122, "support": 3079}, "micro avg": {"f1-score": 0.4192073170731707, "precision": 0.9626604434072346, "recall": 0.2679441377070477, "support": 3079}, "weighted avg": {"f1-score": 0.4061560430204549, "precision": 0.8735966623353164, "recall": 0.2679441377070477, "support": 3079}, "\u2205": {"f1-score": 0.5025754231052244, "precision": 0.9715504978662873, "recall": 0.3389578163771712, "support": 2015}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 127}, "\u2423": {"f1-score": 0.2995780590717299, "precision": 0.922077922077922, "recall": 0.17884130982367757, "support": 794}},
  "ppcr": 0.2783371224423514
}
```
</details>
