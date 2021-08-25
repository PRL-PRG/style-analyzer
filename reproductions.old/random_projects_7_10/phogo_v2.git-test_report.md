# Test report for javascript / file:///tmp/top-repos-quality-repos-2_e7t08_/phogo_v2.git HEAD 513f38b722e565568f7dd8b14c5c8db132e37b3a

### Classification report

PPCR: 0.708

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.991| 0.933| 0.973| 0.944| 633| 672| 0.942 |
| `␣` | 0.957| 0.912| 0.689| 0.934| 0.801| 194| 257| 0.755 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.950| 0.950| 0.927| 0.950| 0.938| 40| 41| 0.976 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 76| 0.105 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 46| 0.087 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 114| 0.018 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 39| 0.000 |
| `macro avg` | 0.409| 0.408| 0.364| 0.408| 0.383| 881| 1245| 0.708 |
| `micro avg` | 0.956| 0.956| 0.676| 0.956| 0.792| 881| 1245| 0.708 |
| `weighted avg` | 0.941| 0.956| 0.676| 0.948| 0.706| 881| 1245| 0.708 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|39 |627 |6 |0 |0 |0 |0 |0 |
|63 |17 |177 |0 |0 |0 |0 |0 |
|112 |0 |2 |0 |0 |0 |0 |0 |
|68 |6 |0 |0 |0 |2 |0 |0 |
|1 |2 |0 |0 |0 |38 |0 |0 |
|39 |0 |0 |0 |0 |0 |0 |0 |
|42 |4 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "macro avg": {"f1-score": 0.4081263010945058, "precision": 0.40893563424051227, "recall": 0.4075560658621195, "support": 881}, "micro avg": {"f1-score": 0.9557321225879682, "precision": 0.9557321225879682, "recall": 0.9557321225879682, "support": 881}, "weighted avg": {"f1-score": 0.9478041132700523, "precision": 0.9405534382559521, "recall": 0.9557321225879682, "support": 881}, "\u2205": {"f1-score": 0.9728471683475564, "precision": 0.9557926829268293, "recall": 0.990521327014218, "support": 633}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9500000000000001, "precision": 0.95, "recall": 0.95, "support": 40}, "\u2423": {"f1-score": 0.9340369393139842, "precision": 0.9567567567567568, "recall": 0.9123711340206185, "support": 194}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "macro avg": {"f1-score": 0.3833505272496532, "precision": 0.40893563424051227, "recall": 0.3640829908408271, "support": 1245}, "micro avg": {"f1-score": 0.7920978363123237, "precision": 0.9557321225879682, "recall": 0.6763052208835342, "support": 1245}, "weighted avg": {"f1-score": 0.705908379000369, "precision": 0.7446820637857958, "recall": 0.6763052208835342, "support": 1245}, "\u2205": {"f1-score": 0.944277108433735, "precision": 0.9557926829268293, "recall": 0.9330357142857143, "support": 672}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9382716049382716, "precision": 0.95, "recall": 0.926829268292683, "support": 41}, "\u2423": {"f1-score": 0.8009049773755657, "precision": 0.9567567567567568, "recall": 0.688715953307393, "support": 257}},
  "ppcr": 0.7076305220883534
}
```
</details>
