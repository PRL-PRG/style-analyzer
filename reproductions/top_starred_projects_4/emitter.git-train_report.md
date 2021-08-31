# Train report for javascript / file:///tmp/top-repos-quality-repos-sl35_1xu/emitter.git HEAD 6b64b870f651dfd918153813fc124b988d000597

### Classification report

PPCR: 0.333

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.990| 0.374| 0.992| 0.543| 622| 1648| 0.377 |
| `␣` | 0.973| 1.000| 0.550| 0.987| 0.703| 329| 598| 0.550 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 87| 0.046 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 160| 0.019 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 277| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 109| 0.000 |
| `macro avg` | 0.328| 0.332| 0.154| 0.330| 0.208| 958| 2879| 0.333 |
| `weighted avg` | 0.979| 0.986| 0.328| 0.983| 0.457| 958| 2879| 0.333 |
| `micro avg` | 0.986| 0.986| 0.328| 0.986| 0.493| 958| 2879| 0.333 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1026 |616 |6 |0 |0 |0 |0 |
|269 |0 |329 |0 |0 |0 |0 |
|277 |0 |0 |0 |0 |0 |0 |
|157 |3 |0 |0 |0 |0 |0 |
|109 |0 |0 |0 |0 |0 |0 |
|83 |1 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/__tests__/BaseEventEmitter-test.js | 6 |
| gulpfile.js | 5 |
| scripts/babel/default-options.js | 1 |
| index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.32974253613933774, "precision": 0.3278201946936438, "recall": 0.3317256162915327, "support": 958}, "micro avg": {"f1-score": 0.9864300626304802, "precision": 0.9864300626304802, "recall": 0.9864300626304802, "support": 958}, "weighted avg": {"f1-score": 0.9828315951047905, "precision": 0.9793598556833066, "recall": 0.9864300626304802, "support": 958}, "\u2205": {"f1-score": 0.9919484702093397, "precision": 0.9935483870967742, "recall": 0.9903536977491961, "support": 622}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9865067466266867, "precision": 0.9733727810650887, "recall": 1.0, "support": 329}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 277}, "macro avg": {"f1-score": 0.20770022158911047, "precision": 0.3278201946936438, "recall": 0.153992271974543, "support": 2879}, "micro avg": {"f1-score": 0.4925723221266615, "precision": 0.9864300626304802, "recall": 0.328238971865231, "support": 2879}, "weighted avg": {"f1-score": 0.45696379487047545, "precision": 0.7709081851380364, "recall": 0.328238971865231, "support": 2879}, "\u2205": {"f1-score": 0.54320987654321, "precision": 0.9935483870967742, "recall": 0.3737864077669903, "support": 1648}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 109}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 87}, "\u2423": {"f1-score": 0.7029914529914529, "precision": 0.9733727810650887, "recall": 0.5501672240802675, "support": 598}},
  "ppcr": 0.332754428621049
}
```
</details>
