# Test report for javascript / file:///tmp/top-repos-quality-repos-vcus7b9x/aleph.git HEAD 42b2afc8ef9c63033c20ff335e70fd8e7b75b1ed

### Classification report

PPCR: 0.949

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.902| 0.968| 0.960| 0.934| 0.930| 1768| 1784| 0.991 |
| `␣` | 0.873| 0.952| 0.894| 0.911| 0.884| 1152| 1227| 0.939 |
| `'` | 1.000| 0.841| 0.841| 0.913| 0.913| 270| 270| 1.000 |
| `⏎` | 0.979| 0.301| 0.240| 0.461| 0.385| 156| 196| 0.796 |
| `⏎␣⁺␣⁺` | 0.870| 0.482| 0.482| 0.620| 0.620| 83| 83| 1.000 |
| `⏎␣⁻␣⁻` | 0.912| 0.477| 0.463| 0.626| 0.614| 65| 67| 0.970 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 71| 0.225 |
| `micro avg` | 0.899| 0.899| 0.853| 0.899| 0.875| 3510| 3698| 0.949 |
| `weighted avg` | 0.899| 0.899| 0.853| 0.886| 0.854| 3510| 3698| 0.949 |
| `macro avg` | 0.791| 0.574| 0.554| 0.638| 0.621| 3510| 3698| 0.949 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|16 |1712 |56 |0 |0 |0 |0 |0 |
|75 |46 |1097 |0 |0 |6 |3 |0 |
|40 |61 |48 |47 |0 |0 |0 |0 |
|0 |39 |4 |0 |227 |0 |0 |0 |
|0 |17 |26 |0 |0 |40 |0 |0 |
|2 |9 |25 |0 |0 |0 |31 |0 |
|55 |15 |0 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9134808853118712, "precision": 1.0, "recall": 0.8407407407407408, "support": 270}, "macro avg": {"f1-score": 0.637935104162713, "precision": 0.7907759075412892, "recall": 0.57449375944127, "support": 3510}, "micro avg": {"f1-score": 0.8985754985754986, "precision": 0.8985754985754986, "recall": 0.8985754985754986, "support": 3510}, "weighted avg": {"f1-score": 0.8863714707710104, "precision": 0.8986479120689682, "recall": 0.8985754985754986, "support": 3510}, "\u2205": {"f1-score": 0.9337332969730024, "precision": 0.9015271195365983, "recall": 0.9683257918552036, "support": 1768}, "\u23ce": {"f1-score": 0.46078431372549017, "precision": 0.9791666666666666, "recall": 0.30128205128205127, "support": 156}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6201550387596898, "precision": 0.8695652173913043, "recall": 0.4819277108433735, "support": 83}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6262626262626263, "precision": 0.9117647058823529, "recall": 0.47692307692307695, "support": 65}, "\u2423": {"f1-score": 0.9111295681063122, "precision": 0.8734076433121019, "recall": 0.9522569444444444, "support": 1152}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9134808853118712, "precision": 1.0, "recall": 0.8407407407407408, "support": 270}, "macro avg": {"f1-score": 0.620861234820695, "precision": 0.7907759075412892, "recall": 0.5541203889240532, "support": 3698}, "micro avg": {"f1-score": 0.8751387347391787, "precision": 0.8985754985754986, "recall": 0.8528934559221201, "support": 3698}, "weighted avg": {"f1-score": 0.8538347408454311, "precision": 0.8856609990809905, "recall": 0.8528934559221201, "support": 3698}, "\u2205": {"f1-score": 0.9296768938365464, "precision": 0.9015271195365983, "recall": 0.9596412556053812, "support": 1784}, "\u23ce": {"f1-score": 0.38524590163934425, "precision": 0.9791666666666666, "recall": 0.23979591836734693, "support": 196}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 71}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6201550387596898, "precision": 0.8695652173913043, "recall": 0.4819277108433735, "support": 83}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6138613861386137, "precision": 0.9117647058823529, "recall": 0.4626865671641791, "support": 67}, "\u2423": {"f1-score": 0.8836085380587998, "precision": 0.8734076433121019, "recall": 0.8940505297473512, "support": 1227}},
  "ppcr": 0.9491617090319091
}
```
</details>
