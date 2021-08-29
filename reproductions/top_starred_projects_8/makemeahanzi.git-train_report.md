# Train report for javascript / file:///tmp/top-repos-quality-repos-4l3wri28/makemeahanzi.git HEAD 618dbab8a8ddefb958763c8b4afbaa741a4460de

### Classification report

PPCR: 0.429

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.938| 1.000| 0.649| 0.968| 0.768| 930| 1432| 0.649 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 55| 734| 0.075 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 142| 0.042 |
| `weighted avg` | 0.881| 0.938| 0.403| 0.909| 0.476| 991| 2308| 0.429 |
| `macro avg` | 0.313| 0.333| 0.216| 0.323| 0.256| 991| 2308| 0.429 |
| `micro avg` | 0.938| 0.938| 0.403| 0.938| 0.564| 991| 2308| 0.429 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|502 |930 |0 |0 |
|679 |55 |0 |0 |
|136 |6 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| stroke_caps/utils.js | 26 |
| stroke_caps/fixStrokes.js | 24 |
| stroke_caps/updateGraphicsTxt.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.32274856845393024, "precision": 0.31281533804238143, "recall": 0.3333333333333333, "support": 991}, "micro avg": {"f1-score": 0.9384460141271443, "precision": 0.9384460141271443, "recall": 0.9384460141271443, "support": 991}, "weighted avg": {"f1-score": 0.9086463228924979, "precision": 0.8806809214311244, "recall": 0.9384460141271443, "support": 991}, "\u2205": {"f1-score": 0.9682457053617908, "precision": 0.9384460141271443, "recall": 1.0, "support": 930}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}},
  "cl_report_full": {"macro avg": {"f1-score": 0.25588113908378046, "precision": 0.31281533804238143, "recall": 0.2164804469273743, "support": 2308}, "micro avg": {"f1-score": 0.5638072143073658, "precision": 0.9384460141271443, "recall": 0.40294627383015597, "support": 2308}, "weighted avg": {"f1-score": 0.4762848238751823, "precision": 0.5822593987132022, "recall": 0.40294627383015597, "support": 2308}, "\u2205": {"f1-score": 0.7676434172513413, "precision": 0.9384460141271443, "recall": 0.6494413407821229, "support": 1432}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 734}},
  "ppcr": 0.42937608318890813
}
```
</details>
