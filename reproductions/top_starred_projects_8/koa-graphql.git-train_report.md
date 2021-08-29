# Train report for javascript / file:///tmp/top-repos-quality-repos-0hb9ysqw/koa-graphql.git HEAD 7a4ed0578c4511baf210d44c542e0954c8e655e3

### Classification report

PPCR: 0.063

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 1.000| 0.967| 0.192| 0.983| 0.322| 632| 3179| 0.199 |
| `∅` | 0.779| 1.000| 0.023| 0.876| 0.044| 201| 8809| 0.023 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 735| 0.049 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1069| 0.000 |
| `weighted avg` | 0.907| 0.934| 0.059| 0.918| 0.103| 869| 13792| 0.063 |
| `macro avg` | 0.445| 0.492| 0.054| 0.465| 0.092| 869| 13792| 0.063 |
| `micro avg` | 0.934| 0.934| 0.059| 0.934| 0.111| 869| 13792| 0.063 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|8608 |201 |0 |0 |0 |
|2547 |21 |611 |0 |0 |
|699 |36 |0 |0 |0 |
|1069 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/__tests__/http-test.js | 30 |
| src/index.js | 13 |
| resources/watch.js | 8 |
| src/renderGraphiQL.js | 3 |
| src/__tests__/usage-test.js | 2 |
| resources/register.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4647305959122721, "precision": 0.44476744186046513, "recall": 0.49169303797468356, "support": 869}, "micro avg": {"f1-score": 0.9344073647871116, "precision": 0.9344073647871116, "recall": 0.9344073647871116, "support": 869}, "weighted avg": {"f1-score": 0.9175625112580176, "precision": 0.9074718334359194, "recall": 0.9344073647871116, "support": 869}, "\u2205": {"f1-score": 0.8758169934640523, "precision": 0.7790697674418605, "recall": 1.0, "support": 201}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.9831053901850362, "precision": 1.0, "recall": 0.9667721518987342, "support": 632}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1069}, "macro avg": {"f1-score": 0.09169101147627759, "precision": 0.44476744186046513, "recall": 0.05375409439808032, "support": 13792}, "micro avg": {"f1-score": 0.1107700702544165, "precision": 0.9344073647871116, "recall": 0.05887470997679815, "support": 13792}, "weighted avg": {"f1-score": 0.10263616514014977, "precision": 0.7280906018993147, "recall": 0.05887470997679815, "support": 13792}, "\u2205": {"f1-score": 0.044336605271865, "precision": 0.7790697674418605, "recall": 0.022817572936769212, "support": 8809}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 735}, "\u2423": {"f1-score": 0.32242744063324535, "precision": 1.0, "recall": 0.19219880465555206, "support": 3179}},
  "ppcr": 0.06300754060324826
}
```
</details>
