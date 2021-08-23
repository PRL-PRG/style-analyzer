# Train report for javascript / file:///tmp/top-repos-quality-repos-pel7ejne/esfera.git HEAD 163be1282f73bccc57b2df48f51df66cde3e3b24

### Classification report

PPCR: 0.346

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 1.000| 0.506| 0.998| 0.672| 2198| 4341| 0.506 |
| `␣` | 1.000| 0.992| 0.156| 0.996| 0.269| 258| 1644| 0.157 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 426| 0.012 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 249| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 198| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 245| 0.000 |
| `weighted avg` | 0.995| 0.997| 0.345| 0.996| 0.473| 2461| 7103| 0.346 |
| `micro avg` | 0.997| 0.997| 0.345| 0.997| 0.513| 2461| 7103| 0.346 |
| `macro avg` | 0.333| 0.332| 0.110| 0.332| 0.157| 2461| 7103| 0.346 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|2143 |2198 |0 |0 |0 |0 |0 |
|1386 |2 |256 |0 |0 |0 |0 |
|421 |5 |0 |0 |0 |0 |0 |
|249 |0 |0 |0 |0 |0 |0 |
|198 |0 |0 |0 |0 |0 |0 |
|245 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| public/javascripts/main.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.33241985404951757, "precision": 0.3328042328042328, "recall": 0.33204134366925064, "support": 2461}, "micro avg": {"f1-score": 0.9971556277935798, "precision": 0.9971556277935798, "recall": 0.9971556277935798, "support": 2461}, "weighted avg": {"f1-score": 0.9961404605190303, "precision": 0.9951329631134589, "recall": 0.9971556277935798, "support": 2461}, "\u2205": {"f1-score": 0.9984101748807631, "precision": 0.9968253968253968, "recall": 1.0, "support": 2198}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9961089494163424, "precision": 1.0, "recall": 0.9922480620155039, "support": 258}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 249}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 198}, "macro avg": {"f1-score": 0.1568381387321037, "precision": 0.3328042328042328, "recall": 0.11034211790369762, "support": 7103}, "micro avg": {"f1-score": 0.5131744040150564, "precision": 0.9971556277935798, "recall": 0.3454878220470224, "support": 7103}, "weighted avg": {"f1-score": 0.47279116360700885, "precision": 0.8406615581612062, "recall": 0.3454878220470224, "support": 7103}, "\u2205": {"f1-score": 0.6715551481820959, "precision": 0.9968253968253968, "recall": 0.506334945865008, "support": 4341}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 426}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u2423": {"f1-score": 0.2694736842105263, "precision": 1.0, "recall": 0.15571776155717762, "support": 1644}},
  "ppcr": 0.3464733211319161
}
```
</details>
