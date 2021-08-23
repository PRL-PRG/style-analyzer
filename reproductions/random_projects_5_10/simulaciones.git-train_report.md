# Train report for javascript / file:///tmp/top-repos-quality-repos-7r9y5l73/simulaciones.git HEAD 5e875fc1b6a1ab7f94e7287dff1a3607771a170c

### Classification report

PPCR: 0.409

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 0.994| 0.543| 0.997| 0.704| 3190| 5836| 0.547 |
| `␣` | 0.965| 1.000| 0.288| 0.982| 0.443| 690| 2399| 0.288 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 187| 0.027 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 536| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 378| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 154| 0.000 |
| `weighted avg` | 0.993| 0.994| 0.407| 0.993| 0.545| 3885| 9490| 0.409 |
| `macro avg` | 0.328| 0.332| 0.138| 0.330| 0.191| 3885| 9490| 0.409 |
| `micro avg` | 0.994| 0.994| 0.407| 0.994| 0.577| 3885| 9490| 0.409 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|2646 |3170 |20 |0 |0 |0 |0 |
|1709 |0 |690 |0 |0 |0 |0 |
|536 |0 |0 |0 |0 |0 |0 |
|378 |0 |0 |0 |0 |0 |0 |
|182 |0 |5 |0 |0 |0 |0 |
|154 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Suma_De_Vectores/src/codigo.js | 15 |
| Tiro_Parabolico/src/codigo.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.32984362526765, "precision": 0.3275058275058275, "recall": 0.3322884012539185, "support": 3885}, "micro avg": {"f1-score": 0.9935649935649936, "precision": 0.9935649935649936, "recall": 0.9935649935649936, "support": 3885}, "weighted avg": {"f1-score": 0.9929706495207066, "precision": 0.9925029925029925, "recall": 0.9935649935649936, "support": 3885}, "\u2205": {"f1-score": 0.9968553459119497, "precision": 1.0, "recall": 0.9937304075235109, "support": 3190}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9822064056939501, "precision": 0.965034965034965, "recall": 1.0, "support": 690}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 378}, "macro avg": {"f1-score": 0.19118917510355973, "precision": 0.3275058275058275, "recall": 0.13846668367550527, "support": 9490}, "micro avg": {"f1-score": 0.5771962616822429, "precision": 0.9935649935649936, "recall": 0.4067439409905163, "support": 9490}, "weighted avg": {"f1-score": 0.5449462065669415, "precision": 0.858916636577332, "recall": 0.4067439409905163, "support": 9490}, "\u2205": {"f1-score": 0.7039751276926495, "precision": 1.0, "recall": 0.5431802604523647, "support": 5836}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 536}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u2423": {"f1-score": 0.44315992292870904, "precision": 0.965034965034965, "recall": 0.28761984160066695, "support": 2399}},
  "ppcr": 0.40937829293993677
}
```
</details>
