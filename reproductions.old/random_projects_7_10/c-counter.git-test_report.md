# Test report for javascript / file:///tmp/top-repos-quality-repos-kyshot0_/c-counter.git HEAD fdeabc2c1686484dafdae95b5cb0f33f59dfe504

### Classification report

PPCR: 0.579

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.940| 0.940| 0.643| 0.940| 0.764| 67| 98| 0.684 |
| `␣` | 0.765| 0.812| 0.342| 0.788| 0.473| 16| 38| 0.421 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 9| 0.111 |
| `macro avg` | 0.568| 0.584| 0.328| 0.576| 0.412| 84| 145| 0.579 |
| `micro avg` | 0.905| 0.905| 0.524| 0.905| 0.664| 84| 145| 0.579 |
| `weighted avg` | 0.896| 0.905| 0.524| 0.900| 0.640| 84| 145| 0.579 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|31 |63 |4 |0 |
|22 |3 |13 |0 |
|8 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.5760590984471582, "precision": 0.5683347966052092, "recall": 0.5842661691542289, "support": 84}, "micro avg": {"f1-score": 0.9047619047619048, "precision": 0.9047619047619048, "recall": 0.9047619047619048, "support": 84}, "weighted avg": {"f1-score": 0.9000721500721499, "precision": 0.8956582633053222, "recall": 0.9047619047619048, "support": 84}, "\u2205": {"f1-score": 0.9402985074626865, "precision": 0.9402985074626866, "recall": 0.9402985074626866, "support": 67}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.787878787878788, "precision": 0.7647058823529411, "recall": 0.8125, "support": 16}},
  "cl_report_full": {"macro avg": {"f1-score": 0.4121212121212121, "precision": 0.5683347966052092, "recall": 0.32832080200501257, "support": 145}, "micro avg": {"f1-score": 0.663755458515284, "precision": 0.9047619047619048, "recall": 0.5241379310344828, "support": 145}, "weighted avg": {"f1-score": 0.6400000000000001, "precision": 0.8359177742121039, "recall": 0.5241379310344828, "support": 145}, "\u2205": {"f1-score": 0.7636363636363637, "precision": 0.9402985074626866, "recall": 0.6428571428571429, "support": 98}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.4727272727272727, "precision": 0.7647058823529411, "recall": 0.34210526315789475, "support": 38}},
  "ppcr": 0.5793103448275863
}
```
</details>
