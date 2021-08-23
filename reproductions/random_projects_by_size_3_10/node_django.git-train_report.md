# Train report for javascript / file:///tmp/top-repos-quality-repos-k10zevp7/node_django.git HEAD 8fb39b22445a0362d48a88a10a8c38aa67ecadc2

### Classification report

PPCR: 0.178

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 1.000| 0.309| 0.998| 0.472| 311| 1005| 0.309 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 546| 0.002 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 204| 0.000 |
| `weighted avg` | 0.994| 0.997| 0.177| 0.995| 0.270| 312| 1755| 0.178 |
| `micro avg` | 0.997| 0.997| 0.177| 0.997| 0.301| 312| 1755| 0.178 |
| `macro avg` | 0.332| 0.333| 0.103| 0.333| 0.157| 312| 1755| 0.178 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|694 |311 |0 |0 |
|545 |1 |0 |0 |
|204 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| backend/static/js/js.cookie.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.33279828785446763, "precision": 0.33226495726495725, "recall": 0.3333333333333333, "support": 312}, "micro avg": {"f1-score": 0.9967948717948718, "precision": 0.9967948717948718, "recall": 0.9967948717948718, "support": 312}, "weighted avg": {"f1-score": 0.9951948800263407, "precision": 0.9936000164365549, "recall": 0.9967948717948718, "support": 312}, "\u2205": {"f1-score": 0.9983948635634029, "precision": 0.9967948717948718, "recall": 1.0, "support": 311}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 204}, "macro avg": {"f1-score": 0.15742849911414833, "precision": 0.33226495726495725, "recall": 0.10315091210613599, "support": 1755}, "micro avg": {"f1-score": 0.3009192065795839, "precision": 0.9967948717948718, "recall": 0.1772079772079772, "support": 1755}, "weighted avg": {"f1-score": 0.270454088221742, "precision": 0.5708141573526189, "recall": 0.1772079772079772, "support": 1755}, "\u2205": {"f1-score": 0.472285497342445, "precision": 0.9967948717948718, "recall": 0.309452736318408, "support": 1005}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 546}},
  "ppcr": 0.17777777777777778
}
```
</details>
