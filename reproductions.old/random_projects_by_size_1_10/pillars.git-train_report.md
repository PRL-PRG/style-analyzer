# Train report for javascript / file:///tmp/top-repos-quality-repos-pbb3ua0h/pillars.git HEAD 9bc9fbb6b4b42c702da3826ee88eca3564d9ddf1

### Classification report

PPCR: 0.399

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 1.000| 0.647| 0.996| 0.783| 1187| 1834| 0.647 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 992| 0.010 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 175| 0.000 |
| `weighted avg` | 0.983| 0.992| 0.396| 0.987| 0.479| 1197| 3001| 0.399 |
| `macro avg` | 0.331| 0.333| 0.216| 0.332| 0.261| 1197| 3001| 0.399 |
| `micro avg` | 0.992| 0.992| 0.396| 0.992| 0.566| 1197| 3001| 0.399 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|647 |1187 |0 |0 |
|982 |10 |0 |0 |
|175 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/Tiles.js | 4 |
| src/Game.js | 3 |
| src/Boot.js | 1 |
| src/MainMenu.js | 1 |
| src/Preloader.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3319351230425056, "precision": 0.3305485937064884, "recall": 0.3333333333333333, "support": 1197}, "micro avg": {"f1-score": 0.9916457811194653, "precision": 0.9916457811194653, "recall": 0.9916457811194653, "support": 1197}, "weighted avg": {"f1-score": 0.987486193111414, "precision": 0.9833613552120345, "recall": 0.9916457811194653, "support": 1197}, "\u2205": {"f1-score": 0.9958053691275168, "precision": 0.9916457811194653, "recall": 1.0, "support": 1187}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}},
  "cl_report_full": {"macro avg": {"f1-score": 0.26107995161112946, "precision": 0.3305485937064884, "recall": 0.21573973100690658, "support": 3001}, "micro avg": {"f1-score": 0.5655073844687947, "precision": 0.9916457811194653, "recall": 0.3955348217260913, "support": 3001}, "weighted avg": {"f1-score": 0.4786610775622906, "precision": 0.606024112820093, "recall": 0.3955348217260913, "support": 3001}, "\u2205": {"f1-score": 0.7832398548333883, "precision": 0.9916457811194653, "recall": 0.6472191930207197, "support": 1834}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 175}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 992}},
  "ppcr": 0.39886704431856046
}
```
</details>
