# Test report for javascript / file:///tmp/top-repos-quality-repos-d1qpf08f/realtime-multiplayer-in-html5.git HEAD 00af50dd57baa29f30809925881a624bc50234f1

### Classification report

PPCR: 0.681

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.974| 0.872| 0.969| 0.916| 302| 337| 0.896 |
| `␣` | 0.628| 0.794| 0.262| 0.701| 0.370| 34| 103| 0.330 |
| `'` | 1.000| 0.652| 0.395| 0.789| 0.566| 23| 38| 0.605 |
| `⏎` | 0.933| 0.778| 0.212| 0.848| 0.346| 18| 66| 0.273 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 11| 0.091 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.926| 0.926| 0.631| 0.926| 0.750| 378| 555| 0.681 |
| `macro avg` | 0.392| 0.355| 0.193| 0.368| 0.244| 378| 555| 0.681 |
| `weighted avg` | 0.932| 0.926| 0.631| 0.925| 0.705| 378| 555| 0.681 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|35 |294 |8 |0 |0 |0 |
|69 |7 |27 |0 |0 |0 |
|15 |4 |4 |15 |0 |0 |
|48 |0 |4 |0 |14 |0 |
|10 |0 |0 |0 |1 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.7894736842105263, "precision": 1.0, "recall": 0.6521739130434783, "support": 23}, "macro avg": {"f1-score": 0.36755063903247376, "precision": 0.39168608181189196, "recall": 0.3552865857394349, "support": 378}, "micro avg": {"f1-score": 0.9259259259259259, "precision": 0.9259259259259259, "recall": 0.9259259259259259, "support": 378}, "weighted avg": {"f1-score": 0.9254543123750739, "precision": 0.9318969151603539, "recall": 0.9259259259259259, "support": 378}, "\u2205": {"f1-score": 0.9686985172981878, "precision": 0.9639344262295082, "recall": 0.9735099337748344, "support": 302}, "\u23ce": {"f1-score": 0.8484848484848485, "precision": 0.9333333333333333, "recall": 0.7777777777777778, "support": 18}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.7012987012987012, "precision": 0.627906976744186, "recall": 0.7941176470588235, "support": 34}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5660377358490566, "precision": 1.0, "recall": 0.39473684210526316, "support": 38}, "macro avg": {"f1-score": 0.24416306804007284, "precision": 0.39168608181189196, "recall": 0.1934886152652703, "support": 555}, "micro avg": {"f1-score": 0.7502679528403002, "precision": 0.9259259259259259, "recall": 0.6306306306306306, "support": 555}, "weighted avg": {"f1-score": 0.7046384590910172, "precision": 0.8812978743135054, "recall": 0.6306306306306306, "support": 555}, "\u2205": {"f1-score": 0.9158878504672897, "precision": 0.9639344262295082, "recall": 0.8724035608308606, "support": 337}, "\u23ce": {"f1-score": 0.345679012345679, "precision": 0.9333333333333333, "recall": 0.21212121212121213, "support": 66}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.3698630136986301, "precision": 0.627906976744186, "recall": 0.2621359223300971, "support": 103}},
  "ppcr": 0.6810810810810811
}
```
</details>
