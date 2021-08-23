# Train report for javascript / file:///tmp/top-repos-quality-repos-ndrmo_t8/hackathons.git HEAD ce0349b5aa8807664b5fadc65ca5af1f855e8c25

### Classification report

PPCR: 0.405

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.998| 1.000| 0.486| 0.999| 0.654| 1329| 2734| 0.486 |
| `⏎` | 1.000| 1.000| 0.753| 1.000| 0.859| 497| 660| 0.753 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 683| 0.003 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 265| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 176| 0.000 |
| `weighted avg` | 0.998| 0.999| 0.404| 0.998| 0.521| 1828| 4518| 0.405 |
| `micro avg` | 0.999| 0.999| 0.404| 0.999| 0.575| 1828| 4518| 0.405 |
| `macro avg` | 0.400| 0.400| 0.248| 0.400| 0.303| 1828| 4518| 0.405 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1405 |1329 |0 |0 |0 |0 |
|681 |2 |0 |0 |0 |0 |
|163 |0 |0 |497 |0 |0 |
|265 |0 |0 |0 |0 |0 |
|176 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Coventry Hack - Mar 2016/client/lib/socketorig.js | 1 |
| Coventry Hack - Mar 2016/client/lib/socket.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.39984962406015034, "precision": 0.3996994740796394, "recall": 0.4, "support": 1828}, "micro avg": {"f1-score": 0.9989059080962801, "precision": 0.9989059080962801, "recall": 0.9989059080962801, "support": 1828}, "weighted avg": {"f1-score": 0.9983592734571659, "precision": 0.9978134602074418, "recall": 0.9989059080962801, "support": 1828}, "\u2205": {"f1-score": 0.9992481203007518, "precision": 0.9984973703981969, "recall": 1.0, "support": 1329}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 497}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 176}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 265}, "macro avg": {"f1-score": 0.302598589685119, "precision": 0.3996994740796394, "recall": 0.24782625080357343, "support": 4518}, "micro avg": {"f1-score": 0.5754806177119446, "precision": 0.9989059080962801, "recall": 0.4041611332447986, "support": 4518}, "weighted avg": {"f1-score": 0.5211844044530382, "precision": 0.7503080590236101, "recall": 0.4041611332447986, "support": 4518}, "\u2205": {"f1-score": 0.6538745387453875, "precision": 0.9984973703981969, "recall": 0.486100950987564, "support": 2734}, "\u23ce": {"f1-score": 0.8591184096802075, "precision": 1.0, "recall": 0.753030303030303, "support": 660}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 683}},
  "ppcr": 0.40460380699424525
}
```
</details>
