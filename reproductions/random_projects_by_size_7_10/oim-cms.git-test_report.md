# Test report for javascript / file:///tmp/top-repos-quality-repos-uciyj34o/oim-cms.git HEAD 7918ec7ed93a536669d7c17312aacbeec0cdfbe3

### Classification report

PPCR: 0.670

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.884| 0.951| 0.808| 0.916| 0.844| 527| 620| 0.850 |
| `␣` | 0.733| 0.597| 0.357| 0.658| 0.481| 124| 207| 0.599 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 94| 0.106 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 18| 0.333 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 43| 0.023 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 15| 0.000 |
| `weighted avg` | 0.833| 0.861| 0.577| 0.845| 0.625| 668| 997| 0.670 |
| `micro avg` | 0.861| 0.861| 0.577| 0.861| 0.691| 668| 997| 0.670 |
| `macro avg` | 0.269| 0.258| 0.194| 0.262| 0.221| 668| 997| 0.670 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|93 |501 |26 |0 |0 |0 |0 |
|83 |50 |74 |0 |0 |0 |0 |
|12 |6 |0 |0 |0 |0 |0 |
|84 |10 |0 |0 |0 |0 |0 |
|42 |0 |1 |0 |0 |0 |0 |
|15 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "macro avg": {"f1-score": 0.2622804522987338, "precision": 0.2693785251541027, "recall": 0.2579063883617963, "support": 668}, "micro avg": {"f1-score": 0.8607784431137725, "precision": 0.8607784431137725, "recall": 0.8607784431137725, "support": 668}, "weighted avg": {"f1-score": 0.8446801582696885, "precision": 0.8330951643781429, "recall": 0.8607784431137725, "support": 668}, "\u2205": {"f1-score": 0.9159049360146252, "precision": 0.8835978835978836, "recall": 0.9506641366223909, "support": 527}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.6577777777777778, "precision": 0.7326732673267327, "recall": 0.5967741935483871, "support": 124}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "macro avg": {"f1-score": 0.22077739727276371, "precision": 0.2693785251541027, "recall": 0.1942587398057244, "support": 997}, "micro avg": {"f1-score": 0.6906906906906907, "precision": 0.8607784431137725, "recall": 0.5767301905717152, "support": 997}, "weighted avg": {"f1-score": 0.6247115069208984, "precision": 0.7015988507194799, "recall": 0.5767301905717152, "support": 997}, "\u2205": {"f1-score": 0.8441449031171019, "precision": 0.8835978835978836, "recall": 0.8080645161290323, "support": 620}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.4805194805194805, "precision": 0.7326732673267327, "recall": 0.357487922705314, "support": 207}},
  "ppcr": 0.6700100300902708
}
```
</details>
