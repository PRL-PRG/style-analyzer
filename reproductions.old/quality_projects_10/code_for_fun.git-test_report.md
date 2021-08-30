# Test report for javascript / file:///tmp/top-repos-quality-repos-8wxup_of/code_for_fun.git HEAD 9abbd7cf8d332319f83bc1640693178af90ac9de

### Classification report

PPCR: 0.959

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.969| 0.877| 0.858| 0.920| 0.910| 10767| 11002| 0.979 |
| `␣` | 0.745| 0.952| 0.903| 0.836| 0.817| 4429| 4668| 0.949 |
| `⏎` | 0.555| 0.618| 0.595| 0.585| 0.574| 1258| 1308| 0.962 |
| `'` | 0.979| 0.854| 0.765| 0.913| 0.859| 666| 744| 0.895 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.897| 0.744| 0.738| 0.813| 0.810| 454| 458| 0.991 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.954| 0.501| 0.492| 0.657| 0.649| 411| 419| 0.981 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 354| 510| 0.694 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 11| 0.727 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 9| 0.556 |
| `macro avg` | 0.567| 0.505| 0.483| 0.525| 0.513| 18352| 19129| 0.959 |
| `micro avg` | 0.847| 0.847| 0.813| 0.847| 0.829| 18352| 19129| 0.959 |
| `weighted avg` | 0.865| 0.847| 0.813| 0.850| 0.829| 18352| 19129| 0.959 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|235 |9439 |1085 |228 |0 |13 |0 |0 |2 |0 |
|239 |137 |4215 |50 |1 |1 |3 |0 |22 |0 |
|50 |60 |209 |778 |10 |98 |90 |0 |9 |4 |
|78 |51 |46 |0 |569 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |0 |5 |0 |
|3 |0 |1 |1 |0 |0 |0 |0 |0 |6 |
|156 |1 |11 |327 |0 |8 |6 |0 |1 |0 |
|4 |16 |39 |4 |1 |56 |0 |0 |338 |0 |
|8 |40 |49 |14 |0 |0 |102 |0 |0 |206 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9125902165196471, "precision": 0.9793459552495697, "recall": 0.8543543543543544, "support": 666}, "macro avg": {"f1-score": 0.5249432910628161, "precision": 0.5665088555893542, "recall": 0.5052053914912459, "support": 18352}, "micro avg": {"f1-score": 0.847046643417611, "precision": 0.8470466434176112, "recall": 0.8470466434176112, "support": 18352}, "weighted avg": {"f1-score": 0.8497916192512563, "precision": 0.8653286915274935, "recall": 0.8470466434176112, "support": 18352}, "\u2205": {"f1-score": 0.9203841840963385, "precision": 0.9686986863711001, "recall": 0.8766601653199592, "support": 10767}, "\u23ce": {"f1-score": 0.5849624060150376, "precision": 0.5549215406562055, "recall": 0.6184419713831478, "support": 1258}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 354}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8134777376654633, "precision": 0.896551724137931, "recall": 0.7444933920704846, "support": 454}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.6570972886762361, "precision": 0.9537037037037037, "recall": 0.5012165450121655, "support": 411}, "\u2423": {"f1-score": 0.835977786592622, "precision": 0.7453580901856764, "recall": 0.9516820952811018, "support": 4429}},
  "cl_report_full": {"\u0027": {"f1-score": 0.858867924528302, "precision": 0.9793459552495697, "recall": 0.7647849462365591, "support": 744}, "macro avg": {"f1-score": 0.5131132247889152, "precision": 0.5665088555893542, "recall": 0.4833461592245152, "support": 19129}, "micro avg": {"f1-score": 0.8294869400496252, "precision": 0.8470466434176112, "recall": 0.8126404934915573, "support": 19129}, "weighted avg": {"f1-score": 0.8288993328594959, "precision": 0.8574231700471849, "recall": 0.8126404934915573, "support": 19129}, "\u2205": {"f1-score": 0.9099585462257784, "precision": 0.9686986863711001, "recall": 0.8579349209234685, "support": 11002}, "\u23ce": {"f1-score": 0.574169741697417, "precision": 0.5549215406562055, "recall": 0.5948012232415902, "support": 1308}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 510}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8095808383233534, "precision": 0.896551724137931, "recall": 0.7379912663755459, "support": 458}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.6488188976377953, "precision": 0.9537037037037037, "recall": 0.4916467780429594, "support": 419}, "\u2423": {"f1-score": 0.8166230746875908, "precision": 0.7453580901856764, "recall": 0.9029562982005142, "support": 4668}},
  "ppcr": 0.9593810444874274
}
```
</details>