# Train report for javascript / file:///tmp/top-repos-quality-repos-q95g38_n/kafkastreaming.git HEAD 4c41a27b6eb0944b862b13e808415445a50adfcd

### Classification report

PPCR: 0.221

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 1.000| 0.365| 0.993| 0.533| 301| 824| 0.365 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 97| 0.031 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 297| 0.003 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 165| 0.000 |
| `macro avg` | 0.247| 0.250| 0.091| 0.248| 0.133| 305| 1383| 0.221 |
| `weighted avg` | 0.974| 0.987| 0.218| 0.980| 0.318| 305| 1383| 0.221 |
| `micro avg` | 0.987| 0.987| 0.218| 0.987| 0.357| 305| 1383| 0.221 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|523 |301 |0 |0 |0 |
|296 |1 |0 |0 |0 |
|165 |0 |0 |0 |0 |
|94 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| node_fanout_message/test/tasks.js | 2 |
| node_fanout_message/controllers/message.js | 1 |
| node_fanout_message/test/errors.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24834983498349833, "precision": 0.24672131147540985, "recall": 0.25, "support": 305}, "micro avg": {"f1-score": 0.9868852459016394, "precision": 0.9868852459016394, "recall": 0.9868852459016394, "support": 305}, "weighted avg": {"f1-score": 0.9803711518692853, "precision": 0.9739424885783392, "recall": 0.9868852459016394, "support": 305}, "\u2205": {"f1-score": 0.9933993399339933, "precision": 0.9868852459016394, "recall": 1.0, "support": 301}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "macro avg": {"f1-score": 0.133303808680248, "precision": 0.24672131147540985, "recall": 0.09132281553398058, "support": 1383}, "micro avg": {"f1-score": 0.3566350710900474, "precision": 0.9868852459016394, "recall": 0.21764280549530007, "support": 1383}, "weighted avg": {"f1-score": 0.3176929525741847, "precision": 0.5879923663217288, "recall": 0.21764280549530007, "support": 1383}, "\u2205": {"f1-score": 0.533215234720992, "precision": 0.9868852459016394, "recall": 0.36529126213592233, "support": 824}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 297}},
  "ppcr": 0.2205350686912509
}
```
</details>
