# Train report for javascript / file:///tmp/top-repos-quality-repos-iecphu_t/alaio-oracle.git HEAD dc4ba8da61304656df1eded68c814b3f8fe3beed

### Classification report

PPCR: 0.212

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.996| 0.334| 0.991| 0.499| 1076| 3207| 0.336 |
| `␣` | 0.979| 0.944| 0.115| 0.961| 0.206| 195| 1600| 0.122 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 529| 0.006 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 148| 0.014 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 210| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 169| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 151| 0.000 |
| `macro avg` | 0.281| 0.277| 0.064| 0.279| 0.101| 1276| 6014| 0.212 |
| `micro avg` | 0.984| 0.984| 0.209| 0.984| 0.345| 1276| 6014| 0.212 |
| `weighted avg` | 0.980| 0.984| 0.209| 0.982| 0.321| 1276| 6014| 0.212 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2131 |1072 |4 |0 |0 |0 |0 |0 |
|1405 |11 |184 |0 |0 |0 |0 |0 |
|526 |3 |0 |0 |0 |0 |0 |0 |
|210 |0 |0 |0 |0 |0 |0 |0 |
|169 |0 |0 |0 |0 |0 |0 |0 |
|151 |0 |0 |0 |0 |0 |0 |0 |
|146 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| oracle-app/objectActionHandler.js | 11 |
| oracle-app/index.js | 4 |
| oracle-app/aggregate.js | 3 |
| oracle-app/encoding.js | 1 |
| oracle-app/requestProcessor.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.27879905213727596, "precision": 0.2805739317003397, "recall": 0.27712461021011203, "support": 1276}, "micro avg": {"f1-score": 0.9843260188087775, "precision": 0.9843260188087775, "recall": 0.9843260188087775, "support": 1276}, "weighted avg": {"f1-score": 0.9823028034067781, "precision": 0.9804291022084818, "recall": 0.9843260188087775, "support": 1276}, "\u2205": {"f1-score": 0.9907578558225507, "precision": 0.9852941176470589, "recall": 0.9962825278810409, "support": 1076}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9608355091383811, "precision": 0.9787234042553191, "recall": 0.9435897435897436, "support": 195}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 169}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 210}, "macro avg": {"f1-score": 0.10071452196601623, "precision": 0.2805739317003397, "recall": 0.06418125528976791, "support": 6014}, "micro avg": {"f1-score": 0.34458161865569276, "precision": 0.9843260188087775, "recall": 0.20884602593947457, "support": 6014}, "weighted avg": {"f1-score": 0.32094996675019616, "precision": 0.7857990824912918, "recall": 0.20884602593947457, "support": 6014}, "\u2205": {"f1-score": 0.49918509895227015, "precision": 0.9852941176470589, "recall": 0.3342687870283754, "support": 3207}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 529}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}, "\u2423": {"f1-score": 0.20581655480984337, "precision": 0.9787234042553191, "recall": 0.115, "support": 1600}},
  "ppcr": 0.21217159960093115
}
```
</details>
