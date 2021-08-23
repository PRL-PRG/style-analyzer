# Train report for javascript / file:///tmp/top-repos-quality-repos-u4xgdhbw/csd2c.git HEAD c54a22757bfff916871c5ac950d9f06356330ee3

### Classification report

PPCR: 0.274

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 1.000| 0.434| 0.996| 0.604| 2590| 5971| 0.434 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 2894| 0.007 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 665| 0.000 |
| `weighted avg` | 0.985| 0.992| 0.272| 0.989| 0.378| 2610| 9530| 0.274 |
| `micro avg` | 0.992| 0.992| 0.272| 0.992| 0.427| 2610| 9530| 0.274 |
| `macro avg` | 0.331| 0.333| 0.145| 0.332| 0.201| 2610| 9530| 0.274 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|3381 |2590 |0 |0 |
|2874 |20 |0 |0 |
|665 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| webapp/Library/addons/p5.dom.js | 14 |
| webapp/chaos-pad/sketch.js | 4 |
| webapp/oscServer.js | 1 |
| webapp/oscLib.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3320512820512821, "precision": 0.33077905491698595, "recall": 0.3333333333333333, "support": 2610}, "micro avg": {"f1-score": 0.9923371647509579, "precision": 0.9923371647509579, "recall": 0.9923371647509579, "support": 2610}, "weighted avg": {"f1-score": 0.9885204833480696, "precision": 0.9847330485459697, "recall": 0.9923371647509579, "support": 2610}, "\u2205": {"f1-score": 0.9961538461538463, "precision": 0.9923371647509579, "recall": 1.0, "support": 2590}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}},
  "cl_report_full": {"macro avg": {"f1-score": 0.20121974905799636, "precision": 0.33077905491698595, "recall": 0.1445877295818679, "support": 9530}, "micro avg": {"f1-score": 0.4266886326194399, "precision": 0.9923371647509579, "recall": 0.27177334732423925, "support": 9530}, "weighted avg": {"f1-score": 0.37822133944133146, "precision": 0.6217466118287481, "recall": 0.27177334732423925, "support": 9530}, "\u2205": {"f1-score": 0.6036592471739891, "precision": 0.9923371647509579, "recall": 0.43376318874560377, "support": 5971}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 665}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2894}},
  "ppcr": 0.2738719832109129
}
```
</details>
