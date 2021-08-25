# Test report for javascript / file:///tmp/top-repos-quality-repos-d64ll3vb/soundslike.git HEAD 02fb51f5c813c9a8fa7071a4f94c74976e1e3989

### Classification report

PPCR: 0.607

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.856| 0.987| 0.855| 0.917| 0.856| 1405| 1622| 0.866 |
| `␣` | 0.896| 0.594| 0.300| 0.715| 0.450| 377| 746| 0.505 |
| `⏎` | 0.947| 0.474| 0.120| 0.632| 0.213| 38| 150| 0.253 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 126| 0.214 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 67| 0.313 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 121| 0.140 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 66| 0.045 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 214| 0.005 |
| `macro avg` | 0.337| 0.257| 0.159| 0.283| 0.190| 1889| 3112| 0.607 |
| `micro avg` | 0.862| 0.862| 0.523| 0.862| 0.651| 1889| 3112| 0.607 |
| `weighted avg` | 0.835| 0.862| 0.523| 0.837| 0.564| 1889| 3112| 0.607 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|217 |1387 |18 |0 |0 |0 |0 |0 |0 |
|369 |153 |224 |0 |0 |0 |0 |0 |0 |
|112 |20 |0 |18 |0 |0 |0 |0 |0 |
|63 |3 |0 |0 |0 |0 |0 |0 |0 |
|213 |0 |1 |0 |0 |0 |0 |0 |0 |
|99 |20 |7 |0 |0 |0 |0 |0 |0 |
|104 |17 |0 |0 |0 |0 |0 |0 |0 |
|46 |20 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "macro avg": {"f1-score": 0.2828896621719588, "precision": 0.33744265756985053, "recall": 0.2568796598574227, "support": 1889}, "micro avg": {"f1-score": 0.8623610375860244, "precision": 0.8623610375860243, "recall": 0.8623610375860243, "support": 1889}, "weighted avg": {"f1-score": 0.8373697435447531, "precision": 0.8346822866628759, "recall": 0.8623610375860243, "support": 1889}, "\u2205": {"f1-score": 0.9170247933884298, "precision": 0.8561728395061728, "recall": 0.9871886120996441, "support": 1405}, "\u23ce": {"f1-score": 0.631578947368421, "precision": 0.9473684210526315, "recall": 0.47368421052631576, "support": 38}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.7145135566188198, "precision": 0.896, "recall": 0.5941644562334217, "support": 377}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 214}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "macro avg": {"f1-score": 0.18980770150677798, "precision": 0.33744265756985053, "recall": 0.1594231544811126, "support": 3112}, "micro avg": {"f1-score": 0.6514697060587882, "precision": 0.8623610375860243, "recall": 0.5234575835475579, "support": 3112}, "weighted avg": {"f1-score": 0.564061217284349, "precision": 0.7066946043820396, "recall": 0.5234575835475579, "support": 3112}, "\u2205": {"f1-score": 0.8556446637877854, "precision": 0.8561728395061728, "recall": 0.8551171393341553, "support": 1622}, "\u23ce": {"f1-score": 0.21301775147928992, "precision": 0.9473684210526315, "recall": 0.12, "support": 150}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 126}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u2423": {"f1-score": 0.4497991967871485, "precision": 0.896, "recall": 0.3002680965147453, "support": 746}},
  "ppcr": 0.6070051413881749
}
```
</details>
