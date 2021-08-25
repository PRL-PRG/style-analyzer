# Train report for javascript / file:///tmp/top-repos-quality-repos-usr5ditx/pixelchain.git HEAD 175f3c9ac7a8c450c677425ac4912b2fe42ebca1

### Classification report

PPCR: 0.184

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.367| 1.000| 0.537| 915| 2493| 0.367 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2209| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 276| 0.000 |
| `macro avg` | 0.333| 0.333| 0.122| 0.333| 0.179| 915| 4978| 0.184 |
| `micro avg` | 1.000| 1.000| 0.184| 1.000| 0.311| 915| 4978| 0.184 |
| `weighted avg` | 1.000| 1.000| 0.184| 1.000| 0.269| 915| 4978| 0.184 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|1578 |915 |0 |0 |
|2209 |0 |0 |0 |
|276 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 915}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 915}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 915}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 915}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"macro avg": {"f1-score": 0.1789906103286385, "precision": 0.3333333333333333, "recall": 0.12234255916566385, "support": 4978}, "micro avg": {"f1-score": 0.3105379263533006, "precision": 1.0, "recall": 0.1838087585375653, "support": 4978}, "weighted avg": {"f1-score": 0.26891739145196614, "precision": 0.5008035355564484, "recall": 0.1838087585375653, "support": 4978}, "\u2205": {"f1-score": 0.5369718309859155, "precision": 1.0, "recall": 0.36702767749699156, "support": 2493}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 276}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2209}},
  "ppcr": 0.1838087585375653
}
```
</details>
