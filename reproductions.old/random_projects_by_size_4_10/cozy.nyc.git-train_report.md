# Train report for javascript / file:///tmp/top-repos-quality-repos-lk1woi3x/cozy.nyc.git HEAD 4cfa03fd4762d59a89e10b244e67b1359da3e7a9

### Classification report

PPCR: 0.393

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 1.000| 0.573| 0.990| 0.723| 4574| 7983| 0.573 |
| `'` | 1.000| 1.000| 0.819| 1.000| 0.900| 1120| 1368| 0.819 |
| `␣` | 1.000| 0.859| 0.109| 0.924| 0.197| 566| 4455| 0.127 |
| `"` | 1.000| 1.000| 0.977| 1.000| 0.988| 299| 306| 0.977 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 640| 0.009 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 620| 0.003 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1032| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 299| 0.000 |
| `macro avg` | 0.498| 0.482| 0.310| 0.489| 0.351| 6567| 16703| 0.393 |
| `weighted avg` | 0.986| 0.987| 0.388| 0.986| 0.490| 6567| 16703| 0.393 |
| `micro avg` | 0.987| 0.987| 0.388| 0.987| 0.557| 6567| 16703| 0.393 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3409 |4574 |0 |0 |0 |0 |0 |0 |0 |
|3889 |80 |486 |0 |0 |0 |0 |0 |0 |
|248 |0 |0 |1120 |0 |0 |0 |0 |0 |
|1032 |0 |0 |0 |0 |0 |0 |0 |0 |
|634 |6 |0 |0 |0 |0 |0 |0 |0 |
|618 |2 |0 |0 |0 |0 |0 |0 |0 |
|7 |0 |0 |0 |0 |0 |0 |299 |0 |
|299 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/redux/create.js | 11 |
| src/client.js | 9 |
| src/components/NavBar/NavBar.js | 6 |
| src/components/MessageItem/MessageEdition.js | 6 |
| src/components/MessageItem/MessageItem.js | 6 |
| src/redux/modules/notifs.js | 5 |
| src/helpers/apiClient.js | 5 |
| src/app.js | 4 |
| src/components/LoginForm/LoginForm.js | 3 |
| src/components/MessageItem/Message.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 299}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1120}, "macro avg": {"f1-score": 0.4893033048066172, "precision": 0.49764049764049767, "recall": 0.4823321554770318, "support": 6567}, "micro avg": {"f1-score": 0.9865996649916248, "precision": 0.9865996649916248, "recall": 0.9865996649916248, "support": 6567}, "weighted avg": {"f1-score": 0.9855911990267171, "precision": 0.9856343976946993, "recall": 0.9865996649916248, "support": 6567}, "\u2205": {"f1-score": 0.9904720658293633, "precision": 0.9811239811239811, "recall": 1.0, "support": 4574}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9239543726235742, "precision": 1.0, "recall": 0.8586572438162544, "support": 566}},
  "cl_report_full": {"\"": {"f1-score": 0.9884297520661156, "precision": 1.0, "recall": 0.9771241830065359, "support": 306}, "\u0027": {"f1-score": 0.9003215434083601, "precision": 1.0, "recall": 0.8187134502923976, "support": 1368}, "macro avg": {"f1-score": 0.3511150762641489, "precision": 0.49764049764049767, "recall": 0.30973701230580786, "support": 16703}, "micro avg": {"f1-score": 0.5568543188654921, "precision": 0.9865996649916248, "recall": 0.3878943902293001, "support": 16703}, "weighted avg": {"f1-score": 0.49007832289950837, "precision": 0.8358565970970928, "recall": 0.3878943902293001, "support": 16703}, "\u2205": {"f1-score": 0.7234480031633057, "precision": 0.9811239811239811, "recall": 0.5729675560566203, "support": 7983}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1032}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 299}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 640}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 620}, "\u2423": {"f1-score": 0.19672131147540983, "precision": 1.0, "recall": 0.10909090909090909, "support": 4455}},
  "ppcr": 0.3931629048673891
}
```
</details>
