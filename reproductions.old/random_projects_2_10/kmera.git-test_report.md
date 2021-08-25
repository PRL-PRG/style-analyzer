# Test report for javascript / file:///tmp/top-repos-quality-repos-z1xfkvat/kmera.git HEAD 77ba2c13238342eb1d768d26bcbf135580f49555

### Classification report

PPCR: 0.287

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.946| 0.972| 0.440| 0.959| 0.601| 72| 159| 0.453 |
| `␣` | 0.946| 0.897| 0.438| 0.921| 0.598| 39| 80| 0.487 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 100| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 36| 0.000 |
| `macro avg` | 0.378| 0.374| 0.176| 0.376| 0.240| 111| 387| 0.287 |
| `weighted avg` | 0.946| 0.946| 0.271| 0.946| 0.371| 111| 387| 0.287 |
| `micro avg` | 0.946| 0.946| 0.271| 0.946| 0.422| 111| 387| 0.287 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|87 |70 |2 |0 |0 |0 |
|41 |4 |35 |0 |0 |0 |
|100 |0 |0 |0 |0 |0 |
|36 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.37599134823359776, "precision": 0.3783783783783784, "recall": 0.37393162393162394, "support": 111}, "micro avg": {"f1-score": 0.9459459459459459, "precision": 0.9459459459459459, "recall": 0.9459459459459459, "support": 111}, "weighted avg": {"f1-score": 0.9456049416395488, "precision": 0.9459459459459459, "recall": 0.9459459459459459, "support": 111}, "\u2205": {"f1-score": 0.9589041095890412, "precision": 0.9459459459459459, "recall": 0.9722222222222222, "support": 72}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9210526315789475, "precision": 0.9459459459459459, "recall": 0.8974358974358975, "support": 39}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "macro avg": {"f1-score": 0.23982979347786215, "precision": 0.3783783783783784, "recall": 0.1755503144654088, "support": 387}, "micro avg": {"f1-score": 0.42168674698795183, "precision": 0.9459459459459459, "recall": 0.2713178294573643, "support": 387}, "weighted avg": {"f1-score": 0.3705419342375792, "precision": 0.5841888400027935, "recall": 0.2713178294573643, "support": 387}, "\u2205": {"f1-score": 0.6008583690987125, "precision": 0.9459459459459459, "recall": 0.44025157232704404, "support": 159}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.5982905982905983, "precision": 0.9459459459459459, "recall": 0.4375, "support": 80}},
  "ppcr": 0.2868217054263566
}
```
</details>
