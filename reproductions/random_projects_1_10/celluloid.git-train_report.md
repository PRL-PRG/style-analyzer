# Train report for javascript / file:///tmp/top-repos-quality-repos-moexuz3b/celluloid.git HEAD f54bfe86c60324c7f57667cb494bdd1cbb4e17c8

### Classification report

PPCR: 0.805

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 1.000| 0.994| 0.986| 0.984| 7557| 7600| 0.994 |
| `␣` | 1.000| 0.908| 0.633| 0.952| 0.776| 1691| 2425| 0.697 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 890| 1780| 0.500 |
| `⏎␣⁻␣⁻` | 0.964| 0.884| 0.868| 0.922| 0.914| 336| 342| 0.982 |
| `⏎␣⁺␣⁺` | 0.963| 0.921| 0.632| 0.942| 0.763| 229| 334| 0.686 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 728| 0.022 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 113| 0.000 |
| `micro avg` | 0.979| 0.979| 0.787| 0.979| 0.873| 10719| 13322| 0.805 |
| `weighted avg` | 0.978| 0.979| 0.787| 0.978| 0.834| 10719| 13322| 0.805 |
| `macro avg` | 0.700| 0.673| 0.518| 0.686| 0.586| 10719| 13322| 0.805 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|43 |7557 |0 |0 |0 |0 |0 |0 |
|734 |136 |1536 |0 |0 |11 |8 |0 |
|890 |0 |0 |890 |0 |0 |0 |0 |
|712 |16 |0 |0 |0 |0 |0 |0 |
|6 |39 |0 |0 |0 |297 |0 |0 |
|105 |18 |0 |0 |0 |0 |211 |0 |
|113 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| spec/transform.js | 62 |
| spec/vector.js | 36 |
| build-html.js | 34 |
| spec/animation.js | 25 |
| spec/render-loop.js | 15 |
| debugger/shims.js | 14 |
| spec/gl.js | 12 |
| debugger/main.js | 11 |
| spec/status.js | 6 |
| spec/support/scenario.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 890}, "macro avg": {"f1-score": 0.6860932399115608, "precision": 0.7001205503739002, "recall": 0.6733806018178617, "support": 10719}, "micro avg": {"f1-score": 0.9787293590820039, "precision": 0.9787293590820039, "recall": 0.9787293590820039, "support": 10719}, "weighted avg": {"f1-score": 0.9776402377294187, "precision": 0.9776340469342942, "recall": 0.9787293590820039, "support": 10719}, "\u2205": {"f1-score": 0.9863603732950467, "precision": 0.9730878186968839, "recall": 1.0, "support": 7557}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9419642857142857, "precision": 0.9634703196347032, "recall": 0.9213973799126638, "support": 229}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.922360248447205, "precision": 0.9642857142857143, "recall": 0.8839285714285714, "support": 336}, "\u2423": {"f1-score": 0.9519677719243881, "precision": 1.0, "recall": 0.9083382613837966, "support": 1691}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 1780}, "macro avg": {"f1-score": 0.5861121444217959, "precision": 0.7001205503739002, "recall": 0.518271678099502, "support": 13322}, "micro avg": {"f1-score": 0.8727590366457303, "precision": 0.9787293590820039, "recall": 0.7874943702146825, "support": 13322}, "weighted avg": {"f1-score": 0.8339723216163918, "precision": 0.9196856495376086, "recall": 0.7874943702146825, "support": 13322}, "\u2205": {"f1-score": 0.9836001561889887, "precision": 0.9730878186968839, "recall": 0.9943421052631579, "support": 7600}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 728}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 113}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7631103074141049, "precision": 0.9634703196347032, "recall": 0.6317365269461078, "support": 334}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.913846153846154, "precision": 0.9642857142857143, "recall": 0.868421052631579, "support": 342}, "\u2423": {"f1-score": 0.7755617268366574, "precision": 1.0, "recall": 0.63340206185567, "support": 2425}},
  "ppcr": 0.8046089175799429
}
```
</details>
