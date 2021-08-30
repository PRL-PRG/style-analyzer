# Train report for javascript / file:///tmp/top-repos-quality-repos-ch9n34ot/hypercore.git HEAD becc8ce8e2317d84e60724ebaf17fdc5db3c40f4

### Classification report

PPCR: 0.909

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.999| 0.994| 0.976| 0.996| 0.987| 30713| 31271| 0.982 |
| `␣` | 0.976| 0.981| 0.928| 0.979| 0.952| 16058| 16974| 0.946 |
| `'` | 1.000| 1.000| 0.996| 1.000| 0.998| 4175| 4193| 0.996 |
| `⏎␣⁺␣⁺` | 0.982| 0.989| 0.940| 0.986| 0.960| 1226| 1291| 0.950 |
| `⏎␣⁻␣⁻` | 0.812| 0.981| 0.893| 0.888| 0.850| 1173| 1289| 0.910 |
| `⏎⏎` | 0.996| 0.989| 0.208| 0.993| 0.344| 278| 1325| 0.210 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 185| 2838| 0.065 |
| `weighted avg` | 0.984| 0.987| 0.897| 0.985| 0.912| 53808| 59181| 0.909 |
| `macro avg` | 0.824| 0.848| 0.706| 0.835| 0.727| 53808| 59181| 0.909 |
| `micro avg` | 0.987| 0.987| 0.897| 0.987| 0.940| 53808| 59181| 0.909 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|558 |30526 |186 |0 |0 |1 |0 |0 |
|916 |15 |15754 |0 |0 |0 |22 |267 |
|2653 |7 |178 |0 |0 |0 |0 |0 |
|18 |0 |0 |0 |4175 |0 |0 |0 |
|1047 |0 |3 |0 |0 |275 |0 |0 |
|65 |7 |6 |0 |0 |0 |1213 |0 |
|116 |14 |8 |0 |0 |0 |0 |1151 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/copy.js | 225 |
| index.js | 103 |
| test/ack.js | 82 |
| test/compat.js | 61 |
| lib/bitfield.js | 40 |
| lib/replicate.js | 38 |
| test/tree-index.js | 36 |
| test/basic.js | 33 |
| test/extensions.js | 11 |
| test/value-encoding.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4175}, "macro avg": {"f1-score": 0.8345695596598367, "precision": 0.8236071083678239, "recall": 0.847832816461772, "support": 53808}, "micro avg": {"f1-score": 0.9867305976806423, "precision": 0.9867305976806423, "recall": 0.9867305976806423, "support": 53808}, "weighted avg": {"f1-score": 0.9852767134105982, "precision": 0.9841826608776083, "recall": 0.9867305976806423, "support": 53808}, "\u2205": {"f1-score": 0.9962468587839822, "precision": 0.9985933462003991, "recall": 0.9939113730342201, "support": 30713}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 185}, "\u23ce\u23ce": {"f1-score": 0.9927797833935018, "precision": 0.9963768115942029, "recall": 0.9892086330935251, "support": 278}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9857781389678992, "precision": 0.9821862348178138, "recall": 0.9893964110929854, "support": 1226}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8884600540331918, "precision": 0.811706629055007, "recall": 0.9812446717817562, "support": 1173}, "\u2423": {"f1-score": 0.9787220824402821, "precision": 0.9763867369073442, "recall": 0.9810686262299165, "support": 16058}},
  "cl_report_full": {"\u0027": {"f1-score": 0.997848948374761, "precision": 1.0, "recall": 0.9957071309325065, "support": 4193}, "macro avg": {"f1-score": 0.7272979781012322, "precision": 0.8236071083678239, "recall": 0.7057253802918678, "support": 59181}, "micro avg": {"f1-score": 0.9398082999230013, "precision": 0.9867305976806423, "recall": 0.8971460434936889, "support": 59181}, "weighted avg": {"f1-score": 0.9124710408271783, "precision": 0.939958644677548, "recall": 0.8971460434936889, "support": 59181}, "\u2205": {"f1-score": 0.9872574385510996, "precision": 0.9985933462003991, "recall": 0.9761760097214671, "support": 31271}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2838}, "\u23ce\u23ce": {"f1-score": 0.34353529044347286, "precision": 0.9963768115942029, "recall": 0.20754716981132076, "support": 1325}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9604117181314331, "precision": 0.9821862348178138, "recall": 0.9395817195972115, "support": 1291}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8503878832656075, "precision": 0.811706629055007, "recall": 0.8929402637703646, "support": 1289}, "\u2423": {"f1-score": 0.9516445679422514, "precision": 0.9763867369073442, "recall": 0.9281253682102039, "support": 16974}},
  "ppcr": 0.9092107264155725
}
```
</details>
