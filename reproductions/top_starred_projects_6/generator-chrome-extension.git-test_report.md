# Test report for javascript / file:///tmp/top-repos-quality-repos-kgq856p0/generator-chrome-extension.git HEAD 3304eefb1a7d04e9e1a487b8b476d51a72d42aff

### Classification report

PPCR: 0.391

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.943| 1.000| 0.642| 0.970| 0.764| 115| 179| 0.642 |
| `'` | 1.000| 0.959| 0.480| 0.979| 0.648| 49| 98| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 64| 0.047 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 25| 0.080 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 25| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 16| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 25| 0.000 |
| `micro avg` | 0.959| 0.959| 0.375| 0.959| 0.539| 169| 432| 0.391 |
| `weighted avg` | 0.931| 0.959| 0.375| 0.944| 0.464| 169| 432| 0.391 |
| `macro avg` | 0.278| 0.280| 0.160| 0.279| 0.202| 169| 432| 0.391 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|64 |115 |0 |0 |0 |0 |0 |0 |
|61 |3 |0 |0 |0 |0 |0 |0 |
|49 |2 |0 |47 |0 |0 |0 |0 |
|25 |0 |0 |0 |0 |0 |0 |0 |
|25 |0 |0 |0 |0 |0 |0 |0 |
|23 |2 |0 |0 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9791666666666666, "precision": 1.0, "recall": 0.9591836734693877, "support": 49}, "macro avg": {"f1-score": 0.2785186859553948, "precision": 0.27751756440281034, "recall": 0.27988338192419826, "support": 169}, "micro avg": {"f1-score": 0.9585798816568046, "precision": 0.9585798816568047, "recall": 0.9585798816568047, "support": 169}, "weighted avg": {"f1-score": 0.9442753975981825, "precision": 0.9313706470074694, "recall": 0.9585798816568047, "support": 169}, "\u2205": {"f1-score": 0.9704641350210971, "precision": 0.9426229508196722, "recall": 1.0, "support": 115}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6482758620689655, "precision": 1.0, "recall": 0.47959183673469385, "support": 98}, "macro avg": {"f1-score": 0.20177078048540986, "precision": 0.27751756440281034, "recall": 0.16029284818476475, "support": 432}, "micro avg": {"f1-score": 0.5391014975041597, "precision": 0.9585798816568047, "recall": 0.375, "support": 432}, "weighted avg": {"f1-score": 0.4636769516681305, "precision": 0.6174294171220401, "recall": 0.375, "support": 432}, "\u2205": {"f1-score": 0.7641196013289037, "precision": 0.9426229508196722, "recall": 0.6424581005586593, "support": 179}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}},
  "ppcr": 0.3912037037037037
}
```
</details>
