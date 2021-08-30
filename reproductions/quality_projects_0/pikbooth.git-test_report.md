# Test report for javascript / file:///tmp/top-repos-quality-repos-kvmignaw/pikbooth.git HEAD 1136cc8a9cc9b3dac6f509f3bd00b514bc6db3a2

### Classification report

PPCR: 0.899

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 0.873| 0.856| 0.908| 0.899| 1631| 1664| 0.980 |
| `␣` | 0.642| 0.846| 0.819| 0.730| 0.720| 525| 542| 0.969 |
| `'` | 0.709| 0.881| 0.779| 0.786| 0.742| 260| 294| 0.884 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 110| 132| 0.833 |
| `⏎␣⁺␣⁺` | 0.594| 0.719| 0.695| 0.651| 0.641| 57| 59| 0.966 |
| `⏎␣⁻␣⁻` | 0.758| 0.962| 0.962| 0.847| 0.847| 52| 52| 1.000 |
| `⏎` | 0.545| 0.324| 0.065| 0.407| 0.116| 37| 185| 0.200 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 49| 0.082 |
| `macro avg` | 0.524| 0.576| 0.522| 0.541| 0.496| 2676| 2977| 0.899 |
| `micro avg` | 0.822| 0.822| 0.739| 0.822| 0.778| 2676| 2977| 0.899 |
| `weighted avg` | 0.807| 0.822| 0.739| 0.809| 0.742| 2676| 2977| 0.899 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|33 |1424 |191 |5 |0 |5 |6 |0 |0 |
|17 |34 |444 |5 |15 |22 |5 |0 |0 |
|148 |8 |10 |12 |7 |0 |0 |0 |0 |
|34 |16 |13 |0 |229 |0 |2 |0 |0 |
|2 |0 |7 |0 |9 |41 |0 |0 |0 |
|0 |0 |2 |0 |0 |0 |50 |0 |0 |
|45 |3 |1 |0 |0 |0 |0 |0 |0 |
|22 |19 |24 |0 |63 |1 |3 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 110}, "\u0027": {"f1-score": 0.7855917667238421, "precision": 0.7089783281733746, "recall": 0.8807692307692307, "support": 260}, "macro avg": {"f1-score": 0.5410923452762071, "precision": 0.5243298171878158, "recall": 0.5755910681884818, "support": 2676}, "micro avg": {"f1-score": 0.8221225710014948, "precision": 0.8221225710014948, "recall": 0.8221225710014948, "support": 2676}, "weighted avg": {"f1-score": 0.8091284134980633, "precision": 0.8067541404902648, "recall": 0.8221225710014948, "support": 2676}, "\u2205": {"f1-score": 0.9084529505582137, "precision": 0.9468085106382979, "recall": 0.8730839975475169, "support": 1631}, "\u23ce": {"f1-score": 0.4067796610169491, "precision": 0.5454545454545454, "recall": 0.32432432432432434, "support": 37}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6507936507936508, "precision": 0.5942028985507246, "recall": 0.7192982456140351, "support": 57}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8474576271186441, "precision": 0.7575757575757576, "recall": 0.9615384615384616, "support": 52}, "\u2423": {"f1-score": 0.7296631059983566, "precision": 0.6416184971098265, "recall": 0.8457142857142858, "support": 525}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u0027": {"f1-score": 0.7423014586709887, "precision": 0.7089783281733746, "recall": 0.7789115646258503, "support": 294}, "macro avg": {"f1-score": 0.49561587935434137, "precision": 0.5243298171878158, "recall": 0.5218984459897018, "support": 2977}, "micro avg": {"f1-score": 0.7783477799398549, "precision": 0.8221225710014948, "recall": 0.7389989922741015, "support": 2977}, "weighted avg": {"f1-score": 0.7415179513472367, "precision": 0.77495707656572, "recall": 0.7389989922741015, "support": 2977}, "\u2205": {"f1-score": 0.898989898989899, "precision": 0.9468085106382979, "recall": 0.8557692307692307, "support": 1664}, "\u23ce": {"f1-score": 0.11594202898550726, "precision": 0.5454545454545454, "recall": 0.06486486486486487, "support": 185}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6406250000000001, "precision": 0.5942028985507246, "recall": 0.6949152542372882, "support": 59}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8474576271186441, "precision": 0.7575757575757576, "recall": 0.9615384615384616, "support": 52}, "\u2423": {"f1-score": 0.719611021069692, "precision": 0.6416184971098265, "recall": 0.8191881918819188, "support": 542}},
  "ppcr": 0.8988915015115888
}
```
</details>