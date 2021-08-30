# Test report for javascript / file:///tmp/top-repos-quality-repos-m_gkys17/instafeed.js.git HEAD 22cdc482a2a01aaa59437c0476e7e7570a026a22

### Classification report

PPCR: 0.556

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.890| 0.975| 0.806| 0.931| 0.846| 316| 382| 0.827 |
| `'` | 0.786| 0.564| 0.500| 0.657| 0.611| 39| 44| 0.886 |
| `␣` | 0.842| 0.421| 0.092| 0.561| 0.167| 38| 173| 0.220 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 24| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 18| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 33| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 33| 0.000 |
| `weighted avg` | 0.875| 0.880| 0.489| 0.868| 0.536| 393| 707| 0.556 |
| `micro avg` | 0.880| 0.880| 0.489| 0.880| 0.629| 393| 707| 0.556 |
| `macro avg` | 0.360| 0.280| 0.200| 0.307| 0.232| 393| 707| 0.556 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|66 |308 |2 |6 |0 |0 |0 |0 |
|135 |22 |16 |0 |0 |0 |0 |0 |
|5 |16 |1 |22 |0 |0 |0 |0 |
|24 |0 |0 |0 |0 |0 |0 |0 |
|33 |0 |0 |0 |0 |0 |0 |0 |
|33 |0 |0 |0 |0 |0 |0 |0 |
|18 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.6567164179104477, "precision": 0.7857142857142857, "recall": 0.5641025641025641, "support": 39}, "macro avg": {"f1-score": 0.30694764597836294, "precision": 0.3597132798966864, "recall": 0.2799769628550441, "support": 393}, "micro avg": {"f1-score": 0.8804071246819338, "precision": 0.8804071246819338, "recall": 0.8804071246819338, "support": 393}, "weighted avg": {"f1-score": 0.8676528491204791, "precision": 0.8751594270501741, "recall": 0.8804071246819338, "support": 393}, "\u2205": {"f1-score": 0.930513595166163, "precision": 0.8901734104046243, "recall": 0.9746835443037974, "support": 316}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.5614035087719298, "precision": 0.8421052631578947, "recall": 0.42105263157894735, "support": 38}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6111111111111112, "precision": 0.7857142857142857, "recall": 0.5, "support": 44}, "macro avg": {"f1-score": 0.231990231990232, "precision": 0.3597132798966864, "recall": 0.19982403880657668, "support": 707}, "micro avg": {"f1-score": 0.6290909090909091, "precision": 0.8804071246819338, "recall": 0.4893917963224894, "support": 707}, "weighted avg": {"f1-score": 0.5360014023380361, "precision": 0.7359291115591383, "recall": 0.4893917963224894, "support": 707}, "\u2205": {"f1-score": 0.8461538461538461, "precision": 0.8901734104046243, "recall": 0.806282722513089, "support": 382}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u2423": {"f1-score": 0.16666666666666669, "precision": 0.8421052631578947, "recall": 0.09248554913294797, "support": 173}},
  "ppcr": 0.5558698727015559
}
```
</details>
