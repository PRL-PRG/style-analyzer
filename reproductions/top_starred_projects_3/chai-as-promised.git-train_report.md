# Train report for javascript / file:///tmp/top-repos-quality-repos-_43lu_bh/chai-as-promised.git HEAD 39bfd34988132e443d703c8547fef4e2fbfa7a21

### Classification report

PPCR: 0.125

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 0.822| 1.000| 0.902| 143| 174| 0.822 |
| `␣` | 1.000| 0.989| 0.126| 0.995| 0.224| 95| 745| 0.128 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.986| 1.000| 0.859| 0.993| 0.918| 73| 85| 0.859 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1195| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 95| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 118| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 72| 0.000 |
| `weighted avg` | 0.997| 0.997| 0.125| 0.997| 0.162| 311| 2484| 0.125 |
| `micro avg` | 0.997| 0.997| 0.125| 0.997| 0.222| 311| 2484| 0.125 |
| `macro avg` | 0.427| 0.427| 0.258| 0.427| 0.292| 311| 2484| 0.125 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1195 |0 |0 |0 |0 |0 |0 |0 |
|650 |0 |94 |0 |0 |1 |0 |0 |
|31 |0 |0 |143 |0 |0 |0 |0 |
|95 |0 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |73 |0 |0 |
|118 |0 |0 |0 |0 |0 |0 |0 |
|72 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/chai-as-promised.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 143}, "macro avg": {"f1-score": 0.4268437533743656, "precision": 0.4266409266409266, "recall": 0.4270676691729323, "support": 311}, "micro avg": {"f1-score": 0.9967845659163987, "precision": 0.9967845659163987, "recall": 0.9967845659163987, "support": 311}, "weighted avg": {"f1-score": 0.9967869963276486, "precision": 0.9968280177283394, "recall": 0.9967845659163987, "support": 311}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9931972789115647, "precision": 0.9864864864864865, "recall": 1.0, "support": 73}, "\u2423": {"f1-score": 0.9947089947089947, "precision": 1.0, "recall": 0.9894736842105263, "support": 95}},
  "cl_report_full": {"\"": {"f1-score": 0.9022082018927445, "precision": 1.0, "recall": 0.8218390804597702, "support": 174}, "macro avg": {"f1-score": 0.29207478241295476, "precision": 0.4266409266409266, "recall": 0.2581195866451186, "support": 2484}, "micro avg": {"f1-score": 0.22182468694096605, "precision": 0.9967845659163987, "recall": 0.1247987117552335, "support": 2484}, "weighted avg": {"f1-score": 0.1618242234918459, "precision": 0.4037243765504635, "recall": 0.1247987117552335, "support": 2484}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1195}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9182389937106918, "precision": 0.9864864864864865, "recall": 0.8588235294117647, "support": 85}, "\u2423": {"f1-score": 0.22407628128724674, "precision": 1.0, "recall": 0.1261744966442953, "support": 745}},
  "ppcr": 0.1252012882447665
}
```
</details>
