# Train report for javascript / file:///tmp/top-repos-quality-repos-xfpuz3kv/papercut-smtp.git HEAD 4530139edde1c71d18734150e59c90a18719a33c

### Classification report

PPCR: 0.248

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.474| 1.000| 0.643| 4059| 8558| 0.474 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 998| 0.002 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 5755| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1035| 0.000 |
| `weighted avg` | 0.999| 1.000| 0.248| 0.999| 0.337| 4061| 16346| 0.248 |
| `macro avg` | 0.250| 0.250| 0.119| 0.250| 0.161| 4061| 16346| 0.248 |
| `micro avg` | 1.000| 1.000| 0.248| 1.000| 0.398| 4061| 16346| 0.248 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|4499 |4059 |0 |0 |0 |
|5755 |0 |0 |0 |0 |
|1035 |0 |0 |0 |0 |
|996 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/Papercut.App.WebApi/assets/js/controllers.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24993842364532018, "precision": 0.24987687761635066, "recall": 0.25, "support": 4061}, "micro avg": {"f1-score": 0.9995075104654026, "precision": 0.9995075104654026, "recall": 0.9995075104654026, "support": 4061}, "weighted avg": {"f1-score": 0.9992613263495244, "precision": 0.999015263476747, "recall": 0.9995075104654026, "support": 4061}, "\u2205": {"f1-score": 0.9997536945812807, "precision": 0.9995075104654026, "recall": 1.0, "support": 4059}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1035}, "macro avg": {"f1-score": 0.16082890878833506, "precision": 0.24987687761635066, "recall": 0.11857326478149101, "support": 16346}, "micro avg": {"f1-score": 0.3978046748664674, "precision": 0.9995075104654026, "recall": 0.24831763122476447, "support": 16346}, "weighted avg": {"f1-score": 0.3368099354975092, "precision": 0.5232953183997868, "recall": 0.24831763122476447, "support": 16346}, "\u2205": {"f1-score": 0.6433156351533402, "precision": 0.9995075104654026, "recall": 0.47429305912596403, "support": 8558}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 998}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5755}},
  "ppcr": 0.24843998531750888
}
```
</details>
