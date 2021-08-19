# Test report for javascript / file:///tmp/top-repos-quality-repos-ufwu3n5e/axios.git HEAD e9965bfafc82d8b42765705061b9ebe2d5532493

### Classification report

PPCR: 0.911

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 0.998| 0.995| 0.997| 0.996| 5036| 5049| 0.997 |
| `␣` | 0.988| 0.993| 0.685| 0.991| 0.809| 1290| 1870| 0.690 |
| `'` | 0.992| 0.998| 0.990| 0.995| 0.991| 893| 900| 0.992 |
| `⏎␣⁻␣⁻` | 0.972| 0.994| 0.924| 0.983| 0.947| 343| 369| 0.930 |
| `⏎␣⁺␣⁺` | 1.000| 0.988| 0.845| 0.994| 0.916| 325| 380| 0.855 |
| `⏎` | 0.904| 0.893| 0.653| 0.898| 0.758| 242| 331| 0.731 |
| `⏎⏎` | 0.878| 0.824| 0.671| 0.850| 0.761| 131| 161| 0.814 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 10| 0.700 |
| `weighted avg` | 0.989| 0.990| 0.902| 0.989| 0.938| 8267| 9070| 0.911 |
| `macro avg` | 0.841| 0.836| 0.720| 0.839| 0.772| 8267| 9070| 0.911 |
| `micro avg` | 0.990| 0.990| 0.902| 0.990| 0.944| 8267| 9070| 0.911 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|13 |5024 |11 |0 |0 |0 |1 |0 |0 |
|580 |0 |1281 |0 |0 |0 |9 |0 |0 |
|7 |2 |0 |891 |0 |0 |0 |0 |0 |
|89 |11 |0 |0 |216 |0 |0 |15 |0 |
|55 |1 |3 |0 |0 |321 |0 |0 |0 |
|26 |1 |1 |0 |0 |0 |341 |0 |0 |
|30 |0 |0 |0 |23 |0 |0 |108 |0 |
|3 |0 |0 |7 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u0027": {"f1-score": 0.9949748743718594, "precision": 0.9922048997772829, "recall": 0.9977603583426652, "support": 893}, "macro avg": {"f1-score": 0.8385067266671911, "precision": 0.8413723108712741, "recall": 0.835906454864917, "support": 8267}, "micro avg": {"f1-score": 0.9897181565259465, "precision": 0.9897181565259465, "recall": 0.9897181565259465, "support": 8267}, "weighted avg": {"f1-score": 0.9892161574219317, "precision": 0.9887602465019757, "recall": 0.9897181565259465, "support": 8267}, "\u2205": {"f1-score": 0.9973200992555832, "precision": 0.9970232188926375, "recall": 0.9976171564733916, "support": 5036}, "\u23ce": {"f1-score": 0.8981288981288981, "precision": 0.9037656903765691, "recall": 0.8925619834710744, "support": 242}, "\u23ce\u23ce": {"f1-score": 0.8503937007874016, "precision": 0.8780487804878049, "recall": 0.8244274809160306, "support": 131}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9938080495356036, "precision": 1.0, "recall": 0.9876923076923076, "support": 325}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9827089337175793, "precision": 0.9715099715099715, "recall": 0.9941690962099126, "support": 343}, "\u2423": {"f1-score": 0.9907192575406032, "precision": 0.9884259259259259, "recall": 0.9930232558139535, "support": 1290}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.9911012235817576, "precision": 0.9922048997772829, "recall": 0.99, "support": 900}, "macro avg": {"f1-score": 0.7722342465369684, "precision": 0.8413723108712741, "recall": 0.7202883468716055, "support": 9070}, "micro avg": {"f1-score": 0.9438772567341523, "precision": 0.9897181565259465, "recall": 0.9020948180815876, "support": 9070}, "weighted avg": {"f1-score": 0.9377149124122329, "precision": 0.9872448291213146, "recall": 0.9020948180815876, "support": 9070}, "\u2205": {"f1-score": 0.9960348929421096, "precision": 0.9970232188926375, "recall": 0.9950485244602891, "support": 5049}, "\u23ce": {"f1-score": 0.7578947368421053, "precision": 0.9037656903765691, "recall": 0.6525679758308157, "support": 331}, "\u23ce\u23ce": {"f1-score": 0.7605633802816902, "precision": 0.8780487804878049, "recall": 0.6708074534161491, "support": 161}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9158345221112697, "precision": 1.0, "recall": 0.8447368421052631, "support": 380}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9472222222222223, "precision": 0.9715099715099715, "recall": 0.924119241192412, "support": 369}, "\u2423": {"f1-score": 0.8092229943145924, "precision": 0.9884259259259259, "recall": 0.6850267379679145, "support": 1870}},
  "ppcr": 0.9114663726571114
}
```
</details>