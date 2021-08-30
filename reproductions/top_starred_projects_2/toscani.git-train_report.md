# Train report for javascript / file:///tmp/top-repos-quality-repos-h5mm5sj6/toscani.git HEAD 98bd486591029911cfe1a0aba946e25bef0fcc44

### Classification report

PPCR: 0.564

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 1.000| 0.943| 0.986| 0.958| 6112| 6478| 0.944 |
| `␣` | 0.998| 0.846| 0.159| 0.916| 0.274| 765| 4082| 0.187 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.939| 0.907| 0.822| 0.923| 0.877| 290| 320| 0.906 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 685| 0.060 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 344| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 346| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 525| 0.000 |
| `micro avg` | 0.974| 0.974| 0.549| 0.974| 0.703| 7208| 12780| 0.564 |
| `macro avg` | 0.416| 0.393| 0.275| 0.404| 0.301| 7208| 12780| 0.564 |
| `weighted avg` | 0.969| 0.974| 0.549| 0.971| 0.595| 7208| 12780| 0.564 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| "| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|366 |6111 |1 |0 |0 |0 |0 |0 |
|3317 |105 |647 |0 |0 |0 |0 |13 |
|644 |37 |0 |0 |0 |0 |0 |4 |
|344 |0 |0 |0 |0 |0 |0 |0 |
|346 |0 |0 |0 |0 |0 |0 |0 |
|525 |0 |0 |0 |0 |0 |0 |0 |
|30 |27 |0 |0 |0 |0 |0 |263 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| paymentInfo/assets/scripts/libs/jquery.inputmask.js | 107 |
| paymentInfo/assets/scripts/app.js | 49 |
| paymentInfo/assets/scripts/libs/jquery.inputmask.date.extensions.js | 31 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4035529305033578, "precision": 0.41583309662624185, "recall": 0.3932120818779459, "support": 7208}, "micro avg": {"f1-score": 0.9740566037735849, "precision": 0.9740566037735849, "recall": 0.9740566037735849, "support": 7208}, "weighted avg": {"f1-score": 0.9706353551407192, "precision": 0.9688864207402305, "recall": 0.9740566037735849, "support": 7208}, "\u2205": {"f1-score": 0.9862814719173659, "precision": 0.9730891719745223, "recall": 0.9998363874345549, "support": 6112}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9228070175438597, "precision": 0.9392857142857143, "recall": 0.906896551724138, "support": 290}, "\u2423": {"f1-score": 0.9157820240622788, "precision": 0.9984567901234568, "recall": 0.8457516339869281, "support": 765}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 525}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 344}, "macro avg": {"f1-score": 0.3011752500966382, "precision": 0.41583309662624185, "recall": 0.27481749241171255, "support": 12780}, "micro avg": {"f1-score": 0.7025215129077447, "precision": 0.9740566037735849, "recall": 0.5493740219092331, "support": 12780}, "weighted avg": {"f1-score": 0.5949216585641202, "precision": 0.8356763460020606, "recall": 0.5493740219092331, "support": 12780}, "\u2205": {"f1-score": 0.957987145320583, "precision": 0.9730891719745223, "recall": 0.9433467119481321, "support": 6478}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 685}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 346}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8766666666666666, "precision": 0.9392857142857143, "recall": 0.821875, "support": 320}, "\u2423": {"f1-score": 0.2735729386892177, "precision": 0.9984567901234568, "recall": 0.15850073493385594, "support": 4082}},
  "ppcr": 0.5640062597809077
}
```
</details>
